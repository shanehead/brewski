// TabBar — the recipe sub-tab bar (Overview, Ingredients, Mash, …).
// Brewing icon + text label, 2px accent underline on active.

function TabBar({ tabs, active, onChange }) {
  return (
    <div className="tabbar">
      {tabs.map((t) => (
        <button
          key={t.key}
          onClick={() => onChange(t.key)}
          className={"tab" + (active === t.key ? " active" : "")}
        >
          {t.icon && <window.BrewingIcon name={t.icon} size={18} />}
          {t.label}
        </button>
      ))}
    </div>
  );
}

window.TabBar = TabBar;
