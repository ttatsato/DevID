"use client";

import { SkillPicker } from "./SkillPicker";
import { INDUSTRY_LABELS, RESPONSIBILITY_LABELS } from "../lib/labels";
import type { Industry, Project, Responsibility } from "../lib/types";

type Props = {
  value: Project;
  onChange: (next: Project) => void;
  onRemove: () => void;
};

export function ProjectForm({ value, onChange, onRemove }: Props) {
  const set = <K extends keyof Project>(k: K, v: Project[K]) =>
    onChange({ ...value, [k]: v });

  const toggleResp = (r: Responsibility) => {
    const has = value.responsibilities.includes(r);
    set(
      "responsibilities",
      has ? value.responsibilities.filter((x) => x !== r) : [...value.responsibilities, r]
    );
  };

  return (
    <div style={cardStyle}>
      <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center" }}>
        <strong style={{ fontSize: 14 }}>案件</strong>
        <button onClick={onRemove} style={smallBtn}>削除</button>
      </div>

      <Field label="案件名">
        <input value={value.name} onChange={(e) => set("name", e.target.value)} style={inputStyle} />
      </Field>

      <Row>
        <Field label="クライアント名">
          <input
            value={value.client_name ?? ""}
            onChange={(e) => set("client_name", e.target.value || null)}
            style={inputStyle}
          />
        </Field>
        <Field label="業界">
          <select
            value={value.client_industry ?? ""}
            onChange={(e) => set("client_industry", (e.target.value || null) as Industry | null)}
            style={inputStyle}
          >
            <option value="">—</option>
            {Object.entries(INDUSTRY_LABELS).map(([k, label]) => (
              <option key={k} value={k}>{label}</option>
            ))}
          </select>
        </Field>
      </Row>

      <Row>
        <Field label="ロール">
          <input value={value.role} onChange={(e) => set("role", e.target.value)} style={inputStyle} />
        </Field>
        <Field label="チーム人数">
          <input
            type="number"
            min={1}
            value={value.team_size ?? ""}
            onChange={(e) => set("team_size", e.target.value ? Number(e.target.value) : null)}
            style={inputStyle}
          />
        </Field>
      </Row>

      <Row>
        <Field label="開始">
          <input
            type="date"
            value={value.start_date}
            onChange={(e) => set("start_date", e.target.value)}
            style={inputStyle}
          />
        </Field>
        <Field label="終了 (空欄=継続中)">
          <input
            type="date"
            value={value.end_date ?? ""}
            onChange={(e) => set("end_date", e.target.value || null)}
            style={inputStyle}
          />
        </Field>
      </Row>

      <Field label="概要">
        <textarea
          value={value.summary}
          onChange={(e) => set("summary", e.target.value)}
          rows={2}
          style={{ ...inputStyle, resize: "vertical" }}
        />
      </Field>

      <Field label="クライアント匿名化">
        <label style={{ fontSize: 13 }}>
          <input
            type="checkbox"
            checked={value.client_anonymized}
            onChange={(e) => set("client_anonymized", e.target.checked)}
          />
          {" "}公開時に匿名化する
        </label>
      </Field>

      <Field label="担当領域">
        <div style={{ display: "flex", flexWrap: "wrap", gap: 8 }}>
          {Object.entries(RESPONSIBILITY_LABELS).map(([k, label]) => (
            <label key={k} style={{ fontSize: 12 }}>
              <input
                type="checkbox"
                checked={value.responsibilities.includes(k as Responsibility)}
                onChange={() => toggleResp(k as Responsibility)}
              />
              {" "}{label}
            </label>
          ))}
        </div>
      </Field>

      <Field label="使用スキル">
        <SkillPicker
          value={value.skills}
          onChange={(skills) => set("skills", skills)}
        />
      </Field>
    </div>
  );
}

function Field({ label, children }: { label: string; children: React.ReactNode }) {
  return (
    <div style={{ marginTop: 10 }}>
      <div style={{ fontSize: 12, color: "#555", marginBottom: 4 }}>{label}</div>
      {children}
    </div>
  );
}

function Row({ children }: { children: React.ReactNode }) {
  return <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: 12 }}>{children}</div>;
}

const cardStyle: React.CSSProperties = {
  padding: 14,
  border: "1px solid #e5e5e5",
  borderRadius: 8,
  background: "#fff",
  marginTop: 12,
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
