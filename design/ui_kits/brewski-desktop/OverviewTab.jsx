// OverviewTab — first tab of a recipe. Two cards: details + volumes.

function Field({ label, children }) {
  return (
    <div className="field">
      <label>{label}</label>
      {children}
    </div>
  );
}

function OverviewTab({ recipe, onChange }) {
  const set = (k, v) => onChange({ ...recipe, [k]: v });
  return (
    <div className="gap-4 maxw-2xl">
      <window.Card title="Recipe Details">
        <div className="grid-2">
          <Field label="Recipe Type">
            <select value={recipe.type} onChange={(e) => set("type", e.target.value)}>
              <option value="all_grain">all grain</option>
              <option value="extract">extract</option>
              <option value="partial_mash">partial mash</option>
            </select>
          </Field>
          <Field label="Brewer">
            <input value={recipe.brewer} onChange={(e) => set("brewer", e.target.value)} />
          </Field>
          <Field label="Style">
            <input value={recipe.style} onChange={(e) => set("style", e.target.value)} />
          </Field>
          <Field label="Date">
            <input type="date" value={recipe.date || ""} onChange={(e) => set("date", e.target.value)} />
          </Field>
          <div className="field" style={{ gridColumn: "1 / -1" }}>
            <label>Equipment Profile</label>
            <select defaultValue="">
              <option value="">None</option>
              <option>10 gal Anvil Foundry</option>
              <option>5 gal Brew-in-a-Bag</option>
            </select>
          </div>
        </div>
      </window.Card>

      <window.Card title="Volumes & Timing">
        <div className="grid-2">
          <Field label="Batch Size (gal)">
            <input type="number" step="0.1" value={recipe.batch_size_gal}
              onChange={(e) => set("batch_size_gal", parseFloat(e.target.value) || 0)} />
          </Field>
          <Field label="Boil Size (gal)">
            <input type="number" step="0.1" value={recipe.boil_size_gal}
              onChange={(e) => set("boil_size_gal", parseFloat(e.target.value) || 0)} />
          </Field>
          <Field label="Boil Time (min)">
            <input type="number" step="5" value={recipe.boil_time_min}
              onChange={(e) => set("boil_time_min", parseFloat(e.target.value) || 0)} />
          </Field>
          <Field label="Efficiency (%)">
            <input type="number" step="1" value={recipe.efficiency_pct}
              onChange={(e) => set("efficiency_pct", parseFloat(e.target.value) || 0)} />
          </Field>
        </div>
      </window.Card>

      {recipe.notes && (
        <window.Card title="Notes">
          <p style={{ margin: 0, fontSize: 14, lineHeight: 1.5, color: "var(--color-text-primary)" }}>
            {recipe.notes}
          </p>
        </window.Card>
      )}
    </div>
  );
}

window.OverviewTab = OverviewTab;
