// StatPill — small status pill used in batch lists.

function StatPill({ status }) {
  const label = window.STATUS_LABELS[status] || status;
  const color = window.STATUS_COLORS[status] || "var(--color-text-muted)";
  return (
    <span className="pill" style={{ color }}>{label}</span>
  );
}

window.StatPill = StatPill;
