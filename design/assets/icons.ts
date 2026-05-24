export type BrewingIconName =
  | "fermentable"
  | "hop"
  | "yeast"
  | "overview"
  | "ingredients"
  | "mash"
  | "water"
  | "fermentation"
  | "notes"
  | "batches"
  | "recipes"
  | "tools"
  | "equipment"
  | "library"
  | "settings";

export const ICONS: Record<BrewingIconName, string> = {
  recipes: `
    <path d="M4 4C4 3 5 2 6 2H12V12L9 10.5L6 12V2" fill="#3b82f6"/>
    <path d="M12 2H18C19 2 20 3 20 4V20C20 21 19 22 18 22H6C5 22 4 21 4 20V12" fill="#2563eb"/>
    <path d="M12 2V22" fill="#1d4ed8" opacity="0.4"/>
    <rect x="8" y="14" width="8" height="1.5" rx="0.75" fill="white" opacity="0.5"/>
    <rect x="8" y="17" width="5" height="1.5" rx="0.75" fill="white" opacity="0.5"/>
  `,
  batches: `
    <ellipse cx="12" cy="6" rx="7" ry="2.5" fill="#10b981"/>
    <rect x="5" y="6" width="14" height="4" fill="#059669"/>
    <ellipse cx="12" cy="10" rx="7" ry="2.5" fill="#10b981"/>
    <rect x="5" y="10" width="14" height="5" fill="#047857"/>
    <ellipse cx="12" cy="15" rx="7" ry="2.5" fill="#10b981"/>
    <rect x="5" y="15" width="14" height="3" fill="#065f46"/>
    <ellipse cx="12" cy="18" rx="7" ry="2.5" fill="#10b981"/>
    <line x1="12" y1="6" x2="12" y2="18" stroke="#6ee7b7" stroke-width="1" opacity="0.5"/>
  `,
  tools: `
    <path d="M14.5 2.5C11.5 2.5 9 5 9 8C9 8.8 9.2 9.5 9.5 10.2L3 17C2.4 17.6 2.4 18.6 3 19.2L4.8 21C5.4 21.6 6.4 21.6 7 21L13.8 14.5C14.5 14.8 15.2 15 16 15C19 15 21.5 12.5 21.5 9.5C21.5 8.9 21.4 8.3 21.2 7.8L18 11L16 9L19.2 5.8C18.7 5.6 18.1 5.5 17.5 5.5" fill="#f59e0b"/>
    <path d="M3 17L7 21L9.5 18.5L5.5 14.5Z" fill="#d97706"/>
    <circle cx="5.5" cy="18.5" r="1.2" fill="#fcd34d"/>
  `,
  equipment: `
    <path d="M3 9C1 9 1 12 3 12" stroke="#8b5cf6" stroke-width="2" fill="none" stroke-linecap="round"/>
    <path d="M21 9C23 9 23 12 21 12" stroke="#8b5cf6" stroke-width="2" fill="none" stroke-linecap="round"/>
    <rect x="4" y="7" width="16" height="11" rx="3" fill="#7c3aed"/>
    <rect x="4" y="7" width="16" height="5" rx="3" fill="#8b5cf6"/>
    <rect x="6" y="4" width="12" height="4" rx="2" fill="#a78bfa"/>
    <rect x="10" y="2" width="4" height="3" rx="1.5" fill="#c4b5fd"/>
    <rect x="6" y="9" width="4" height="2" rx="1" fill="white" opacity="0.2"/>
  `,
  library: `
    <rect x="3" y="4" width="5" height="16" rx="1.5" fill="#ec4899"/>
    <rect x="3" y="4" width="2" height="16" rx="1" fill="#be185d"/>
    <rect x="9" y="6" width="4" height="14" rx="1.5" fill="#f97316"/>
    <rect x="9" y="6" width="1.5" height="14" rx="0.75" fill="#c2410c"/>
    <rect x="14.5" y="3" width="6" height="17" rx="1.5" fill="#06b6d4"/>
    <rect x="14.5" y="3" width="2.5" height="17" rx="1" fill="#0e7490"/>
  `,
  settings: `
    <circle cx="12" cy="12" r="10" fill="#64748b"/>
    <circle cx="12" cy="12" r="7" fill="#475569"/>
    <rect x="10.5" y="2" width="3" height="3" rx="0.5" fill="#64748b"/>
    <rect x="10.5" y="19" width="3" height="3" rx="0.5" fill="#64748b"/>
    <rect x="2" y="10.5" width="3" height="3" rx="0.5" fill="#64748b"/>
    <rect x="19" y="10.5" width="3" height="3" rx="0.5" fill="#64748b"/>
    <rect x="4.5" y="4.5" width="2.5" height="2.5" rx="0.5" fill="#64748b" transform="rotate(45 5.75 5.75)"/>
    <rect x="17" y="4.5" width="2.5" height="2.5" rx="0.5" fill="#64748b" transform="rotate(45 18.25 5.75)"/>
    <rect x="4.5" y="17" width="2.5" height="2.5" rx="0.5" fill="#64748b" transform="rotate(45 5.75 18.25)"/>
    <rect x="17" y="17" width="2.5" height="2.5" rx="0.5" fill="#64748b" transform="rotate(45 18.25 18.25)"/>
    <circle cx="12" cy="12" r="4" fill="#94a3b8"/>
    <circle cx="12" cy="12" r="2.5" fill="#334155"/>
  `,
  overview: `
    <rect x="5" y="3" width="14" height="18" rx="2" fill="#3b82f6"/>
    <rect x="5" y="3" width="14" height="9" rx="2" fill="#2563eb"/>
    <rect x="9" y="1" width="6" height="4" rx="2" fill="#93c5fd"/>
    <rect x="7" y="10" width="10" height="1.5" rx="0.75" fill="white" opacity="0.6"/>
    <rect x="7" y="13" width="7" height="1.5" rx="0.75" fill="white" opacity="0.6"/>
    <rect x="7" y="16" width="8" height="1.5" rx="0.75" fill="white" opacity="0.6"/>
  `,
  ingredients: `
    <circle cx="8" cy="12" r="6" fill="#84cc16"/>
    <path d="M8 6C5 6 2 9 2 12C2 15 5 18 8 18" fill="#65a30d"/>
    <path d="M6 12L8 9L10 12L8 15Z" fill="white" opacity="0.8"/>
    <line x1="16" y1="20" x2="16" y2="8" stroke="#d97706" stroke-width="2"/>
    <ellipse cx="16" cy="8" rx="2.5" ry="4" fill="#f59e0b"/>
    <ellipse cx="13.5" cy="11" rx="2" ry="3" fill="#fbbf24" transform="rotate(-20 13.5 11)"/>
    <ellipse cx="18.5" cy="11" rx="2" ry="3" fill="#f59e0b" transform="rotate(20 18.5 11)"/>
  `,
  mash: `
    <rect x="10" y="3" width="4" height="13" rx="2" fill="#fed7aa"/>
    <rect x="10.5" y="3" width="3" height="10" rx="1.5" fill="#f97316" opacity="0.3"/>
    <rect x="11" y="9" width="2" height="7" fill="#f97316"/>
    <circle cx="12" cy="17" r="4" fill="#f97316"/>
    <circle cx="12" cy="17" r="2.5" fill="#ea580c"/>
    <rect x="14" y="7" width="2" height="1" rx="0.5" fill="#fb923c"/>
    <rect x="14" y="10" width="2" height="1" rx="0.5" fill="#fb923c"/>
  `,
  water: `
    <path d="M12 3C12 3 4 12 4 16C4 20 7.6 23 12 23C16.4 23 20 20 20 16C20 12 12 3 12 3Z" fill="#38bdf8"/>
    <path d="M12 3C12 3 20 12 20 16C20 20 16.4 23 12 23L12 3Z" fill="#0284c7"/>
    <path d="M8.5 16C8.5 13.5 10 12 12 11.5" stroke="white" stroke-width="1.5" stroke-linecap="round" fill="none" opacity="0.7"/>
  `,
  fermentation: `
    <path d="M7 8L5 19C5 20.1 5.9 21 7 21H17C18.1 21 19 20.1 19 19L17 8Z" fill="#8b5cf6"/>
    <path d="M17 8L19 19C19 20.1 18.1 21 17 21H12L12 8Z" fill="#7c3aed"/>
    <rect x="6" y="6" width="12" height="3" rx="1.5" fill="#a78bfa"/>
    <rect x="11" y="2" width="2" height="5" rx="1" fill="#c4b5fd"/>
    <rect x="9" y="1" width="6" height="2.5" rx="1.25" fill="#a78bfa"/>
    <circle cx="9" cy="14" r="1.5" fill="#c4b5fd" opacity="0.6"/>
    <circle cx="14" cy="12" r="1" fill="#c4b5fd" opacity="0.5"/>
    <circle cx="11" cy="17" r="1" fill="#c4b5fd" opacity="0.4"/>
  `,
  notes: `
    <rect x="4" y="3" width="16" height="18" rx="2" fill="#fbbf24"/>
    <rect x="4" y="3" width="16" height="9" rx="2" fill="#f59e0b"/>
    <rect x="7" y="9" width="10" height="1.5" rx="0.75" fill="white" opacity="0.7"/>
    <rect x="7" y="12" width="10" height="1.5" rx="0.75" fill="white" opacity="0.7"/>
    <rect x="7" y="15" width="7" height="1.5" rx="0.75" fill="white" opacity="0.7"/>
    <path d="M14 4L18 8L16 10L12 6Z" fill="#d97706"/>
    <path d="M12 6L11 9L14 8Z" fill="#92400e"/>
  `,
  fermentable: `
    <line x1="12" y1="22" x2="12" y2="8" stroke="#d97706" stroke-width="2"/>
    <ellipse cx="12" cy="7" rx="3" ry="5" fill="#f59e0b"/>
    <ellipse cx="8.5" cy="11" rx="2.5" ry="4" fill="#fbbf24" transform="rotate(-25 8.5 11)"/>
    <ellipse cx="15.5" cy="11" rx="2.5" ry="4" fill="#f59e0b" transform="rotate(25 15.5 11)"/>
    <ellipse cx="6" cy="15" rx="2" ry="3" fill="#fcd34d" transform="rotate(-30 6 15)"/>
    <ellipse cx="18" cy="15" rx="2" ry="3" fill="#fbbf24" transform="rotate(30 18 15)"/>
  `,
  hop: `
    <path d="M12 4C9 4 6 7 6 12C6 17 9 20 12 20C15 20 18 17 18 12C18 7 15 4 12 4Z" fill="#22c55e"/>
    <path d="M12 4C15 4 18 7 18 12C18 17 15 20 12 20L12 4Z" fill="#16a34a"/>
    <path d="M9 9L12 6L15 9L12 18Z" fill="#bbf7d0" opacity="0.7"/>
    <path d="M7 12C5 10 5 8 7 7" stroke="#4ade80" stroke-width="1.5" fill="none" stroke-linecap="round"/>
    <path d="M17 12C19 10 19 8 17 7" stroke="#4ade80" stroke-width="1.5" fill="none" stroke-linecap="round"/>
  `,
  yeast: `
    <path d="M9 3L15 3L15 10L20 19C20 20.1 19.1 21 18 21L6 21C4.9 21 4 20.1 4 19L9 10Z" fill="#14b8a6"/>
    <path d="M9 3L15 3L15 10L20 19C20 20.1 19.1 21 18 21L12 21L12 10L15 3Z" fill="#0d9488"/>
    <rect x="8" y="2" width="8" height="2" rx="1" fill="#5eead4"/>
    <circle cx="9" cy="16" r="2" fill="white" opacity="0.6"/>
    <circle cx="14" cy="14" r="1.5" fill="white" opacity="0.45"/>
    <circle cx="11" cy="18" r="1" fill="white" opacity="0.35"/>
  `,
};
