// IngredientsTab — table view of fermentables, hops, yeasts, water additions.
// In the real app this is a rich table with edit-in-place; here we
// recreate the look with a couple of sample rows so the design
// system has a high-fidelity reference.

function IngredientsTab({ recipe }) {
  return (
    <div className="gap-4" style={{ maxWidth: 920 }}>
      <window.Card title="Fermentables">
        <table className="ing-table">
          <thead>
            <tr><th>Name</th><th className="r">Amount</th><th className="r">Yield</th><th className="r">°L</th><th className="r">%</th></tr>
          </thead>
          <tbody>
            <tr><td><span className="ing-dot" style={{background:"#fbbf24"}}/>Maris Otter Pale</td><td className="r mono">10.0 lb</td><td className="r mono">81%</td><td className="r mono">3.0</td><td className="r mono">87%</td></tr>
            <tr><td><span className="ing-dot" style={{background:"#f59e0b"}}/>Crystal 60</td><td className="r mono">0.75 lb</td><td className="r mono">74%</td><td className="r mono">60</td><td className="r mono">6.5%</td></tr>
            <tr><td><span className="ing-dot" style={{background:"#d97706"}}/>Munich</td><td className="r mono">0.75 lb</td><td className="r mono">79%</td><td className="r mono">9</td><td className="r mono">6.5%</td></tr>
          </tbody>
        </table>
      </window.Card>

      <window.Card title="Hops">
        <table className="ing-table">
          <thead>
            <tr><th>Name</th><th className="r">Amount</th><th className="r">Time</th><th className="r">% AA</th><th className="r">Use</th></tr>
          </thead>
          <tbody>
            <tr><td><span className="ing-dot" style={{background:"#22c55e"}}/>Citra</td><td className="r mono">1.0 oz</td><td className="r mono">60 min</td><td className="r mono">12.0%</td><td className="r mono">Boil</td></tr>
            <tr><td><span className="ing-dot" style={{background:"#22c55e"}}/>Simcoe</td><td className="r mono">1.0 oz</td><td className="r mono">15 min</td><td className="r mono">13.0%</td><td className="r mono">Boil</td></tr>
            <tr><td><span className="ing-dot" style={{background:"#16a34a"}}/>Mosaic</td><td className="r mono">2.0 oz</td><td className="r mono">Day 3</td><td className="r mono">12.5%</td><td className="r mono">Dry hop</td></tr>
          </tbody>
        </table>
      </window.Card>

      <window.Card title="Yeast">
        <table className="ing-table">
          <thead>
            <tr><th>Name</th><th className="r">Form</th><th className="r">Att.</th></tr>
          </thead>
          <tbody>
            <tr><td><span className="ing-dot" style={{background:"#14b8a6"}}/>Wyeast 1056 American Ale</td><td className="r mono">Liquid</td><td className="r mono">76%</td></tr>
          </tbody>
        </table>
      </window.Card>

      <style>{`
        .ing-table { width: 100%; border-collapse: collapse; font-size: 13px; }
        .ing-table th { text-align: left; font: 600 11px var(--font-sans); text-transform: uppercase; letter-spacing: 0.08em; color: var(--color-text-muted); padding: 0 8px 8px; }
        .ing-table th.r, .ing-table td.r { text-align: right; }
        .ing-table td { padding: 8px; border-top: 1px solid var(--color-border); color: var(--color-text-primary); }
        .ing-table .mono { font-family: var(--font-mono); font-variant-numeric: tabular-nums; }
        .ing-dot { display: inline-block; width: 8px; height: 8px; border-radius: 50%; margin-right: 8px; vertical-align: middle; }
      `}</style>
    </div>
  );
}

window.IngredientsTab = IngredientsTab;
