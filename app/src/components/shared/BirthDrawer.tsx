import { useEffect } from "react";
import { AnimatePresence, motion } from "framer-motion";
import { X, Sparkles } from "lucide-react";
import { BirthInputForm } from "./BirthInputForm";
import type { BirthData } from "../../types";

interface BirthDrawerProps {
  open: boolean;
  onClose: () => void;
  birthData: BirthData;
  setBirthData: React.Dispatch<React.SetStateAction<BirthData>>;
  selectedCity: string;
  onCitySelect: (city: { name: string; lat: number; lon: number; timezone: string }) => void;
  isMale: boolean;
  setIsMale: (v: boolean) => void;
  isDST: boolean;
  loading: boolean;
  onAnalysis: () => void;
  sajuReport: import("../../types").SajuAnalysisResult | null;
}

export function BirthDrawer({ open, onClose, ...formProps }: BirthDrawerProps) {
  // Esc key to close
  useEffect(() => {
    if (!open) return;
    const onKey = (e: KeyboardEvent) => { if (e.key === "Escape") onClose(); };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [open, onClose]);

  return (
    <AnimatePresence>
      {open && (
        <>
          {/* Backdrop */}
          <motion.div
            key="drawer-backdrop"
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            transition={{ duration: 0.22 }}
            onClick={onClose}
            className="fixed inset-0 z-40 bg-black/55 backdrop-blur-sm"
          />

          {/* Drawer Panel */}
          <motion.aside
            key="drawer-panel"
            initial={{ x: "100%" }}
            animate={{ x: 0 }}
            exit={{ x: "100%" }}
            transition={{ type: "spring", damping: 30, stiffness: 290 }}
            className="fixed top-0 right-0 h-full w-full max-w-[500px] z-50 flex flex-col"
            style={{ background: "rgba(18, 22, 35, 0.97)", borderLeft: "1px solid rgba(255,255,255,0.07)" }}
          >
            {/* Drawer header */}
            <div
              className="flex items-center gap-3 px-6 py-5 shrink-0"
              style={{ borderBottom: "1px solid rgba(255,255,255,0.07)" }}
            >
              <div className="w-9 h-9 rounded-xl bg-gradient-to-br from-celestial-purple to-celestial-cyan flex items-center justify-center shadow-lg shadow-indigo-500/20">
                <Sparkles className="w-4 h-4 text-white" />
              </div>
              <div>
                <p className="text-sm font-bold text-white leading-tight">출생 정보</p>
                <p className="text-[11px] text-white/40 leading-tight mt-0.5">생년월일시 · 출생지 · 성별</p>
              </div>
              <button
                onClick={onClose}
                aria-label="닫기"
                className="ml-auto w-8 h-8 flex items-center justify-center rounded-lg text-white/40 hover:text-white hover:bg-white/10 transition-all"
              >
                <X className="w-4 h-4" />
              </button>
            </div>

            {/* Form body */}
            <div className="flex-1 overflow-y-auto px-6 py-6">
              <BirthInputForm {...formProps} compact onClose={onClose} />
            </div>
          </motion.aside>
        </>
      )}
    </AnimatePresence>
  );
}
