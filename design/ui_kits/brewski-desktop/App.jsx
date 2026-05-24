// App — top-level state, view router, theme persistence.

function EquipmentView() {
  const [profiles, setProfiles] = React.useState([
    { name: "10 gal Anvil Foundry", batch: 5.5, boil: 7.5, efficiency: 75 },
    { name: "5 gal Brew-in-a-Bag",  batch: 5.0, boil: 6.5, efficiency: 70 },
  ]);
  return (
    <div className="kit-pane">
      <div className="pane-head" style={{ justifyContent: "space-between" }}>
        <h1 style={{ font: "var(--fw-semibold) var(--fs-lg) var(--font-sans)", margin: 0, color: "var(--color-text-primary)" }}>
          Equipment Profiles
        </h1>
        <button className="btn btn--primary">+ New Profile</button>
      </div>
      <div className="kit-scroll" style={{ padding: 22, display: "grid", gridTemplateColumns: "repeat(2, minmax(0, 1fr))", gap: 14, alignContent: "start", maxWidth: 720 }}>
        {profiles.map((p, i) => (
          <window.Card key={i} title={p.name}>
            <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: 12 }}>
              <div><div className="h-eyebrow-x">Batch</div><div className="t-stat-value" style={{ fontSize: 20 }}>{p.batch} <span style={{ fontSize: 12, color: "var(--color-text-muted)", fontFamily: "var(--font-sans)" }}>gal</span></div></div>
              <div><div className="h-eyebrow-x">Boil</div><div className="t-stat-value" style={{ fontSize: 20 }}>{p.boil} <span style={{ fontSize: 12, color: "var(--color-text-muted)", fontFamily: "var(--font-sans)" }}>gal</span></div></div>
              <div><div className="h-eyebrow-x">Efficiency</div><div className="t-stat-value" style={{ fontSize: 20 }}>{p.efficiency}<span style={{ fontSize: 12, color: "var(--color-text-muted)", fontFamily: "var(--font-sans)" }}>%</span></div></div>
            </div>
          </window.Card>
        ))}
      </div>
    </div>
  );
}

function App() {
  const [theme, setTheme] = React.useState("midnight");
  const [units, setUnits] = React.useState("imperial");
  const [view, setView]   = React.useState("recipes");
  const [recipes, setRecipes] = React.useState(window.BREWSKI_RECIPES);

  React.useEffect(() => {
    document.documentElement.dataset.theme = theme;
  }, [theme]);

  return (
    <window.AppShell view={view} onViewChange={setView} theme={theme} onThemeChange={setTheme}>
      {view === "recipes"   && <window.RecipeView   recipes={recipes} onRecipesChange={setRecipes} />}
      {view === "batches"   && <window.BatchesView />}
      {view === "tools"     && <window.ToolView />}
      {view === "equipment" && <EquipmentView />}
      {view === "library"   && <window.LibraryView />}
      {view === "settings"  && <window.SettingsView
        theme={theme} onThemeChange={setTheme}
        units={units} onUnitsChange={setUnits}
      />}
    </window.AppShell>
  );
}

window.App = App;

// boot
const root = ReactDOM.createRoot(document.getElementById("root"));
root.render(<App />);
