import type { Employment, SkillExperience } from "./types";

export async function createPortfolio(
  employments: Employment[]
): Promise<{ id: string }> {
  const r = await fetch("/api/portfolios", {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({ employments }),
  });
  if (!r.ok) throw new Error(`createPortfolio failed: ${r.status} ${await r.text()}`);
  return r.json();
}

export async function fetchSkillExperience(
  id: string
): Promise<SkillExperience[]> {
  const r = await fetch(`/api/portfolios/${id}/skill-experience`);
  if (!r.ok) throw new Error(`fetchSkillExperience failed: ${r.status}`);
  return r.json();
}
