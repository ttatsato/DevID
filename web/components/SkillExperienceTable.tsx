"use client";

import type { SkillExperience } from "../lib/types";

export function SkillExperienceTable({ items }: { items: SkillExperience[] }) {
  if (items.length === 0) {
    return <p className="muted">スキル実績なし</p>;
  }
  return (
    <table style={{ width: "100%", borderCollapse: "collapse", fontSize: 13 }}>
      <thead>
        <tr>
          <Th>スキル</Th>
          <Th align="right">合計</Th>
          <Th align="right">Primary</Th>
          <Th>最終使用</Th>
          <Th align="right">案件数</Th>
        </tr>
      </thead>
      <tbody>
        {items.map((e) => (
          <tr key={e.skill_id}>
            <Td>{e.skill_id}</Td>
            <Td align="right">{formatMonths(e.total_months)}</Td>
            <Td align="right">{formatMonths(e.primary_months)}</Td>
            <Td>{e.last_used}</Td>
            <Td align="right">{e.project_count}</Td>
          </tr>
        ))}
      </tbody>
    </table>
  );
}

function formatMonths(m: number): string {
  if (m < 12) return `${m}ヶ月`;
  const y = Math.floor(m / 12);
  const r = m % 12;
  return r === 0 ? `${y}年` : `${y}年${r}ヶ月`;
}

function Th({ children, align = "left" }: { children: React.ReactNode; align?: "left" | "right" }) {
  return (
    <th
      style={{
        textAlign: align,
        padding: "8px 10px",
        borderBottom: "2px solid #ddd",
        fontWeight: 600,
        color: "#555",
      }}
    >
      {children}
    </th>
  );
}

function Td({ children, align = "left" }: { children: React.ReactNode; align?: "left" | "right" }) {
  return (
    <td
      style={{
        textAlign: align,
        padding: "8px 10px",
        borderBottom: "1px solid #eee",
      }}
    >
      {children}
    </td>
  );
}
