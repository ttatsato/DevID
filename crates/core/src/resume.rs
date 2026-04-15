use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Employment {
    pub id: Uuid,
    pub company_name: String,
    pub company_anonymized: bool,
    pub employment_type: EmploymentType,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub summary: Option<String>,
    pub projects: Vec<Project>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EmploymentType {
    FullTime,
    Contract,
    Ses,
    Freelance,
    Internship,
    SideJob,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub client_name: Option<String>,
    pub client_industry: Option<Industry>,
    pub client_anonymized: bool,
    pub role: String,
    pub team_size: Option<u32>,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub summary: String,
    pub achievements: Vec<String>,
    pub skills: Vec<ProjectSkill>,
    pub responsibilities: Vec<Responsibility>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Industry {
    Finance,
    Manufacturing,
    Ec,
    GameDev,
    Telecom,
    PublicSector,
    Healthcare,
    Education,
    Logistics,
    Media,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSkill {
    pub skill_id: String,
    pub usage: SkillUsage,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SkillUsage {
    Primary,
    Secondary,
    Touched,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Responsibility {
    Requirements,
    Design,
    Implementation,
    Review,
    Testing,
    Operation,
    ProjectManagement,
    TeamLead,
    Mentoring,
}
