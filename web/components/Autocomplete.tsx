"use client";

import { useEffect, useRef, useState } from "react";

type Props<T> = {
  endpoint: string;
  placeholder?: string;
  getLabel: (item: T) => string;
  getKey: (item: T) => string;
  renderMeta?: (item: T) => string;
  onSelect: (item: T) => void;
};

export function Autocomplete<T>({
  endpoint,
  placeholder,
  getLabel,
  getKey,
  renderMeta,
  onSelect,
}: Props<T>) {
  const [query, setQuery] = useState("");
  const [items, setItems] = useState<T[]>([]);
  const [open, setOpen] = useState(false);
  const [active, setActive] = useState(0);
  const abortRef = useRef<AbortController | null>(null);

  useEffect(() => {
    if (!query.trim()) {
      setItems([]);
      return;
    }
    abortRef.current?.abort();
    const ac = new AbortController();
    abortRef.current = ac;
    const t = setTimeout(async () => {
      try {
        const res = await fetch(
          `${endpoint}?q=${encodeURIComponent(query)}&limit=8`,
          { signal: ac.signal }
        );
        if (!res.ok) return;
        const data = (await res.json()) as T[];
        setItems(data);
        setActive(0);
      } catch (e) {
        if ((e as Error).name !== "AbortError") console.error(e);
      }
    }, 120);
    return () => {
      clearTimeout(t);
      ac.abort();
    };
  }, [query, endpoint]);

  const choose = (item: T) => {
    onSelect(item);
    setQuery("");
    setItems([]);
    setOpen(false);
  };

  return (
    <div style={{ position: "relative", marginBottom: 8 }}>
      <input
        value={query}
        placeholder={placeholder}
        onChange={(e) => {
          setQuery(e.target.value);
          setOpen(true);
        }}
        onFocus={() => setOpen(true)}
        onBlur={() => setTimeout(() => setOpen(false), 120)}
        onKeyDown={(e) => {
          if (e.key === "ArrowDown") {
            e.preventDefault();
            setActive((a) => Math.min(a + 1, items.length - 1));
          } else if (e.key === "ArrowUp") {
            e.preventDefault();
            setActive((a) => Math.max(a - 1, 0));
          } else if (e.key === "Enter" && items[active]) {
            e.preventDefault();
            choose(items[active]);
          }
        }}
        style={{
          width: "100%",
          padding: "10px 12px",
          fontSize: 14,
          border: "1px solid #ccc",
          borderRadius: 6,
          outline: "none",
        }}
      />
      {open && items.length > 0 && (
        <ul
          style={{
            position: "absolute",
            top: "100%",
            left: 0,
            right: 0,
            margin: 0,
            padding: 0,
            listStyle: "none",
            background: "#fff",
            border: "1px solid #ddd",
            borderRadius: 6,
            marginTop: 4,
            maxHeight: 280,
            overflowY: "auto",
            zIndex: 10,
            boxShadow: "0 4px 12px rgba(0,0,0,0.08)",
          }}
        >
          {items.map((item, i) => (
            <li
              key={getKey(item)}
              onMouseDown={(e) => {
                e.preventDefault();
                choose(item);
              }}
              onMouseEnter={() => setActive(i)}
              style={{
                padding: "8px 12px",
                cursor: "pointer",
                background: i === active ? "#f0f4ff" : "transparent",
                display: "flex",
                justifyContent: "space-between",
                fontSize: 14,
              }}
            >
              <span>{getLabel(item)}</span>
              {renderMeta && (
                <span style={{ color: "#888", fontSize: 12 }}>
                  {renderMeta(item)}
                </span>
              )}
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}
