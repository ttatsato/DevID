import { EMPLOYMENT_TYPE_LABELS, INDUSTRY_LABELS, SKILL_USAGE_LABELS } from "../lib/labels";
import type { Employment, Project, PublicUserResponse } from "../lib/types";

export function PublicPortfolioView({ data }: { data: PublicUserResponse }) {
  const { user, profile, portfolio } = data;
  const displayName = profile.display_name ?? user.name ?? user.username;

  return (
    <main>
      <header style={headerStyle}>
        {profile.avatar_url || user.avatar_url ? (
          <img
            src={profile.avatar_url ?? user.avatar_url ?? ""}
            alt=""
            width={72}
            height={72}
            style={{ borderRadius: "50%" }}
          />
        ) : null}
        <div style={{ flex: 1 }}>
          <h1 style={{ margin: 0, fontSize: 22 }}>{displayName}</h1>
          <div className="muted" style={{ fontSize: 13 }}>@{user.username}</div>
          {profile.headline && (
            <div style={{ fontSize: 14, marginTop: 4 }}>{profile.headline}</div>
          )}
          {profile.location && (
            <div className="muted" style={{ fontSize: 12, marginTop: 2 }}>
              📍 {profile.location}
            </div>
          )}
        </div>
      </header>

      {profile.bio && (
        <section style={sectionStyle}>
          <p style={{ whiteSpace: "pre-wrap", margin: 0 }}>{profile.bio}</p>
        </section>
      )}

      {(profile.social_links.length > 0 || profile.contact_email) && (
        <section style={sectionStyle}>
          <h2 style={h2Style}>リンク</h2>
          <ul style={{ listStyle: "none", padding: 0, margin: 0, display: "flex", flexWrap: "wrap", gap: 8 }}>
            {profile.social_links.map((l, i) => (
              <li key={i}>
                <a href={l.url} target="_blank" rel="noopener noreferrer" style={linkChip}>
                  {l.platform}
                </a>
              </li>
            ))}
            {profile.contact_email && (
              <li>
                <a href={`mailto:${profile.contact_email}`} style={linkChip}>
                  ✉ {profile.contact_email}
                </a>
              </li>
            )}
          </ul>
        </section>
      )}

      {portfolio && portfolio.employments.length > 0 && (
        <section style={sectionStyle}>
          <h2 style={h2Style}>職務経歴</h2>
          {portfolio.employments.map((emp) => (
            <EmploymentView key={emp.id} emp={emp} />
          ))}
        </section>
      )}
    </main>
  );
}

function EmploymentView({ emp }: { emp: Employment }) {
  return (
    <article style={employmentStyle}>
      <div style={{ display: "flex", justifyContent: "space-between", alignItems: "baseline" }}>
        <div style={{ fontWeight: 600 }}>{emp.company_name}</div>
        <div className="muted" style={{ fontSize: 12 }}>
          {EMPLOYMENT_TYPE_LABELS[emp.employment_type]} · {emp.start_date} 〜 {emp.end_date ?? "現在"}
        </div>
      </div>
      {emp.projects.map((p) => (
        <ProjectView key={p.id} project={p} />
      ))}
    </article>
  );
}

function ProjectView({ project: p }: { project: Project }) {
  return (
    <div style={projectStyle}>
      <div style={{ fontWeight: 500, fontSize: 14 }}>{p.name}</div>
      <div className="muted" style={{ fontSize: 12 }}>
        {p.client_name ?? "(非公開クライアント)"}
        {p.client_industry ? ` · ${INDUSTRY_LABELS[p.client_industry]}` : ""}
        {" · "}
        {p.start_date} 〜 {p.end_date ?? "現在"}
        {" · "}
        {p.role}
      </div>
      {p.summary && <p style={{ fontSize: 13, margin: "6px 0" }}>{p.summary}</p>}
      {p.skills.length > 0 && (
        <div style={{ display: "flex", flexWrap: "wrap", gap: 6, marginTop: 4 }}>
          {p.skills.map((s) => (
            <span key={s.skill_id} style={skillChip(s.usage)}>
              {s.skill_id} · {SKILL_USAGE_LABELS[s.usage]}
            </span>
          ))}
        </div>
      )}
    </div>
  );
}

const headerStyle: React.CSSProperties = {
  display: "flex",
  gap: 16,
  alignItems: "center",
  padding: "24px 0",
  borderBottom: "1px solid #eee",
};
const sectionStyle: React.CSSProperties = {
  marginTop: 24,
};
const h2Style: React.CSSProperties = {
  fontSize: 16,
  margin: "0 0 12px",
  color: "#333",
};
const employmentStyle: React.CSSProperties = {
  padding: 14,
  border: "1px solid #e5e5e5",
  borderRadius: 8,
  marginBottom: 12,
  background: "#fff",
};
const projectStyle: React.CSSProperties = {
  marginTop: 10,
  paddingTop: 10,
  borderTop: "1px dashed #eee",
};
const linkChip: React.CSSProperties = {
  display: "inline-block",
  padding: "4px 10px",
  background: "#f0f4ff",
  color: "#3730a3",
  borderRadius: 999,
  fontSize: 12,
  textDecoration: "none",
};
const skillChip = (usage: string): React.CSSProperties => {
  const colors: Record<string, [string, string]> = {
    primary: ["#eef2ff", "#3730a3"],
    secondary: ["#f5f5f5", "#555"],
    touched: ["#fafafa", "#888"],
  };
  const [bg, fg] = colors[usage] ?? colors.secondary;
  return {
    padding: "2px 8px",
    background: bg,
    color: fg,
    borderRadius: 4,
    fontSize: 11,
  };
};
