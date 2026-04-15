import type {
  Employment,
  PortfolioResponse,
  Profile,
  PublicUserResponse,
  SkillExperience,
  User,
} from "./types";

/** SSR (Server Components) から直接APIを叩く。Next.jsのrewriteを介さないので絶対URL必須。 */
export async function fetchPublicUser(
  username: string
): Promise<PublicUserResponse | null> {
  const base = process.env.API_BASE_URL ?? "http://localhost:3001";
  const r = await fetch(
    `${base}/api/users/${encodeURIComponent(username)}`,
    { cache: "no-store" }
  );
  if (r.status === 404) return null;
  if (!r.ok) throw new Error(`fetchPublicUser failed: ${r.status}`);
  return r.json();
}

export async function getMyProfile(): Promise<Profile> {
  const r = await fetch("/api/me/profile");
  if (!r.ok) throw new Error(`getMyProfile failed: ${r.status}`);
  return r.json();
}

export async function updateMyProfile(p: Profile): Promise<Profile> {
  const r = await fetch("/api/me/profile", {
    method: "PUT",
    headers: { "content-type": "application/json" },
    body: JSON.stringify(p),
  });
  if (!r.ok) throw new Error(`updateMyProfile failed: ${r.status}`);
  return r.json();
}

export async function getMe(): Promise<User | null> {
  const r = await fetch("/api/me");
  if (r.status === 401) return null;
  if (!r.ok) throw new Error(`getMe failed: ${r.status}`);
  return r.json();
}

export async function logout(): Promise<void> {
  const r = await fetch("/api/auth/logout", { method: "POST" });
  if (!r.ok) throw new Error(`logout failed: ${r.status}`);
}

export async function fetchMyPortfolio(): Promise<PortfolioResponse | null> {
  const r = await fetch("/api/me/portfolio");
  if (!r.ok) throw new Error(`fetchMyPortfolio failed: ${r.status}`);
  const body: PortfolioResponse | null = await r.json();
  return body;
}


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
