"use client";

import { useState } from "react";
import { updateMyProfile } from "../lib/api";
import type { Profile, SocialLink, SocialPlatform } from "../lib/types";

const PLATFORMS: { value: SocialPlatform; label: string }[] = [
  { value: "github", label: "GitHub" },
  { value: "twitter", label: "X (Twitter)" },
  { value: "linkedin", label: "LinkedIn" },
  { value: "zenn", label: "Zenn" },
  { value: "qiita", label: "Qiita" },
  { value: "website", label: "Website" },
  { value: "blog", label: "Blog" },
  { value: "other", label: "その他" },
];

type Props = {
  initial: Profile;
};

export function ProfileForm({ initial }: Props) {
  const [profile, setProfile] = useState<Profile>(initial);
  const [saving, setSaving] = useState(false);
  const [message, setMessage] = useState<string | null>(null);

  const set = <K extends keyof Profile>(k: K, v: Profile[K]) =>
    setProfile((p) => ({ ...p, [k]: v }));

  const addLink = () =>
    set("social_links", [...profile.social_links, { platform: "github", url: "" }]);

  const updateLink = (idx: number, next: SocialLink) =>
    set(
      "social_links",
      profile.social_links.map((l, i) => (i === idx ? next : l))
    );

  const removeLink = (idx: number) =>
    set(
      "social_links",
      profile.social_links.filter((_, i) => i !== idx)
    );

  const save = async () => {
    setSaving(true);
    setMessage(null);
    try {
      const updated = await updateMyProfile(profile);
      setProfile(updated);
      setMessage("保存しました");
    } catch (e) {
      setMessage(`エラー: ${(e as Error).message}`);
    } finally {
      setSaving(false);
    }
  };

  return (
    <section style={sectionStyle}>
      <h2 style={{ marginTop: 0 }}>プロフィール</h2>

      <Row>
        <Field label="表示名">
          <input
            value={profile.display_name ?? ""}
            onChange={(e) => set("display_name", e.target.value || null)}
            placeholder="山田 太郎"
            style={inputStyle}
          />
        </Field>
        <Field label="肩書き">
          <input
            value={profile.headline ?? ""}
            onChange={(e) => set("headline", e.target.value || null)}
            placeholder="Backend Engineer"
            style={inputStyle}
          />
        </Field>
      </Row>

      <Field label="自己紹介 (Markdown 可)">
        <textarea
          value={profile.bio ?? ""}
          onChange={(e) => set("bio", e.target.value || null)}
          rows={4}
          style={{ ...inputStyle, resize: "vertical" }}
        />
      </Field>

      <Row>
        <Field label="所在地">
          <input
            value={profile.location ?? ""}
            onChange={(e) => set("location", e.target.value || null)}
            placeholder="東京"
            style={inputStyle}
          />
        </Field>
        <Field label="アバター URL (空欄でGitHub既定)">
          <input
            value={profile.avatar_url ?? ""}
            onChange={(e) => set("avatar_url", e.target.value || null)}
            style={inputStyle}
          />
        </Field>
      </Row>

      <Field label="連絡先メール">
        <div style={{ display: "flex", gap: 12, alignItems: "center" }}>
          <input
            type="email"
            value={profile.contact_email ?? ""}
            onChange={(e) => set("contact_email", e.target.value || null)}
            style={{ ...inputStyle, flex: 1 }}
          />
          <label style={{ fontSize: 13, whiteSpace: "nowrap" }}>
            <input
              type="checkbox"
              checked={profile.contact_email_public}
              onChange={(e) => set("contact_email_public", e.target.checked)}
            />
            {" "}公開する
          </label>
        </div>
      </Field>

      <Field label="SNSリンク">
        <div style={{ display: "flex", flexDirection: "column", gap: 6 }}>
          {profile.social_links.map((l, i) => (
            <div key={i} style={{ display: "flex", gap: 6 }}>
              <select
                value={l.platform}
                onChange={(e) =>
                  updateLink(i, { ...l, platform: e.target.value as SocialPlatform })
                }
                style={{ ...inputStyle, flex: "0 0 120px" }}
              >
                {PLATFORMS.map((p) => (
                  <option key={p.value} value={p.value}>
                    {p.label}
                  </option>
                ))}
              </select>
              <input
                type="url"
                placeholder="https://..."
                value={l.url}
                onChange={(e) => updateLink(i, { ...l, url: e.target.value })}
                style={{ ...inputStyle, flex: 1 }}
              />
              <button onClick={() => removeLink(i)} style={removeBtn}>×</button>
            </div>
          ))}
          <button onClick={addLink} style={addLinkBtn}>＋ リンクを追加</button>
        </div>
      </Field>

      <div style={{ marginTop: 16, display: "flex", alignItems: "center", gap: 12 }}>
        <button onClick={save} disabled={saving} style={primaryBtn}>
          {saving ? "保存中..." : "プロフィールを保存"}
        </button>
        {message && <span style={{ fontSize: 13, color: "#555" }}>{message}</span>}
      </div>
    </section>
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

const sectionStyle: React.CSSProperties = {
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
const primaryBtn: React.CSSProperties = {
  padding: "8px 16px",
  background: "#3730a3",
  color: "#fff",
  border: "none",
  borderRadius: 6,
  fontSize: 13,
  cursor: "pointer",
};
const removeBtn: React.CSSProperties = {
  flex: "0 0 32px",
  padding: "4px",
  background: "#fff",
  border: "1px solid #ccc",
  borderRadius: 4,
  cursor: "pointer",
  color: "#888",
};
const addLinkBtn: React.CSSProperties = {
  alignSelf: "flex-start",
  padding: "4px 10px",
  background: "#fff",
  border: "1px dashed #999",
  borderRadius: 4,
  fontSize: 12,
  cursor: "pointer",
  color: "#555",
};
