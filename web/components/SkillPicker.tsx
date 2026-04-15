"use client";

import { Autocomplete } from "./Autocomplete";
import { SKILL_USAGE_LABELS } from "../lib/labels";
import type { ProjectSkill, Skill, SkillUsage } from "../lib/types";

type Props = {
  value: ProjectSkill[];
  onChange: (next: ProjectSkill[]) => void;
};

export function SkillPicker({ value, onChange }: Props) {
  const add = (s: Skill) => {
    if (value.some((v) => v.skill_id === s.id)) return;
    onChange([...value, { skill_id: s.id, usage: "primary" }]);
  };
  const update = (id: string, usage: SkillUsage) => {
    onChange(value.map((v) => (v.skill_id === id ? { ...v, usage } : v)));
  };
  const remove = (id: string) => {
    onChange(value.filter((v) => v.skill_id !== id));
  };

  return (
    <div>
      <Autocomplete<Skill>
        endpoint="/api/dict/skills"
        placeholder="スキル名を入力（例: rust, k8s）"
        getLabel={(s) => s.name}
        getKey={(s) => s.id}
        renderMeta={(s) => s.category}
        onSelect={add}
      />
      {value.length > 0 && (
        <ul style={listStyle}>
          {value.map((ps) => (
            <li key={ps.skill_id} style={itemStyle}>
              <span style={{ flex: 1 }}>{ps.skill_id}</span>
              <select
                value={ps.usage}
                onChange={(e) => update(ps.skill_id, e.target.value as SkillUsage)}
                style={selectStyle}
              >
                {Object.entries(SKILL_USAGE_LABELS).map(([k, label]) => (
                  <option key={k} value={k}>
                    {label}
                  </option>
                ))}
              </select>
              <button onClick={() => remove(ps.skill_id)} style={removeStyle} aria-label="remove">
                ×
              </button>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}

const listStyle: React.CSSProperties = {
  listStyle: "none",
  padding: 0,
  margin: "12px 0 0",
  display: "flex",
  flexDirection: "column",
  gap: 4,
};
const itemStyle: React.CSSProperties = {
  display: "flex",
  alignItems: "center",
  gap: 8,
  padding: "6px 10px",
  background: "#f5f5f5",
  borderRadius: 4,
  fontSize: 13,
};
const selectStyle: React.CSSProperties = {
  fontSize: 12,
  padding: "2px 4px",
};
const removeStyle: React.CSSProperties = {
  border: "none",
  background: "transparent",
  cursor: "pointer",
  color: "#888",
  fontSize: 16,
  lineHeight: 1,
};
