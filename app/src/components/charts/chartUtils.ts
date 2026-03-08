const PLANET_ABBR: Record<string, string> = {
  Sun: "Su", Moon: "Mo", Mercury: "Me", Venus: "Ve", Mars: "Ma",
  Jupiter: "Ju", Saturn: "Sa", Rahu: "Ra", Ketu: "Ke",
};

export function formatDeg(deg?: number) {
  if (deg === undefined) return "";
  const d = ((deg % 360) + 360) % 360;
  return `${Math.floor(d % 30)}°`;
}

export { PLANET_ABBR };
