"use client";

import { ProjectForm } from "./ProjectForm";
import { EMPLOYMENT_TYPE_LABELS } from "../lib/labels";
import type { Employment, EmploymentType, Project } from "../lib/types";

type Props = {
  value: Employment;
  onChange: (next: Employment) => void;
  onRemove: () => void;
};

const newProject = (): Project => ({
  id: crypto.randomUUID(),
  name: "",
  client_name: null,
  client_industry: null,
  client_anonymized: false,
  role: "",
  team_size: null,
  start_date: "",
  end_date: null,
  summary: "",
  achievements: [],
  skills: [],
  responsibilities: [],
});

export function EmploymentForm({ value, onChange, onRemove }: Props) {
  const set = <K extends keyof Employment>(k: K, v: Employment[K]) =>
    onChange({ ...value, [k]: v });

  const updateProject = (id: string, next: Project) =>
    set("projects", value.projects.map((p) => (p.id === id ? next : p)));

  const removeProject = (id: string) =>
    set("projects", value.projects.filter((p) => p.id !== id));

  return (
    <section style={cardStyle}>
      <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center" }}>
        <strong>所属</strong>
        <button onClick={onRemove} style={smallBtn}>所属ごと削除</button>
      </div>

      <div style={{ display: "grid", gridTemplateColumns: "2fr 1fr", gap: 12, marginTop: 10 }}>
        <Field label="会社名">
          <input
            value={value.company_name}
            onChange={(e) => set("company_name", e.target.value)}
            style={inputStyle}
          />
        </Field>
        <Field label="雇用形態">
          <select
            value={value.employment_type}
            onChange={(e) => set("employment_type", e.target.value as EmploymentType)}
            style={inputStyle}
          >
            {Object.entries(EMPLOYMENT_TYPE_LABELS).map(([k, label]) => (
              <option key={k} value={k}>{label}</option>
            ))}
          </select>
        </Field>
      </div>

      <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: 12, marginTop: 10 }}>
        <Field label="入社">
          <input
            type="date"
            value={value.start_date}
            onChange={(e) => set("start_date", e.target.value)}
            style={inputStyle}
          />
        </Field>
        <Field label="退社 (空欄=在職中)">
          <input
            type="date"
            value={value.end_date ?? ""}
            onChange={(e) => set("end_date", e.target.value || null)}
            style={inputStyle}
          />
        </Field>
      </div>

      <Field label="会社匿名化">
        <label style={{ fontSize: 13 }}>
          <input
            type="checkbox"
            checked={value.company_anonymized}
            onChange={(e) => set("company_anonymized", e.target.checked)}
          />
          {" "}公開時に匿名化する
        </label>
      </Field>

      <div style={{ marginTop: 16 }}>
        <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center" }}>
          <strong style={{ fontSize: 14 }}>案件 ({value.projects.length})</strong>
          <button
            onClick={() => set("projects", [...value.projects, newProject()])}
            style={addBtn}
          >
            ＋ 案件を追加
          </button>
        </div>
        {value.projects.map((p) => (
          <ProjectForm
            key={p.id}
            value={p}
            onChange={(next) => updateProject(p.id, next)}
            onRemove={() => removeProject(p.id)}
          />
        ))}
      </div>
    </section>
  );
}

function Field({ label, children }: { label: string; children: React.ReactNode }) {
  return (
    <div>
      <div style={{ fontSize: 12, color: "#555", marginBottom: 4 }}>{label}</div>
      {children}
    </div>
  );
}

const cardStyle: React.CSSProperties = {
  padding: 16,
  border: "1px solid #d4d4d4",
  borderRadius: 10,
  background: "#fafafa",
  marginTop: 16,
};
const inputStyle: React.CSSProperties = {
  width: "100%",
  padding: "8px 10px",
  fontSize: 13,
  border: "1px solid #ccc",
  borderRadius: 4,
};
const smallBtn: React.CSSProperties = {
  fontSize: 12,
  padding: "4px 8px",
  background: "#fff",
  border: "1px solid #ccc",
  borderRadius: 4,
  cursor: "pointer",
  color: "#666",
};
const addBtn: React.CSSProperties = {
  fontSize: 12,
  padding: "6px 12px",
  background: "#3730a3",
  color: "#fff",
  border: "none",
  borderRadius: 4,
  cursor: "pointer",
};
