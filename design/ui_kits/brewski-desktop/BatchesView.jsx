// BatchesView — /batches route. Sidebar of batches + empty hint.

function BatchesView() {
  const [selectedId, setSelectedId] = React.useState(null);
  const batches = window.BREWSKI_BATCHES;

  return <>
    <aside className="sidebar">
      <div className="sidebar-head">
        <button className="btn btn--primary">+ New Batch</button>
      </div>
      <ul style={{ listStyle: "none", margin: 0, padding: "8px", flex: 1, overflowY: "auto", display: "flex", flexDirection: "column", gap: 4 }}>
        {batches.map((b) => (
          <li key={b.id}>
            <div
              className="list-row"
              style={{
                background: "var(--color-bg-elevated)",
                borderLeft: "0",
                paddingLeft: "12px",
                borderRadius: "var(--radius-md)",
                cursor: "pointer",
              }}
              onClick={() => setSelectedId(b.id)}
            >
              <div style={{ display: "flex", alignItems: "center", gap: 8 }}>
                <span className="nm" style={{ flex: 1 }}>{b.recipe_name}</span>
                <window.StatPill status={b.status} />
              </div>
              <span className="sub">{b.name} · {new Date(b.brew_date).toLocaleDateString()}</span>
            </div>
          </li>
        ))}
      </ul>
    </aside>

    <div className="kit-pane" style={{ alignItems: "center", justifyContent: "center" }}>
      {selectedId ? (
        <BatchDetail batch={batches.find((b) => b.id === selectedId)} />
      ) : (
        <p style={{ fontSize: 13, color: "var(--color-text-muted)" }}>
          Select a batch to view
        </p>
      )}
    </div>
  </>;
}

function BatchDetail({ batch }) {
  return (
    <div className="kit-scroll" style={{ width: "100%", padding: 22 }}>
      <header className="pane-head" style={{ borderRadius: 12, marginBottom: 18, background: "var(--color-bg-surface)", border: "1px solid var(--color-border)" }}>
        <span style={{ flex: 1, fontSize: 16, fontWeight: 600 }}>{batch.recipe_name}</span>
        <window.StatPill status={batch.status} />
      </header>

      <div className="gap-4" style={{ maxWidth: 720 }}>
        <window.Card title="Brew day">
          <div className="grid-2">
            <div className="field"><label>Brew date</label><input value={new Date(batch.brew_date).toLocaleDateString()} readOnly /></div>
            <div className="field"><label>Batch name</label><input defaultValue={batch.name} /></div>
            <div className="field"><label>Actual OG</label><input defaultValue={batch.actual_og.toFixed(3)} /></div>
            <div className="field"><label>Target OG</label><input defaultValue="1.052" /></div>
          </div>
        </window.Card>

        <window.Card title="Fermentation">
          <div style={{ display: "flex", gap: 16, padding: "8px 0" }}>
            {[
              { day: "Day 1", t: "68°F", g: "1.050" },
              { day: "Day 3", t: "68°F", g: "1.034" },
              { day: "Day 7", t: "68°F", g: "1.018" },
              { day: "Day 14", t: "68°F", g: "1.012" },
            ].map((row, i) => (
              <div key={i} style={{ flex: 1, padding: 10, background: "var(--color-bg-elevated)", borderRadius: 8, border: "1px solid var(--color-border)" }}>
                <div style={{ font: "500 11px var(--font-sans)", color: "var(--color-text-muted)" }}>{row.day}</div>
                <div style={{ font: "700 18px var(--font-mono)", color: "var(--color-text-primary)", marginTop: 4 }}>{row.g}</div>
                <div style={{ font: "500 12px var(--font-sans)", color: "var(--color-text-secondary)", marginTop: 4 }}>{row.t}</div>
              </div>
            ))}
          </div>
        </window.Card>
      </div>
    </div>
  );
}

window.BatchesView = BatchesView;
