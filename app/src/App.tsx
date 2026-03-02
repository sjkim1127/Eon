import { useState } from "react";
import { get_vedic_analysis } from "./lib/api";
import { motion, AnimatePresence } from "framer-motion";
import {
  Sparkles,
  Sun,
  Star,
  Compass,
  Clock,
  Calendar,
  LayoutDashboard,
  Zap,
  Shield,
  Heart,
} from "lucide-react";
import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";

function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

interface VedicAnalysisReport {
  primary_karakas: {
    atmakaraka: string;
    amatyakaraka: string;
    darakaraka: string;
  };
  house_summary: Array<{
    house: number;
    rating: string;
    total_score: number;
  }>;
  dasha_focus: string;
  nakshatra_info: string;
  overall_strength_score: number;
  sade_sati: string;
}

function App() {
  const [birthData] = useState({
    year: 1990,
    month: 1,
    day: 1,
    hour: 12,
    minute: 0,
    lat: 37.5665,
    lon: 126.978,
  });

  const [report, setReport] = useState<VedicAnalysisReport | null>(null);
  const [loading, setLoading] = useState(false);
  const [activeTab, setActiveTab] = useState("overview");

  const runAnalysis = async () => {
    setLoading(true);
    try {
      const res = await get_vedic_analysis({
        ...birthData,
      });
      setReport(res);
    } catch (e) {
      console.error(e);
    } finally {
      setLoading(false);
    }
  };

  const ShootingStars = () => {
    return (
      <div className="fixed inset-0 pointer-events-none overflow-hidden z-0">
        {[...Array(5)].map((_, i) => (
          <div
            key={i}
            className="shooting-star"
            style={{
              top: `${Math.random() * 50}%`,
              left: `${Math.random() * 50}%`,
              animationDelay: `${Math.random() * 10}s`,
            }}
          />
        ))}
      </div>
    );
  };

  return (
    <div className="min-h-screen relative flex">
      <ShootingStars />

      {/* Sidebar */}
      <nav className="w-64 glass border-r border-white/10 flex flex-col p-6 z-10">
        <div className="flex items-center gap-3 mb-10">
          <div className="w-10 h-10 bg-gradient-to-br from-celestial-purple to-celestial-cyan rounded-xl flex items-center justify-center shadow-lg shadow-indigo-500/20">
            <Sparkles className="w-6 h-6 text-white" />
          </div>
          <h1 className="text-2xl font-bold tracking-tight text-white">Eon.</h1>
        </div>

        <div className="space-y-2 flex-1">
          {[
            { id: "overview", label: "Dashboard", icon: LayoutDashboard },
            { id: "strength", label: "Strength", icon: Zap },
            { id: "transit", label: "Transits", icon: Sun },
          ].map((tab) => (
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

      {/* Main Content */}
      <main className="flex-1 p-10 overflow-y-auto z-10">
        <header className="flex justify-between items-end mb-12">
          <div>
            <h2 className="text-4xl font-bold text-white mb-2">Celestial Insight</h2>
            <p className="text-brand-400">Computational Vedic Astrology Engine</p>
          </div>

          <div className="flex gap-4">
            <div className="glass px-6 py-4 rounded-2xl flex items-center gap-4">
              <Calendar className="w-5 h-5 text-celestial-purple" />
              <div className="text-sm">
                <p className="text-white/40 leading-none mb-1">Birth Date</p>
                <p className="text-white font-semibold">
                  {birthData.year}.{birthData.month}.{birthData.day}
                </p>
              </div>
            </div>
            <button
              onClick={runAnalysis}
              disabled={loading}
              className="bg-gradient-to-r from-celestial-purple to-brand-600 px-8 py-4 rounded-2xl font-bold text-white shadow-xl shadow-indigo-500/20 hover:scale-105 active:scale-95 transition-all disabled:opacity-50"
            >
              {loading ? "Calculating..." : "Update Chart"}
            </button>
          </div>
        </header>

        <AnimatePresence mode="wait">
          {!report ? (
            <motion.div
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              className="h-[60vh] flex flex-col items-center justify-center text-center"
            >
              <div className="w-24 h-24 rounded-full bg-white/5 flex items-center justify-center mb-6">
                <Compass className="w-12 h-12 text-white/20 animate-pulse" />
              </div>
              <h3 className="text-2xl font-semibold text-white mb-2">No Active Chart</h3>
              <p className="text-brand-400 max-w-sm">
                Synchronize with the heavens by clicking the update button to generate your
                personalized Vedic analysis.
              </p>
            </motion.div>
          ) : (
            <motion.div
              key="results"
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              exit={{ opacity: 0, y: -20 }}
              className="space-y-8"
            >
              {/* Hero Statistics */}
              <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                <div className="glass p-8 rounded-[2rem] relative overflow-hidden group">
                  <div className="absolute top-0 right-0 p-8 transform translate-x-4 -translate-y-4 opacity-5 group-hover:translate-x-0 group-hover:translate-y-0 transition-all duration-500">
                    <Heart className="w-32 h-32" />
                  </div>
                  <p className="text-brand-400 text-sm font-bold uppercase tracking-wider mb-2">
                    Soul Indicator
                  </p>
                  <h4 className="text-3xl font-bold text-white mb-4">
                    {report.primary_karakas.atmakaraka}
                  </h4>
                  <p className="text-sm text-white/60 leading-relaxed">
                    Atmakaraka - The indicator of the soul's primary desires and lessons in this
                    incarnation.
                  </p>
                </div>

                <div className="glass p-8 rounded-[2rem] border-celestial-purple/20 bg-celestial-purple/5">
                  <p className="text-celestial-purple/80 text-sm font-bold uppercase tracking-wider mb-2">
                    Current Dasha
                  </p>
                  <h4 className="text-3xl font-bold text-white mb-4">
                    {report.dasha_focus.replace("Current Major Period: ", "")}
                  </h4>
                  <div className="flex items-center gap-2 text-sm text-white/60">
                    <Clock className="w-4 h-4" />
                    <span>Primary influence at this stage of life.</span>
                  </div>
                </div>

                <div className="glass p-8 rounded-[2rem]">
                  <p className="text-brand-400 text-sm font-bold uppercase tracking-wider mb-2">
                    Chart Strength
                  </p>
                  <div className="flex items-baseline gap-2 mb-4">
                    <h4 className="text-5xl font-black text-gradient leading-none">
                      {Math.round(report.overall_strength_score)}
                    </h4>
                    <span className="text-white/20 font-bold">/ 600</span>
                  </div>
                  <div className="w-full bg-white/5 h-2 rounded-full overflow-hidden">
                    <div
                      className="bg-celestial-purple h-full rounded-full transition-all duration-1000"
                      style={{ width: `${(report.overall_strength_score / 600) * 100}%` }}
                    />
                  </div>
                </div>
              </div>

              {/* Secondary Info */}
              <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
                <div className="glass p-10 rounded-[2.5rem]">
                  <h5 className="text-xl font-bold text-white mb-8 flex items-center gap-3">
                    <Star className="w-6 h-6 text-celestial-gold" />
                    Nakshatra Blueprint
                  </h5>
                  <div className="p-6 bg-white/5 rounded-2xl border border-white/5">
                    <p className="text-white text-lg font-medium leading-relaxed">
                      {report.nakshatra_info}
                    </p>
                  </div>
                </div>

                <div className="glass p-10 rounded-[2.5rem]">
                  <h5 className="text-xl font-bold text-white mb-8 flex items-center gap-3">
                    <Shield className="w-6 h-6 text-celestial-cyan" />
                    Transit Alert
                  </h5>
                  <div className="p-6 bg-white/5 rounded-2xl border border-white/5">
                    <p className="text-white text-lg font-medium leading-relaxed">
                      {report.sade_sati}
                    </p>
                  </div>
                </div>
              </div>

              {/* House Grid */}
              <section>
                <h5 className="text-xl font-bold text-white mb-6">Bhava (House) Capacities</h5>
                <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-4">
                  {report.house_summary.map((house: any) => (
                    <div
                      key={house.house}
                      className="glass p-6 rounded-2xl text-center glass-hover cursor-help"
                    >
                      <p className="text-xs text-white/30 font-bold mb-1">BASE {house.house}</p>
                      <p className="text-2xl font-bold text-white mb-2">
                        {Math.round(house.total_score)}
                      </p>
                      <span
                        className={cn(
                          "px-2 py-0.5 rounded text-[10px] font-black uppercase",
                          house.rating === "Excellent"
                            ? "bg-green-500/20 text-green-400"
                            : house.rating === "Strong"
                              ? "bg-blue-500/20 text-blue-400"
                              : house.rating === "Average"
                                ? "bg-yellow-500/20 text-yellow-400"
                                : "bg-red-500/20 text-red-400"
                        )}
                      >
                        {house.rating}
                      </span>
                    </div>
                  ))}
                </div>
              </section>
            </motion.div>
          )}
        </AnimatePresence>
      </main>
    </div>
  );
}

export default App;
