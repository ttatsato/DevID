"use client";

import { useState } from "react";
import { Autocomplete } from "../components/Autocomplete";
import { TagList } from "../components/TagList";
import type { Skill, Certification } from "../lib/types";

export default function Page() {
  const [skills, setSkills] = useState<Skill[]>([]);
  const [certs, setCerts] = useState<Certification[]>([]);

  return (
    <main>
      <h1>ポートフォリオ登録</h1>
      <p className="muted">スキルと資格を入力してください。頭文字でサジェストされます。</p>

      <h2>スキル</h2>
      <Autocomplete<Skill>
        endpoint="/api/dict/skills"
        placeholder="例: rust, ja, k8s"
        getLabel={(s) => s.name}
        getKey={(s) => s.id}
        renderMeta={(s) => s.category}
        onSelect={(s) =>
          setSkills((prev) => (prev.some((p) => p.id === s.id) ? prev : [...prev, s]))
        }
      />
      <TagList
        items={skills}
        getLabel={(s) => s.name}
        getKey={(s) => s.id}
        onRemove={(s) => setSkills((prev) => prev.filter((p) => p.id !== s.id))}
      />

      <h2>資格</h2>
      <Autocomplete<Certification>
        endpoint="/api/dict/certs"
        placeholder="例: 基本, AWS, CKA"
        getLabel={(c) => c.name}
        getKey={(c) => c.id}
        renderMeta={(c) => c.issuer}
        onSelect={(c) =>
          setCerts((prev) => (prev.some((p) => p.id === c.id) ? prev : [...prev, c]))
        }
      />
      <TagList
        items={certs}
        getLabel={(c) => c.name}
        getKey={(c) => c.id}
        onRemove={(c) => setCerts((prev) => prev.filter((p) => p.id !== c.id))}
      />
    </main>
  );
}
