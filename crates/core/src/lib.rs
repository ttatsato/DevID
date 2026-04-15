use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SkillCategory {
    Language,
    Framework,
    Database,
    Infrastructure,
    Tool,
}

#[derive(Debug, Clone, Serialize)]
pub struct Skill {
    pub id: &'static str,
    pub name: &'static str,
    pub category: SkillCategory,
    pub aliases: &'static [&'static str],
}

#[derive(Debug, Clone, Serialize)]
pub struct Certification {
    pub id: &'static str,
    pub name: &'static str,
    pub issuer: &'static str,
    pub aliases: &'static [&'static str],
}
