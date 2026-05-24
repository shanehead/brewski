// RecipeList — left sidebar listing all recipes.
// Search, new-recipe button, BeerXML import, and the list itself.

function RecipeList({ recipes, selectedId, onSelect, onCreate }) {
  const [search, setSearch] = React.useState("");
  const filtered = search.trim()
    ? recipes.filter((r) => r.name.toLowerCase().includes(search.toLowerCase()))
    : recipes;

  return (
    <aside className="sidebar">
      <div className="sidebar-head">
        <div className="search-wrap">
          <window.SearchIcon />
          <input
            type="search"
            placeholder="Search recipes…"
            value={search}
            onChange={(e) => setSearch(e.target.value)}
          />
        </div>
        <button className="btn btn--primary" onClick={onCreate}>
          + New Recipe
        </button>
        <button className="btn btn--outline">Import BeerXML</button>
      </div>

      <ul className="sidebar-list" style={{ listStyle: "none", margin: 0, padding: 0 }}>
        {filtered.map((r) => (
          <li key={r.id}>
            <div
              className={"list-row" + (selectedId === r.id ? " selected" : "")}
              onClick={() => onSelect(r.id)}
            >
              <span className="nm">{r.name}</span>
              <span className="sub">{r.style} · {r.batch_size_gal.toFixed(1)} gal</span>
            </div>
          </li>
        ))}
        {filtered.length === 0 && (
          <li style={{ padding: "24px 12px", textAlign: "center", fontSize: 13, color: "var(--color-text-muted)" }}>
            {search ? "No matches" : "No recipes yet"}
          </li>
        )}
      </ul>
    </aside>
  );
}

window.RecipeList = RecipeList;
