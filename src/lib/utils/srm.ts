// SRM beer color scale — represents color of beer in glass, per ASBC standard
const SRM_STOPS: [number, string][] = [
  [1, "#FFE699"], [2, "#FFD878"], [3, "#FFCA5A"], [4, "#FFBF42"],
  [6, "#FBB123"], [8, "#F8A600"], [10, "#F39C00"], [13, "#EA8F00"],
  [17, "#D77200"], [20, "#CF6900"], [24, "#BB5100"], [29, "#A13600"],
  [35, "#8D1D00"], [40, "#611200"],
];

export function srmToHex(srm: number): string {
  const clamp = Math.min(Math.max(srm, 1), 40);
  for (let i = SRM_STOPS.length - 1; i >= 0; i--) {
    if (clamp >= SRM_STOPS[i][0]) return SRM_STOPS[i][1];
  }
  return SRM_STOPS[0][1];
}
