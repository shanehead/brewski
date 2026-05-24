// BrewingIcon — renders a Brewski multi-color SVG icon by name.
// Icons live in icons.js as `window.BREWSKI_ICONS = { name: '<paths>' }`.
//
// Usage:  <BrewingIcon name="hop" size={22} />

function BrewingIcon({ name, size = 18, style }) {
  const body = (window.BREWSKI_ICONS && window.BREWSKI_ICONS[name]) || "";
  return (
    <svg
      aria-hidden="true"
      data-icon={name}
      width={size}
      height={size}
      viewBox="0 0 24 24"
      style={style}
      dangerouslySetInnerHTML={{ __html: body }}
    />
  );
}

// Feather-style stroke icons used in chrome (search magnifier, chevron).
function StrokeIcon({ d, size = 14, strokeWidth = 2 }) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth={strokeWidth}
      strokeLinecap="round"
      strokeLinejoin="round"
      aria-hidden="true"
    >
      {d}
    </svg>
  );
}

const SearchIcon = ({ size = 13 }) => (
  <StrokeIcon size={size} strokeWidth={2.5} d={<>
    <circle cx="11" cy="11" r="8" />
    <line x1="21" y1="21" x2="16.65" y2="16.65" />
  </>} />
);

const ChevronRight = ({ size = 14 }) => (
  <StrokeIcon size={size} d={<polyline points="9 18 15 12 9 6" />} />
);

const PaletteIcon = ({ size = 22 }) => (
  <svg width={size} height={size} viewBox="0 0 24 24" aria-hidden="true">
    <circle cx="6.5" cy="11.5" r="2" fill="#5c5cff" />
    <circle cx="10" cy="6"    r="2" fill="#bd93f9" />
    <circle cx="15.5" cy="6.5" r="2" fill="#88c0d0" />
    <circle cx="18" cy="11.5" r="2" fill="#10b981" />
    <path d="M12 2C6.5 2 2 6.3 2 11.6 2 16.4 5.7 20 10.3 20c1.5 0 2-1.2 1.5-2-0.6-1 0-2.4 1.4-2.4h1.6c4 0 7.2-2.4 7.2-6 0-4.2-4.4-7.6-10-7.6Z" fill="none" stroke="#a0a0b4" strokeWidth="1.5" />
  </svg>
);

window.BrewingIcon = BrewingIcon;
window.StrokeIcon  = StrokeIcon;
window.SearchIcon  = SearchIcon;
window.ChevronRight = ChevronRight;
window.PaletteIcon = PaletteIcon;
