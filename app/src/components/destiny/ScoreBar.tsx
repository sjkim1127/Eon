export function ScoreBar({ label, value, max, color }: { label: string; value: number; max: number; color: string }) {
  const pct = Math.min(100, (value / max) * 100);
  return (
    <div className="flex items-center gap-3">
      <span className="text-xs text-white/50 w-36 shrink-0">{label}</span>
      <div className="flex-1 bg-white/10 h-2 rounded-full overflow-hidden">
        <div className={`h-full rounded-full ${color} transition-all duration-500`} style={{ width: `${pct}%` }} />
      </div>
      <span className="text-xs text-white/70 w-8 text-right shrink-0">{Math.round(value)}</span>
    </div>
  );
}
