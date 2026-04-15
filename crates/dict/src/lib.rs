use yokogushi_core::{Certification, Skill};

mod certs;
mod skills;

pub use certs::CERTIFICATIONS;
pub use skills::SKILLS;

pub fn suggest_skills(query: &str, limit: usize) -> Vec<&'static Skill> {
    suggest(SKILLS, query, limit, |s| {
        std::iter::once(s.name).chain(s.aliases.iter().copied())
    })
}

pub fn suggest_certifications(query: &str, limit: usize) -> Vec<&'static Certification> {
    suggest(CERTIFICATIONS, query, limit, |c| {
        std::iter::once(c.name).chain(c.aliases.iter().copied())
    })
}

fn suggest<'a, T, F, I>(items: &'a [T], query: &str, limit: usize, names: F) -> Vec<&'a T>
where
    F: Fn(&'a T) -> I,
    I: Iterator<Item = &'a str>,
{
    if query.is_empty() {
        return Vec::new();
    }
    let q = query.to_lowercase();
    let mut scored: Vec<(u8, &T)> = items
        .iter()
        .filter_map(|item| {
            let mut best: Option<u8> = None;
            for name in names(item) {
                let n = name.to_lowercase();
                let score = if n == q {
                    0
                } else if n.starts_with(&q) {
                    1
                } else if n.contains(&q) {
                    2
                } else {
                    continue;
                };
                best = Some(best.map_or(score, |b| b.min(score)));
            }
            best.map(|s| (s, item))
        })
        .collect();
    scored.sort_by_key(|(s, _)| *s);
    scored.into_iter().take(limit).map(|(_, i)| i).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefix_match_ranks_first() {
        let r = suggest_skills("ja", 5);
        assert!(r.iter().any(|s| s.name == "Java"));
        assert!(r.iter().any(|s| s.name == "JavaScript"));
    }

    #[test]
    fn alias_match_works() {
        let r = suggest_skills("k8s", 5);
        assert!(r.iter().any(|s| s.name == "Kubernetes"));
    }

    #[test]
    fn cert_japanese_match() {
        let r = suggest_certifications("基本", 5);
        assert!(r.iter().any(|c| c.name.contains("基本情報")));
    }
}
