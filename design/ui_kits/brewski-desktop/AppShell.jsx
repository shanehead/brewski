// AppShell — left icon rail + theme switcher popover + view switcher.

const RAIL_TOP = [
  { key: "recipes",   icon: "recipes",   label: "Recipes" },
  { key: "batches",   icon: "batches",   label: "Batches" },
  { key: "tools",     icon: "tools",     label: "Tools" },
  { key: "equipment", icon: "equipment", label: "Equipment" },
  { key: "library",   icon: "library",   label: "Library" },
];

function ThemePopover({ current, onPick, onClose }) {
  React.useEffect(() => {
    const close = (e) => {
      if (!e.target.closest(".theme-pop") && !e.target.closest("[data-theme-trigger]")) {
        onClose();
      }
    };
    document.addEventListener("click", close);
    return () => document.removeEventListener("click", close);
  }, [onClose]);

  return (
    <div className="theme-pop">
      <div style={{ font: "var(--fw-semibold) var(--fs-xs) var(--font-sans)", letterSpacing: "var(--tracking-wider)", textTransform: "uppercase", color: "var(--color-text-muted)", padding: "6px 10px 4px" }}>
        Theme
      </div>
      {window.THEMES.map((t) => (
        <button
          key={t.id}
          className={current === t.id ? "active" : ""}
          onClick={() => { onPick(t.id); onClose(); }}
        >
          <span className="dot" style={{ background: t.accent }} />
          {t.name}
          <span style={{ marginLeft: "auto", fontSize: 10, color: "var(--color-text-muted)", textTransform: "uppercase", letterSpacing: "0.08em" }}>
            {t.scheme}
          </span>
        </button>
      ))}
    </div>
  );
}

function AppShell({ view, onViewChange, theme, onThemeChange, children }) {
  const [showTheme, setShowTheme] = React.useState(false);

  return (
    <div className="kit-app">
      <nav className="rail" style={{ position: "relative" }}>
        {RAIL_TOP.map((it) => (
          <button
            key={it.key}
            className={"rail-tile" + (view === it.key ? " active" : "")}
            title={it.label}
            aria-label={it.label}
            onClick={() => onViewChange(it.key)}
          >
            <window.BrewingIcon name={it.icon} size={22} />
          </button>
        ))}

        <div className="rail-spacer" />

        <button
          data-theme-trigger
          className={"rail-tile" + (showTheme ? " active" : "")}
          title="Theme"
          aria-label="Theme"
          onClick={() => setShowTheme((s) => !s)}
        >
          <window.PaletteIcon size={22} />
        </button>

        <button
          className={"rail-tile" + (view === "settings" ? " active" : "")}
          title="Settings"
          aria-label="Settings"
          onClick={() => onViewChange("settings")}
        >
          <window.BrewingIcon name="settings" size={22} />
        </button>

        {showTheme && (
          <ThemePopover
            current={theme}
            onPick={onThemeChange}
            onClose={() => setShowTheme(false)}
          />
        )}
      </nav>

      <div className="kit-main">{children}</div>
    </div>
  );
}

window.AppShell = AppShell;
