"use client";

type Props<T> = {
  items: T[];
  getLabel: (item: T) => string;
  getKey: (item: T) => string;
  onRemove: (item: T) => void;
};

export function TagList<T>({ items, getLabel, getKey, onRemove }: Props<T>) {
  if (items.length === 0) {
    return <p className="muted" style={{ marginTop: 8 }}>未選択</p>;
  }
  return (
    <ul
      style={{
        listStyle: "none",
        padding: 0,
        margin: "12px 0 0",
        display: "flex",
        flexWrap: "wrap",
        gap: 8,
      }}
    >
      {items.map((item) => (
        <li
          key={getKey(item)}
          style={{
            display: "inline-flex",
            alignItems: "center",
            gap: 6,
            padding: "4px 10px",
            background: "#eef2ff",
            color: "#3730a3",
            borderRadius: 999,
            fontSize: 13,
          }}
        >
          {getLabel(item)}
          <button
            onClick={() => onRemove(item)}
            style={{
              border: "none",
              background: "transparent",
              color: "#3730a3",
              cursor: "pointer",
              fontSize: 14,
              padding: 0,
              lineHeight: 1,
            }}
            aria-label="remove"
          >
            ×
          </button>
        </li>
      ))}
    </ul>
  );
}
