"use client";

import { useState } from "react";
import { EmploymentForm } from "../components/EmploymentForm";
import { SkillExperienceTable } from "../components/SkillExperienceTable";
import { createPortfolio, fetchSkillExperience } from "../lib/api";
import type { Employment, SkillExperience } from "../lib/types";

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
  const [employments, setEmployments] = useState<Employment[]>([newEmployment()]);
  const [saving, setSaving] = useState(false);
  const [result, setResult] = useState<{ id: string; experience: SkillExperience[] } | null>(null);
  const [error, setError] = useState<string | null>(null);

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

  return (
    <main>
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
