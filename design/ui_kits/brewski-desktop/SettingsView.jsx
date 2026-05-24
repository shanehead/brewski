// SettingsView and LibraryView — small placeholder routes.

function SettingsView({ theme, onThemeChange, units, onUnitsChange }) {
  return (
    <div className="kit-scroll" style={{ padding: 24 }}>
      <h1 style={{ font: "var(--fw-semibold) var(--fs-lg)/1.2 var(--font-sans)", margin: "0 0 24px", color: "var(--color-text-primary)" }}>
        Settings
      </h1>
      <div className="gap-4" style={{ maxWidth: 460 }}>
        <section>
          <h2 className="h-eyebrow-x" style={{ marginBottom: 10 }}>Appearance</h2>
          <div style={{ display: "flex", alignItems: "center", justifyContent: "space-between", padding: "8px 0" }}>
            <label htmlFor="set-theme" style={{ fontSize: 14, color: "var(--color-text-primary)" }}>Theme</label>
            <select id="set-theme" value={theme} onChange={(e) => onThemeChange(e.target.value)} style={{
              background: "var(--color-bg-elevated)", color: "var(--color-text-primary)",
              border: "1px solid var(--color-border)", borderRadius: 6, padding: "6px 8px", fontSize: 13,
            }}>
              {window.THEMES.map((t) => <option key={t.id} value={t.id}>{t.name}</option>)}
            </select>
          </div>
        </section>

        <section>
          <h2 className="h-eyebrow-x" style={{ marginBottom: 10 }}>Units</h2>
          <div style={{ display: "flex", alignItems: "center", justifyContent: "space-between", padding: "8px 0" }}>
            <label htmlFor="set-units" style={{ fontSize: 14, color: "var(--color-text-primary)" }}>Measurement system</label>
            <select id="set-units" value={units} onChange={(e) => onUnitsChange(e.target.value)} style={{
              background: "var(--color-bg-elevated)", color: "var(--color-text-primary)",
              border: "1px solid var(--color-border)", borderRadius: 6, padding: "6px 8px", fontSize: 13,
            }}>
              <option value="imperial">Imperial (gal, lb, °F)</option>
              <option value="metric">Metric (L, kg, °C)</option>
            </select>
          </div>
        </section>

        <section>
          <h2 className="h-eyebrow-x" style={{ marginBottom: 10 }}>Database</h2>
          <div style={{ padding: 12, background: "var(--color-bg-surface)", border: "1px solid var(--color-border)", borderRadius: 8, fontSize: 12, color: "var(--color-text-secondary)" }}>
            <div style={{ fontWeight: 500, color: "var(--color-text-primary)", marginBottom: 4 }}>
              ~/Library/Application Support/brewski/brewski.db
            </div>
            <div>Local SQLite file. Point at iCloud / Dropbox to sync.</div>
          </div>
        </section>

        <section>
          <h2 className="h-eyebrow-x" style={{ marginBottom: 10 }}>Ingredients</h2>
          <a href="#library" style={{
            display: "flex", alignItems: "center", justifyContent: "space-between",
            padding: "10px 12px", background: "var(--color-bg-elevated)", color: "var(--color-text-primary)",
            borderRadius: 6, textDecoration: "none", fontSize: 14,
          }}>
            <span>Ingredient Library</span>
            <window.ChevronRight />
          </a>
        </section>
      </div>
    </div>
  );
}

function LibraryView() {
  const [tab, setTab] = React.useState("hop");
  const ITEMS = {
    hop: [
      { name: "Citra", subtext: "12.0% AA · Pellet", seeded: true },
      { name: "Simcoe", subtext: "13.0% AA · Pellet", seeded: true },
      { name: "Mosaic", subtext: "12.5% AA · Pellet", seeded: true },
      { name: "Galaxy", subtext: "14.0% AA · Pellet", seeded: true },
      { name: "East Kent Goldings", subtext: "5.0% AA · Pellet", seeded: true },
      { name: "House blend (Citra/Mosaic 50:50)", subtext: "12.25% AA · Pellet", seeded: false },
    ],
    fermentable: [
      { name: "Maris Otter Pale", subtext: "Grain · 81% yield · 3°L", seeded: true },
      { name: "American 2-row", subtext: "Grain · 80% yield · 2°L", seeded: true },
      { name: "Crystal 60L", subtext: "Grain · 74% yield · 60°L", seeded: true },
      { name: "Munich", subtext: "Grain · 79% yield · 9°L", seeded: true },
      { name: "Flaked oats", subtext: "Adjunct · 73% yield · 1°L", seeded: true },
    ],
    yeast: [
      { name: "Wyeast 1056 American Ale", subtext: "Ale · Liquid", seeded: true },
      { name: "WLP001 California Ale", subtext: "Ale · Liquid", seeded: true },
      { name: "SafAle US-05", subtext: "Ale · Dry", seeded: true },
      { name: "Wyeast 3711 French Saison", subtext: "Saison · Liquid", seeded: true },
    ],
    misc: [
      { name: "Whirlfloc", subtext: "Fining · Boil", seeded: true },
      { name: "Yeast nutrient", subtext: "Other · Boil", seeded: true },
    ],
    water: [
      { name: "RO water + gypsum/CaCl", subtext: "Ca:50 Mg:5 Na:5 SO₄:80 Cl:70 HCO₃:30", seeded: false },
    ],
  };
  const TABS = [["hop","Hops"],["fermentable","Fermentables"],["yeast","Yeasts"],["misc","Misc"],["water","Water"]];

  return (
    <div className="kit-pane">
      <div className="pane-head" style={{ justifyContent: "space-between" }}>
        <h1 style={{ font: "var(--fw-semibold) var(--fs-lg) var(--font-sans)", margin: 0, color: "var(--color-text-primary)" }}>
          Ingredient Library
        </h1>
        <button className="btn btn--primary">+ New {tab === "hop" ? "Hop" : tab.charAt(0).toUpperCase() + tab.slice(1)}</button>
      </div>

      <div style={{ display: "flex", padding: "10px 24px 0", borderBottom: "1px solid var(--color-border)" }}>
        {TABS.map(([k, name]) => (
          <button key={k} onClick={() => setTab(k)}
            style={{
              background: "transparent", border: 0, cursor: "pointer",
              padding: "8px 14px",
              color: tab === k ? "var(--color-accent)" : "var(--color-text-secondary)",
              borderBottom: tab === k ? "2px solid var(--color-accent)" : "2px solid transparent",
              marginBottom: -1, fontSize: 13, fontWeight: 500,
            }}
          >{name}</button>
        ))}
      </div>

      <div className="kit-scroll" style={{ padding: 14 }}>
        {ITEMS[tab].map((item, i) => (
          <div key={i} style={{
            display: "flex", alignItems: "center", gap: 10,
            padding: "10px 12px", margin: "0 0 6px",
            background: "var(--color-bg-elevated)", border: "1px solid var(--color-border)",
            borderRadius: 6, cursor: "pointer",
          }}>
            <div style={{ flex: 1, minWidth: 0 }}>
              <div style={{ display: "flex", gap: 8, alignItems: "center" }}>
                <span style={{ fontSize: 14, fontWeight: 500, color: "var(--color-text-primary)" }}>{item.name}</span>
                <span style={{
                  fontSize: 11, fontWeight: 500, padding: "2px 8px", borderRadius: 9999,
                  background: item.seeded ? "var(--color-bg-surface)" : "color-mix(in srgb, var(--color-accent) 15%, transparent)",
                  color:      item.seeded ? "var(--color-text-muted)" : "var(--color-accent)",
                  border: `1px solid ${item.seeded ? "var(--color-border)" : "color-mix(in srgb, var(--color-accent) 40%, transparent)"}`,
                }}>{item.seeded ? "built-in" : "custom"}</span>
              </div>
              <div style={{ fontSize: 12, color: "var(--color-text-secondary)", marginTop: 2 }}>{item.subtext}</div>
            </div>
            {!item.seeded && (
              <>
                <button className="btn btn--secondary" style={{ padding: "4px 10px", fontSize: 11 }}>Edit</button>
                <button className="btn btn--secondary" style={{ padding: "4px 10px", fontSize: 11, color: "#f87171" }}>Delete</button>
              </>
            )}
          </div>
        ))}
      </div>
    </div>
  );
}

window.SettingsView = SettingsView;
window.LibraryView = LibraryView;
