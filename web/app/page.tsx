"use client";

import { useEffect, useState } from "react";
import { EmploymentForm } from "../components/EmploymentForm";
import { SkillExperienceTable } from "../components/SkillExperienceTable";
import {
  createPortfolio,
  fetchMyPortfolio,
  fetchSkillExperience,
  getMe,
  logout,
} from "../lib/api";
import type { Employment, SkillExperience, User } from "../lib/types";

const newEmployment = (): Employment => ({
  id: crypto.randomUUID(),
  company_name: "",
  company_anonymized: false,
  employment_type: "full_time",
  start_date: "",
  end_date: null,
  summary: null,
  projects: [],
});

export default function Page() {
  const [user, setUser] = useState<User | null | undefined>(undefined);
  const [employments, setEmployments] = useState<Employment[]>([newEmployment()]);
  const [saving, setSaving] = useState(false);
  const [result, setResult] = useState<{ id: string; experience: SkillExperience[] } | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    getMe()
      .then(async (u) => {
        setUser(u);
        if (u) {
          const existing = await fetchMyPortfolio();
          if (existing && existing.employments.length > 0) {
            setEmployments(existing.employments);
          }
        }
      })
      .catch((e) => setError((e as Error).message));
  }, []);

  const updateEmployment = (id: string, next: Employment) =>
    setEmployments((prev) => prev.map((e) => (e.id === id ? next : e)));

  const removeEmployment = (id: string) =>
    setEmployments((prev) => prev.filter((e) => e.id !== id));

  const save = async () => {
    setSaving(true);
    setError(null);
    try {
      const { id } = await createPortfolio(employments);
      const experience = await fetchSkillExperience(id);
      setResult({ id, experience });
    } catch (e) {
      setError((e as Error).message);
    } finally {
      setSaving(false);
    }
  };

  const handleLogout = async () => {
    await logout();
    setUser(null);
    setEmployments([newEmployment()]);
    setResult(null);
  };

  if (user === undefined) {
    return <main><p className="muted">読み込み中...</p></main>;
  }

  if (user === null) {
    return (
      <main>
        <h1>Yokogushi</h1>
        <p className="muted">ポートフォリオを登録・共有するにはログインが必要です。</p>
        <a href="/api/auth/github/login" style={loginBtn}>
          <GitHubIcon /> GitHubでログイン
        </a>
      </main>
    );
  }

  return (
    <main>
      <header style={headerStyle}>
        <div style={{ display: "flex", alignItems: "center", gap: 10 }}>
          {user.avatar_url && (
            <img src={user.avatar_url} alt="" width={32} height={32} style={{ borderRadius: "50%" }} />
          )}
          <div style={{ fontSize: 14 }}>
            <div style={{ fontWeight: 600 }}>{user.name ?? user.username}</div>
            <div className="muted" style={{ fontSize: 12 }}>@{user.username}</div>
          </div>
        </div>
        <button onClick={handleLogout} style={logoutBtn}>ログアウト</button>
      </header>

      <h1>ポートフォリオ登録</h1>
      <p className="muted">
        所属（会社）と案件を入力して保存すると、スキルごとの経験期間が自動集計されます。
      </p>

      {employments.map((e) => (
        <EmploymentForm
          key={e.id}
          value={e}
          onChange={(next) => updateEmployment(e.id, next)}
          onRemove={() => removeEmployment(e.id)}
        />
      ))}

      <div style={{ marginTop: 16, display: "flex", gap: 8 }}>
        <button
          onClick={() => setEmployments((prev) => [...prev, newEmployment()])}
          style={secondaryBtn}
        >
          ＋ 所属を追加
        </button>
        <button onClick={save} disabled={saving} style={primaryBtn}>
          {saving ? "保存中..." : "保存して集計する"}
        </button>
      </div>

      {error && <p style={{ color: "#b91c1c", marginTop: 12 }}>{error}</p>}

      {result && (
        <section style={{ marginTop: 32 }}>
          <h2>スキル経験集計</h2>
          <p className="muted" style={{ fontSize: 12 }}>
            Portfolio ID: <code>{result.id}</code>
          </p>
          <SkillExperienceTable items={result.experience} />
        </section>
      )}
    </main>
  );
}

function GitHubIcon() {
  return (
    <svg viewBox="0 0 24 24" width="18" height="18" fill="currentColor" aria-hidden="true">
      <path d="M12 .5C5.73.5.5 5.73.5 12c0 5.08 3.29 9.39 7.86 10.91.58.1.79-.25.79-.56v-2.16c-3.2.7-3.88-1.36-3.88-1.36-.52-1.32-1.28-1.68-1.28-1.68-1.05-.72.08-.71.08-.71 1.16.08 1.77 1.19 1.77 1.19 1.03 1.76 2.7 1.25 3.36.96.1-.75.4-1.26.73-1.55-2.55-.29-5.24-1.27-5.24-5.67 0-1.25.45-2.27 1.18-3.07-.12-.29-.51-1.46.11-3.04 0 0 .97-.31 3.18 1.17a11 11 0 015.8 0c2.21-1.48 3.18-1.17 3.18-1.17.62 1.58.23 2.75.11 3.04.73.8 1.18 1.82 1.18 3.07 0 4.41-2.7 5.38-5.27 5.66.41.36.78 1.07.78 2.16v3.2c0 .31.21.66.8.55A10.52 10.52 0 0023.5 12C23.5 5.73 18.27.5 12 .5z"/>
    </svg>
  );
}

const loginBtn: React.CSSProperties = {
  display: "inline-flex",
  alignItems: "center",
  gap: 8,
  padding: "10px 16px",
  background: "#24292f",
  color: "#fff",
  borderRadius: 6,
  textDecoration: "none",
  fontSize: 14,
  fontWeight: 500,
};
const headerStyle: React.CSSProperties = {
  display: "flex",
  justifyContent: "space-between",
  alignItems: "center",
  paddingBottom: 16,
  borderBottom: "1px solid #eee",
  marginBottom: 24,
};
const logoutBtn: React.CSSProperties = {
  padding: "6px 12px",
  background: "#fff",
  border: "1px solid #ccc",
  borderRadius: 4,
  fontSize: 12,
  cursor: "pointer",
  color: "#666",
};
const primaryBtn: React.CSSProperties = {
  padding: "10px 20px",
  background: "#3730a3",
  color: "#fff",
  border: "none",
  borderRadius: 6,
  fontSize: 14,
  cursor: "pointer",
};
const secondaryBtn: React.CSSProperties = {
  padding: "10px 20px",
  background: "#fff",
  color: "#3730a3",
  border: "1px solid #3730a3",
  borderRadius: 6,
  fontSize: 14,
  cursor: "pointer",
};
