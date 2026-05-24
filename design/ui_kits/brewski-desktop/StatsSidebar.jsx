// StatsSidebar — the 176px right rail with live brew stats.
// Mirrors src/lib/components/StatsSidebar.svelte.

function StatItem({ label, value, unit, progress, progressColor }) {
  return (
    <div className="stat-card">
      <div className="lbl">{label}</div>
      <div className="val">
        {value}{unit && <span className="unit">{unit}</span>}
      </div>
      {progress !== undefined && (
        <div className="bar">
          <div style={{ width: `${Math.min(100, Math.max(0, progress * 100))}%`, background: progressColor || "var(--color-accent)" }} />
        </div>
      )}
    </div>
  );
}

function StatsSidebar({ recipe }) {
  if (!recipe) {
    return (
      <aside className="stats">
        <p className="h-eyebrow-x" style={{ marginBottom: 4 }}>Stats</p>
        <p style={{ fontSize: 12, color: "var(--color-text-muted)" }}>
          Add ingredients to see stats
        </p>
      </aside>
    );
  }

  const pct = (v, min, max) => (v - min) / (max - min);

  return (
    <aside className="stats">
      <p className="h-eyebrow-x" style={{ marginBottom: 4 }}>Stats</p>
      <StatItem label="OG"     value={recipe.og.toFixed(3)}     progress={pct(recipe.og, 1.000, 1.120)} />
      <StatItem label="FG"     value={recipe.fg.toFixed(3)}     progress={pct(recipe.fg, 1.000, 1.030)} />
      <StatItem label="ABV"    value={recipe.abv.toFixed(1)} unit="%" progress={pct(recipe.abv, 0, 12)} progressColor="#a6e3a1" />
      <StatItem label="IBU"    value={recipe.ibu.toFixed(0)}    progress={pct(recipe.ibu, 0, 120)}     progressColor="#fab387" />

      <div className="stat-card">
        <div className="lbl">SRM</div>
        <div className="val" style={{ display: "flex", alignItems: "center", gap: 8 }}>
          <span style={{ width: 18, height: 18, borderRadius: 4, background: window.srmToHex(recipe.srm), border: "1px solid rgba(255,255,255,0.15)", flexShrink: 0 }} />
          {recipe.srm.toFixed(1)}
        </div>
      </div>

      <StatItem label="BU:GU"     value={recipe.bu_gu.toFixed(2)} />
      <StatItem label="Cal / 12oz" value={recipe.cal_per_12oz} />

      <p className="h-eyebrow-x" style={{ marginTop: 6, marginBottom: 2 }}>Volumes</p>
      <StatItem label="Pre-boil"   value={recipe.pre_boil_vol.toFixed(1)} unit=" gal" />
      <StatItem label="Post-boil"  value={recipe.post_boil_vol.toFixed(1)} unit=" gal" />
      <StatItem label="Pre-boil G" value={recipe.pre_boil_g.toFixed(3)} />
    </aside>
  );
}

window.StatsSidebar = StatsSidebar;
