use chrono::{Datelike, NaiveDate};
use serde::Serialize;
use std::collections::BTreeMap;

use crate::resume::{Employment, SkillUsage};

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct SkillExperience {
    pub skill_id: String,
    pub total_months: u32,
    pub primary_months: u32,
    pub last_used: NaiveDate,
    pub project_count: u32,
}

/// Aggregate per-skill experience across all employments / projects.
///
/// Overlapping intervals are merged so concurrent projects don't double-count.
/// Ongoing projects (`end_date == None`) use `today` as their end.
pub fn aggregate_skill_experience(
    employments: &[Employment],
    today: NaiveDate,
) -> Vec<SkillExperience> {
    #[derive(Default)]
    struct Acc {
        intervals: Vec<(NaiveDate, NaiveDate)>,
        primary_intervals: Vec<(NaiveDate, NaiveDate)>,
        last_used: Option<NaiveDate>,
        project_count: u32,
    }

    let mut by_skill: BTreeMap<String, Acc> = BTreeMap::new();

    for emp in employments {
        for proj in &emp.projects {
            let end = proj.end_date.unwrap_or(today);
            if end < proj.start_date {
                continue;
            }
            for ps in &proj.skills {
                let acc = by_skill.entry(ps.skill_id.clone()).or_default();
                acc.intervals.push((proj.start_date, end));
                if ps.usage == SkillUsage::Primary {
                    acc.primary_intervals.push((proj.start_date, end));
                }
                acc.last_used = Some(acc.last_used.map_or(end, |d| d.max(end)));
                acc.project_count += 1;
            }
        }
    }

    let mut out: Vec<SkillExperience> = by_skill
        .into_iter()
        .map(|(skill_id, acc)| SkillExperience {
            skill_id,
            total_months: sum_months(&merge_intervals(acc.intervals)),
            primary_months: sum_months(&merge_intervals(acc.primary_intervals)),
            last_used: acc.last_used.expect("at least one project recorded"),
            project_count: acc.project_count,
        })
        .collect();

    out.sort_by(|a, b| {
        b.total_months
            .cmp(&a.total_months)
            .then_with(|| a.skill_id.cmp(&b.skill_id))
    });
    out
}

fn merge_intervals(mut intervals: Vec<(NaiveDate, NaiveDate)>) -> Vec<(NaiveDate, NaiveDate)> {
    if intervals.is_empty() {
        return Vec::new();
    }
    intervals.sort_by_key(|&(s, _)| s);
    let mut merged: Vec<(NaiveDate, NaiveDate)> = Vec::with_capacity(intervals.len());
    merged.push(intervals[0]);
    for (s, e) in intervals.into_iter().skip(1) {
        let last = merged.last_mut().unwrap();
        if s <= last.1 {
            if e > last.1 {
                last.1 = e;
            }
        } else {
            merged.push((s, e));
        }
    }
    merged
}

fn sum_months(intervals: &[(NaiveDate, NaiveDate)]) -> u32 {
    intervals.iter().map(|&(s, e)| months_inclusive(s, e)).sum()
}

/// Inclusive month count: 2022-04 〜 2022-04 = 1, 2022-04 〜 2023-03 = 12.
fn months_inclusive(start: NaiveDate, end: NaiveDate) -> u32 {
    let s = start.year() * 12 + start.month() as i32;
    let e = end.year() * 12 + end.month() as i32;
    (e - s + 1).max(0) as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resume::{
        Employment, EmploymentType, Project, ProjectSkill, Responsibility, SkillUsage,
    };
    use uuid::Uuid;

    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).unwrap()
    }

    fn skill(id: &str, usage: SkillUsage) -> ProjectSkill {
        ProjectSkill {
            skill_id: id.to_string(),
            usage,
        }
    }

    fn project(start: NaiveDate, end: Option<NaiveDate>, skills: Vec<ProjectSkill>) -> Project {
        Project {
            id: Uuid::new_v4(),
            name: "test".into(),
            client_name: None,
            client_industry: None,
            client_anonymized: false,
            role: "engineer".into(),
            team_size: None,
            start_date: start,
            end_date: end,
            summary: String::new(),
            achievements: vec![],
            skills,
            responsibilities: vec![Responsibility::Implementation],
        }
    }

    fn employment(projects: Vec<Project>) -> Employment {
        Employment {
            id: Uuid::new_v4(),
            company_name: "Co".into(),
            company_anonymized: false,
            employment_type: EmploymentType::FullTime,
            start_date: d(2020, 1, 1),
            end_date: None,
            summary: None,
            projects,
        }
    }

    #[test]
    fn months_inclusive_basic() {
        assert_eq!(months_inclusive(d(2022, 4, 1), d(2022, 4, 30)), 1);
        assert_eq!(months_inclusive(d(2022, 4, 1), d(2023, 3, 31)), 12);
        assert_eq!(months_inclusive(d(2022, 4, 1), d(2024, 3, 31)), 24);
    }

    #[test]
    fn single_project() {
        let emp = employment(vec![project(
            d(2022, 4, 1),
            Some(d(2023, 3, 31)),
            vec![skill("rust", SkillUsage::Primary)],
        )]);
        let r = aggregate_skill_experience(&[emp], d(2024, 1, 1));
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].skill_id, "rust");
        assert_eq!(r[0].total_months, 12);
        assert_eq!(r[0].primary_months, 12);
        assert_eq!(r[0].project_count, 1);
        assert_eq!(r[0].last_used, d(2023, 3, 31));
    }

    #[test]
    fn overlapping_projects_are_merged() {
        let emp = employment(vec![
            project(
                d(2022, 4, 1),
                Some(d(2023, 3, 31)),
                vec![skill("rust", SkillUsage::Primary)],
            ),
            project(
                d(2022, 9, 1),
                Some(d(2023, 6, 30)),
                vec![skill("rust", SkillUsage::Primary)],
            ),
        ]);
        let r = aggregate_skill_experience(&[emp], d(2024, 1, 1));
        // 2022-04 〜 2023-06 = 15ヶ月 (NOT 12+10=22)
        assert_eq!(r[0].total_months, 15);
        assert_eq!(r[0].project_count, 2);
    }

    #[test]
    fn gap_between_projects_not_counted() {
        let emp = employment(vec![
            project(
                d(2022, 4, 1),
                Some(d(2022, 9, 30)),
                vec![skill("rust", SkillUsage::Primary)],
            ),
            project(
                d(2023, 4, 1),
                Some(d(2023, 9, 30)),
                vec![skill("rust", SkillUsage::Primary)],
            ),
        ]);
        let r = aggregate_skill_experience(&[emp], d(2024, 1, 1));
        // 6 + 6 = 12 (gap doesn't count)
        assert_eq!(r[0].total_months, 12);
    }

    #[test]
    fn ongoing_project_uses_today() {
        let emp = employment(vec![project(
            d(2024, 1, 1),
            None,
            vec![skill("rust", SkillUsage::Primary)],
        )]);
        let r = aggregate_skill_experience(&[emp], d(2024, 12, 31));
        assert_eq!(r[0].total_months, 12);
        assert_eq!(r[0].last_used, d(2024, 12, 31));
    }

    #[test]
    fn primary_vs_secondary_separated() {
        let emp = employment(vec![
            project(
                d(2022, 1, 1),
                Some(d(2022, 12, 31)),
                vec![skill("rust", SkillUsage::Primary)],
            ),
            project(
                d(2023, 1, 1),
                Some(d(2023, 6, 30)),
                vec![skill("rust", SkillUsage::Secondary)],
            ),
        ]);
        let r = aggregate_skill_experience(&[emp], d(2024, 1, 1));
        assert_eq!(r[0].total_months, 18);
        assert_eq!(r[0].primary_months, 12);
    }

    #[test]
    fn multiple_skills_sorted_by_total_months() {
        let emp = employment(vec![
            project(
                d(2022, 1, 1),
                Some(d(2022, 6, 30)),
                vec![
                    skill("rust", SkillUsage::Primary),
                    skill("postgres", SkillUsage::Secondary),
                ],
            ),
            project(
                d(2023, 1, 1),
                Some(d(2024, 12, 31)),
                vec![skill("rust", SkillUsage::Primary)],
            ),
        ]);
        let r = aggregate_skill_experience(&[emp], d(2025, 1, 1));
        assert_eq!(r.len(), 2);
        assert_eq!(r[0].skill_id, "rust"); // 6 + 24 = 30
        assert_eq!(r[0].total_months, 30);
        assert_eq!(r[1].skill_id, "postgres"); // 6
        assert_eq!(r[1].total_months, 6);
    }

    #[test]
    fn cross_employment_aggregation() {
        let emp1 = employment(vec![project(
            d(2020, 4, 1),
            Some(d(2021, 3, 31)),
            vec![skill("rust", SkillUsage::Primary)],
        )]);
        let emp2 = employment(vec![project(
            d(2022, 4, 1),
            Some(d(2023, 3, 31)),
            vec![skill("rust", SkillUsage::Primary)],
        )]);
        let r = aggregate_skill_experience(&[emp1, emp2], d(2024, 1, 1));
        assert_eq!(r[0].total_months, 24);
        assert_eq!(r[0].project_count, 2);
    }
}
