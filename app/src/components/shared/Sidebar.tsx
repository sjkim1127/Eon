import { Sparkles, LayoutDashboard, Activity, Star, Zap, Sun, Heart } from "lucide-react";
import { cn } from "../../utils";
import type { TabId } from "../../types";

interface SidebarProps {
  activeTab: TabId;
  setActiveTab: (tab: TabId) => void;
}

const TABS = [
  { id: "overview" as TabId, label: "대시보드", icon: LayoutDashboard },
  { id: "saju" as TabId, label: "사주 분석", icon: Activity },
  { id: "vedic_charts" as TabId, label: "베딕 차트 (D1-144)", icon: Star },
  { id: "strength" as TabId, label: "역량 및 기운", icon: Zap },
  { id: "transit" as TabId, label: "현재 운세", icon: Sun },
  { id: "compatibility" as TabId, label: "궁합 분석", icon: Heart },
];

export function Sidebar({ activeTab, setActiveTab }: SidebarProps) {
  return (
    <nav className="w-64 glass border-r border-white/10 flex flex-col p-6 z-10">
      <div className="flex items-center gap-3 mb-10">
        <div className="w-10 h-10 bg-gradient-to-br from-celestial-purple to-celestial-cyan rounded-xl flex items-center justify-center shadow-lg shadow-indigo-500/20">
          <Sparkles className="w-6 h-6 text-white" />
        </div>
        <h1 className="text-2xl font-bold tracking-tight text-white">Eon.</h1>
      </div>

      <div className="space-y-2 flex-1">
        {TABS.map((tab) => (
          <button
            key={tab.id}
            onClick={() => setActiveTab(tab.id)}
            className={cn(
              "w-full flex items-center gap-3 px-4 py-3 rounded-xl transition-all duration-200",
              activeTab === tab.id
                ? "bg-white/10 text-white"
                : "text-white/50 hover:text-white hover:bg-white/5"
            )}
          >
            <tab.icon className="w-5 h-5" />
            <span className="font-medium">{tab.label}</span>
          </button>
        ))}
      </div>

      <div className="mt-auto pt-6 border-t border-white/5">
        <p className="text-xs text-brand-400 font-medium">BPHS COMPLIANT v0.1.0</p>
      </div>
    </nav>
  );
}
