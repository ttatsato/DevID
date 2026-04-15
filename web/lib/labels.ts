import type { EmploymentType, Industry, Responsibility, SkillUsage } from "./types";

export const EMPLOYMENT_TYPE_LABELS: Record<EmploymentType, string> = {
  full_time: "正社員",
  contract: "業務委託",
  ses: "SES",
  freelance: "フリーランス",
  internship: "インターン",
  side_job: "副業",
};

export const INDUSTRY_LABELS: Record<Industry, string> = {
  finance: "金融",
  manufacturing: "製造",
  ec: "EC",
  game_dev: "ゲーム",
  telecom: "通信",
  public_sector: "公共",
  healthcare: "医療",
  education: "教育",
  logistics: "物流",
  media: "メディア",
  other: "その他",
};

export const SKILL_USAGE_LABELS: Record<SkillUsage, string> = {
  primary: "Primary",
  secondary: "Secondary",
  touched: "Touched",
};

export const RESPONSIBILITY_LABELS: Record<Responsibility, string> = {
  requirements: "要件定義",
  design: "設計",
  implementation: "実装",
  review: "レビュー",
  testing: "テスト",
  operation: "運用",
  project_management: "PM",
  team_lead: "リード",
  mentoring: "メンタリング",
};
