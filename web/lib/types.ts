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
