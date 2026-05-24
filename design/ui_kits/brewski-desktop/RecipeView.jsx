// RecipeView — open recipe with sidebar + header + tabbar + main + stats sidebar.

const RECIPE_TABS = [
  { key: "overview",     label: "Overview",     icon: "overview" },
  { key: "ingredients",  label: "Ingredients",  icon: "ingredients" },
  { key: "mash",         label: "Mash",         icon: "mash" },
  { key: "water",        label: "Water",        icon: "water" },
  { key: "fermentation", label: "Fermentation", icon: "fermentation" },
  { key: "batches",      label: "Batches",      icon: "batches" },
  { key: "notes",        label: "Notes",        icon: "notes" },
];

function RecipeView({ recipes, onRecipesChange }) {
  const [selectedId, setSelectedId] = React.useState(recipes[0]?.id ?? null);
  const [activeTab, setActiveTab]   = React.useState("overview");

  const recipe = recipes.find((r) => r.id === selectedId);

  const handleRecipeChange = (updated) => {
    onRecipesChange(recipes.map((r) => (r.id === updated.id ? updated : r)));
  };

  const handleCreate = () => {
    const id = "r" + (recipes.length + 1) + "-" + Math.floor(Math.random() * 1000);
    const newRecipe = {
      id, name: "New Recipe", style: "American IPA", type: "all_grain",
      brewer: "", date: null,
      batch_size_gal: 5.0, boil_size_gal: 6.5, boil_time_min: 60, efficiency_pct: 75,
      notes: "",
      og: 1.000, fg: 1.000, abv: 0, ibu: 0, srm: 1, bu_gu: 0,
      cal_per_12oz: 0, pre_boil_vol: 0, post_boil_vol: 0, pre_boil_g: 1.000,
    };
    onRecipesChange([newRecipe, ...recipes]);
    setSelectedId(id);
    setActiveTab("overview");
  };

  return <>
    <window.RecipeList
      recipes={recipes}
      selectedId={selectedId}
      onSelect={setSelectedId}
      onCreate={handleCreate}
    />

    {recipe ? (
      <div className="kit-pane">
        <header className="pane-head">
          <button className="btn btn--secondary" style={{ padding: "3px 9px", fontSize: 11 }}
            onClick={() => setSelectedId(null)}>← Recipes</button>
          <input
            className="pane-title"
            value={recipe.name}
            onChange={(e) => handleRecipeChange({ ...recipe, name: e.target.value })}
          />
          <button className="btn btn--secondary" style={{ padding: "3px 9px", fontSize: 11 }}>Save Version</button>
          <button className="btn btn--secondary" style={{ padding: "3px 9px", fontSize: 11 }}>History (3)</button>
        </header>

        <window.TabBar tabs={RECIPE_TABS} active={activeTab} onChange={setActiveTab} />

        <div style={{ display: "flex", flex: 1, overflow: "hidden" }}>
          <div className="kit-scroll" style={{ padding: 16, flex: 1 }}>
            {activeTab === "overview"    && <window.OverviewTab    recipe={recipe} onChange={handleRecipeChange} />}
            {activeTab === "ingredients" && <window.IngredientsTab recipe={recipe} />}
            {!["overview","ingredients"].includes(activeTab) && (
              <div style={{ padding: 60, textAlign: "center", color: "var(--color-text-muted)" }}>
                <p style={{ fontSize: 14, margin: 0 }}>
                  The <strong style={{ color: "var(--color-text-secondary)", textTransform: "capitalize" }}>{activeTab}</strong> tab is part of the live Brewski app.
                </p>
                <p style={{ fontSize: 12, margin: "8px 0 0" }}>
                  See <code>src/lib/components/tabs/</code> in the repo for the full implementation.
                </p>
              </div>
            )}
          </div>
          <window.StatsSidebar recipe={recipe} />
        </div>
      </div>
    ) : (
      <div className="kit-pane" style={{ alignItems: "center", justifyContent: "center" }}>
        <p style={{ fontSize: 13, color: "var(--color-text-muted)" }}>Select a recipe to edit</p>
      </div>
    )}
  </>;
}

window.RecipeView = RecipeView;
