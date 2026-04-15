export type User = {
  id: string;
  github_id: number;
  username: string;
  name: string | null;
  avatar_url: string | null;
  email: string | null;
};

export type SkillCategory =
  | "language"
  | "framework"
  | "database"
  | "infrastructure"
  | "tool";

export type Skill = {
  id: string;
  name: string;
  category: SkillCategory;
  aliases: string[];
};

export type Certification = {
  id: string;
  name: string;
  issuer: string;
  aliases: string[];
};

export type EmploymentType =
  | "full_time"
  | "contract"
  | "ses"
  | "freelance"
  | "internship"
  | "side_job";

export type Industry =
  | "finance"
  | "manufacturing"
  | "ec"
  | "game_dev"
  | "telecom"
  | "public_sector"
  | "healthcare"
  | "education"
  | "logistics"
  | "media"
  | "other";

export type SkillUsage = "primary" | "secondary" | "touched";

export type Responsibility =
  | "requirements"
  | "design"
  | "implementation"
  | "review"
  | "testing"
  | "operation"
  | "project_management"
  | "team_lead"
  | "mentoring";

export type ProjectSkill = { skill_id: string; usage: SkillUsage };

export type Project = {
  id: string;
  name: string;
  client_name: string | null;
  client_industry: Industry | null;
  client_anonymized: boolean;
  role: string;
  team_size: number | null;
  start_date: string;
  end_date: string | null;
  summary: string;
  achievements: string[];
  skills: ProjectSkill[];
  responsibilities: Responsibility[];
};

export type Employment = {
  id: string;
  company_name: string;
  company_anonymized: boolean;
  employment_type: EmploymentType;
  start_date: string;
  end_date: string | null;
  summary: string | null;
  projects: Project[];
};

export type SkillExperience = {
  skill_id: string;
  total_months: number;
  primary_months: number;
  last_used: string;
  project_count: number;
};

export type PortfolioResponse = {
  id: string;
  employments: Employment[];
};
