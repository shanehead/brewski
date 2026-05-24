// ToolView — the /tools route. Left list of standalone calculators,
// right pane shows the active tool. The ABV calculator is fully wired;
// other tools just render their result panel as a placeholder so the
// visual treatment is documented.

function AbvCalculator() {
  const [og, setOg] = React.useState(1.052);
  const [fg, setFg] = React.useState(1.013);

  const valid = og > 1 && fg > 0.98 && fg <= og;
  // Standard brewing formulas: ABV ≈ (OG - FG) * 131.25,
  // calories use real-extract (°P) and ABW. Per-12-oz (355 ml).
  const abv = valid ? (og - fg) * 131.25 : null;
  const attenuation = valid ? ((og - fg) / (og - 1)) * 100 : null;
  const ogPlato = valid ? ((og - 1) * 1000) / 4 : 0;
  const fgPlato = valid ? ((fg - 1) * 1000) / 4 : 0;
  const realExtract = valid ? 0.1808 * ogPlato + 0.8192 * fgPlato : null;
  const abw = valid ? (abv * 0.79) / fg : null;
  const calories = valid ? Math.round((6.9 * abw + 4.0 * realExtract) * fg * 3.55) : null;

  return (
    <div className="tool-pane">
      <h2 style={{ font: "var(--fw-semibold) var(--fs-xl)/1.2 var(--font-sans)", margin: 0, color: "var(--color-text-primary)" }}>
        ABV / Attenuation / Calories
      </h2>
      <p style={{ marginTop: 8, maxWidth: 640, font: "400 var(--fs-sm)/1.5 var(--font-sans)", color: "var(--color-text-secondary)" }}>
        Estimate beer strength, apparent attenuation, and calories per 12 oz serving from original and final gravity.
      </p>

      <div style={{ display: "flex", flexDirection: "column", gap: 18, marginTop: 24, maxWidth: 720 }}>
        <window.Card>
          <div className="field" style={{ marginBottom: 14 }}>
            <label>Original Gravity</label>
            <input type="number" min="1" max="1.2" step="0.001" value={og}
              onChange={(e) => setOg(parseFloat(e.target.value) || 1)} />
          </div>
          <div className="field">
            <label>Final Gravity</label>
            <input type="number" min="0.99" max="1.2" step="0.001" value={fg}
              onChange={(e) => setFg(parseFloat(e.target.value) || 0.99)} />
          </div>
        </window.Card>

        <div className="tool-result">
          {valid ? <>
            <div><div className="lbl">ABV</div><div className="val">{abv.toFixed(1)}%</div></div>
            <div><div className="lbl">Attenuation</div><div className="val">{attenuation.toFixed(1)}%</div></div>
            <div><div className="lbl">Calories / 12 oz</div><div className="val">{calories}</div></div>
          </> : (
            <div style={{ gridColumn: "1 / -1", fontSize: 13, color: "var(--color-text-secondary)" }}>
              Enter a valid OG and FG to calculate results.
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

function ToolPlaceholder({ tool }) {
  return (
    <div className="tool-pane">
      <h2 style={{ font: "var(--fw-semibold) var(--fs-xl)/1.2 var(--font-sans)", margin: 0, color: "var(--color-text-primary)" }}>
        {tool.name}
      </h2>
      <p style={{ marginTop: 8, maxWidth: 640, font: "400 var(--fs-sm)/1.5 var(--font-sans)", color: "var(--color-text-secondary)" }}>
        {tool.desc}
      </p>
      <div style={{ marginTop: 24, padding: "44px 22px", background: "var(--color-bg-surface)", border: "1px dashed var(--color-border)", borderRadius: 12, color: "var(--color-text-muted)", fontSize: 13, textAlign: "center" }}>
        This tool is part of Brewski. In this kit it stays as a stub — the
        ABV / Calories calculator is the fully-wired example.
      </div>
    </div>
  );
}

function ToolView() {
  const [active, setActive] = React.useState("abv");
  const tool = window.BREWSKI_TOOLS.find((t) => t.slug === active);

  return (
    <div className="tools-layout">
      <aside className="tools-list">
        <div style={{ padding: "16px 18px 8px" }}>
          <h1 style={{ font: "var(--fw-semibold) var(--fs-lg) var(--font-sans)", margin: 0, color: "var(--color-text-primary)" }}>
            Tools
          </h1>
          <p style={{ font: "400 var(--fs-sm)/1.5 var(--font-sans)", margin: "4px 0 8px", color: "var(--color-text-secondary)" }}>
            Standalone brewing calculators.
          </p>
        </div>
        {window.BREWSKI_TOOLS.map((t) => (
          <button
            key={t.slug}
            className={"tool-row" + (active === t.slug ? " active" : "")}
            onClick={() => setActive(t.slug)}
          >
            <div className="ttl">{t.name}</div>
            <div className="desc">{t.desc}</div>
          </button>
        ))}
      </aside>

      {active === "abv" ? <AbvCalculator /> : <ToolPlaceholder tool={tool} />}
    </div>
  );
}

window.ToolView = ToolView;
