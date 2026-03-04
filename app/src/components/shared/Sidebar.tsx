import { useState } from "react";
import { Sparkles, LayoutDashboard, Activity, Star, Zap, Sun, Heart, Brain, Menu, X, Github } from "lucide-react";
import { cn } from "../../utils";
import type { TabId } from "../../types";

interface SidebarProps {
  activeTab: TabId;
  setActiveTab: (tab: TabId) => void;
  onTabHover?: (tab: TabId) => void;
  unknownTime?: boolean;
}

const TABS = [
  { id: "overview" as TabId, label: "대시보드", icon: LayoutDashboard },
  { id: "saju" as TabId, label: "사주 분석", icon: Activity },
  { id: "vedic_charts" as TabId, label: "베딕 차트 (D1-144)", icon: Star },
  { id: "strength" as TabId, label: "역량 및 기운", icon: Zap },
  { id: "transit" as TabId, label: "현재 운세", icon: Sun },
  { id: "compatibility" as TabId, label: "궁합 분석", icon: Heart },
  { id: "ai_audit" as TabId, label: "AI", icon: Brain },
];

export function Sidebar({ activeTab, setActiveTab, onTabHover, unknownTime }: SidebarProps) {
  const [mobileOpen, setMobileOpen] = useState(false);

  const handleSelectTab = (tabId: TabId) => {
    setActiveTab(tabId);
    setMobileOpen(false);
  };

  const filteredTabs = TABS.filter(tab => !(unknownTime && tab.id === "vedic_charts"));

  return (
    <>
      <button
        type="button"
        onClick={() => setMobileOpen(true)}
        className="md:hidden fixed top-4 left-4 z-40 rounded-xl border border-white/10 bg-black/40 backdrop-blur-md p-2 text-white"
        aria-label="메뉴 열기"
      >
        <Menu className="w-5 h-5" />
      </button>

      {mobileOpen && (
        <div className="md:hidden fixed inset-0 z-40 bg-black/60" onClick={() => setMobileOpen(false)} />
      )}

      <nav className={cn(
        "fixed md:static top-0 left-0 h-screen w-72 md:w-64 glass border-r border-white/10 flex flex-col p-6 z-50 transition-transform duration-300",
        mobileOpen ? "translate-x-0" : "-translate-x-full md:translate-x-0"
      )}>
        <div className="flex items-center gap-3 mb-10">
          <div className="w-10 h-10 bg-gradient-to-br from-celestial-purple to-celestial-cyan rounded-xl flex items-center justify-center shadow-lg shadow-indigo-500/20">
            <Sparkles className="w-6 h-6 text-white" />
          </div>
          <h1 className="text-2xl font-bold tracking-tight text-white">Eon.</h1>
          <button
            type="button"
            onClick={() => setMobileOpen(false)}
            className="ml-auto md:hidden rounded-lg p-1.5 hover:bg-white/10 text-white/70"
            aria-label="메뉴 닫기"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        <div className="space-y-2 flex-1">
          {filteredTabs.map((tab) => (
            <button
              key={tab.id}
              onClick={() => handleSelectTab(tab.id)}
              onMouseEnter={() => onTabHover?.(tab.id)}
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

        <div className="mt-auto pt-6 border-t border-white/5 flex flex-col gap-2">
          <p className="text-xs text-brand-400 font-medium tracking-wide">Eon. The Destiny Engine</p>
          <a
            href="https://github.com/sjkim1127"
            target="_blank"
            rel="noopener noreferrer"
            className="flex items-center gap-2 text-xs text-white/30 hover:text-white/80 transition-colors"
          >
            <Github className="w-3 h-3" />
            @sjkim1127
          </a>
        </div>
      </nav>

      <nav className="md:hidden fixed bottom-0 left-0 right-0 z-40 border-t border-white/10 bg-black/60 backdrop-blur-xl px-2 py-2">
        <div className={cn("grid gap-1", filteredTabs.length === 5 ? "grid-cols-5" : "grid-cols-6")}>
          {filteredTabs.map((tab) => (
            <button
              key={`mobile-${tab.id}`}
              type="button"
              onClick={() => handleSelectTab(tab.id)}
              onTouchStart={() => onTabHover?.(tab.id)}
              className={cn(
                "flex flex-col items-center justify-center gap-1 rounded-lg py-2 text-[10px]",
                activeTab === tab.id ? "text-white bg-white/10" : "text-white/50"
              )}
            >
              <tab.icon className="w-4 h-4" />
              <span className="leading-none">{tab.label.split(" ")[0]}</span>
            </button>
          ))}
        </div>
      </nav>
    </>
  );
}
