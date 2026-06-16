import { useState, useCallback } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { Bot, Sparkles, Key, Play, ChevronDown, ChevronUp, AlertCircle, CheckCircle2, Loader2, Shield } from "lucide-react";
import type { SajuAnalysisResult } from "../../../types/saju";
import type { AiAuditState } from "../../../types/ai";

interface AiAuditSectionProps {
    sajuReport: SajuAnalysisResult;
    birthYear: number;
    birthMonth: number;
    birthDay: number;
    birthHour: number;
    isMale: boolean;
}

// ── 마크다운 기본 렌더러 (외부 라이브러리 없이) ──────────────────────────────
function renderMarkdown(text: string): React.ReactNode[] {
    const lines = text.split("\n");
    const elements: React.ReactNode[] = [];
    let keyIdx = 0;

    for (const line of lines) {
        const key = keyIdx++;
        if (line.startsWith("## ")) {
            elements.push(
                <h2 key={key} className="text-lg font-bold text-white mt-6 mb-2 flex items-center gap-2">
                    {line.replace("## ", "")}
                </h2>
            );
        } else if (line.startsWith("### ")) {
            elements.push(
                <h3 key={key} className="text-sm font-semibold text-white/80 mt-4 mb-1.5 border-l-2 border-purple-400 pl-3">
                    {line.replace("### ", "")}
                </h3>
            );
        } else if (line.startsWith("- ")) {
            elements.push(
                <li key={key} className="text-sm text-white/70 ml-4 list-disc mb-1">
                    {line.replace("- ", "")}
                </li>
            );
        } else if (line.startsWith("**") && line.endsWith("**")) {
            elements.push(
                <p key={key} className="text-sm font-semibold text-purple-300 mb-1">
                    {line.replace(/\*\*/g, "")}
                </p>
            );
        } else if (line.trim() === "---") {
            elements.push(<hr key={key} className="border-white/10 my-3" />);
        } else if (line.trim() !== "") {
            // 인라인 볼드 처리
            const parts = line.split(/\*\*(.*?)\*\*/g);
            elements.push(
                <p key={key} className="text-sm text-white/65 leading-relaxed mb-1">
                    {parts.map((part, i) =>
                        i % 2 === 1
                            ? <strong key={i} className="text-white/85 font-semibold">{part}</strong>
                            : part
                    )}
                </p>
            );
        } else {
            elements.push(<div key={key} className="h-1" />);
        }
    }
    return elements;
}

// ── API 키 입력 패널 ──────────────────────────────────────────────────────────
function ApiKeyPanel({
    apiKey,
    onApiKeyChange,
    onRun,
    isLoading,
}: {
    apiKey: string;
    onApiKeyChange: (key: string) => void;
    onRun: () => void;
    isLoading: boolean;
}) {
    const [showKey, setShowKey] = useState(false);

    return (
        <div className="space-y-4">
            {/* API Key 설명 */}
            <div className="flex items-start gap-3 p-3 rounded-xl bg-purple-500/5 border border-purple-500/15">
                <Key className="w-4 h-4 text-purple-400 mt-0.5 flex-shrink-0" />
                <div className="text-xs text-white/60 leading-relaxed">
                    <span className="text-purple-300 font-semibold">Gemini API 키</span>가 필요합니다.
                    {" "}<a
                        href="https://aistudio.google.com/app/api-keys"
                        target="_blank"
                        rel="noopener noreferrer"
                        className="text-purple-400 underline hover:text-purple-300 transition-colors"
                    >
                        Google AI Studio
                    </a>에서 무료로 발급받을 수 있습니다.
                    키는 서버에 저장되지 않으며, 요청 시에만 사용됩니다.
                </div>
            </div>

            {/* 입력 필드 */}
            <div className="flex gap-2">
                <div className="relative flex-1">
                    <input
                        id="ai-audit-api-key"
                        type={showKey ? "text" : "password"}
                        value={apiKey}
                        onChange={e => onApiKeyChange(e.target.value)}
                        placeholder="AIza..."
                        className="w-full bg-white/5 border border-white/10 rounded-xl px-4 py-2.5 text-sm text-white/80 placeholder-white/25 focus:outline-none focus:border-purple-500/50 focus:bg-purple-500/5 transition-all pr-10"
                    />
                    <button
                        onClick={() => setShowKey(v => !v)}
                        className="absolute right-3 top-1/2 -translate-y-1/2 text-white/30 hover:text-white/60 transition-colors"
                        aria-label={showKey ? "키 숨기기" : "키 보기"}
                    >
                        {showKey ? "🙈" : "👁️"}
                    </button>
                </div>

                <button
                    id="ai-audit-run-btn"
                    onClick={onRun}
                    disabled={!apiKey.trim() || isLoading}
                    className="flex items-center gap-2 px-5 py-2.5 rounded-xl bg-gradient-to-r from-purple-600 to-violet-600 text-white text-sm font-semibold disabled:opacity-40 disabled:cursor-not-allowed hover:from-purple-500 hover:to-violet-500 transition-all shadow-lg shadow-purple-900/30 active:scale-95"
                >
                    {isLoading ? (
                        <Loader2 className="w-4 h-4 animate-spin" />
                    ) : (
                        <Play className="w-4 h-4" />
                    )}
                    {isLoading ? "분석 중..." : "감사 실행"}
                </button>
            </div>
        </div>
    );
}

// ── 로딩 애니메이션 ───────────────────────────────────────────────────────────
function LoadingState() {
    const steps = [
        "엔트로피 분석 중...",
        "오행 위상 스캔 중...",
        "취약점 퍼징 중...",
        "근본 원인 역추적 중...",
        "리포트 작성 중...",
    ];
    const [stepIdx, setStepIdx] = useState(0);

    // 3초마다 단계 전환
    useState(() => {
        const interval = setInterval(() => {
            setStepIdx(i => (i + 1) % steps.length);
        }, 3000);
        return () => clearInterval(interval);
    });

    return (
        <div className="flex flex-col items-center justify-center py-12 gap-6">
            {/* 애니메이션 아이콘 */}
            <div className="relative">
                <div className="w-16 h-16 rounded-full bg-purple-500/10 border border-purple-500/20 flex items-center justify-center">
                    <Bot className="w-8 h-8 text-purple-400" />
                </div>
                <motion.div
                    className="absolute inset-0 rounded-full border-2 border-purple-500/40"
                    animate={{ scale: [1, 1.3, 1], opacity: [0.8, 0, 0.8] }}
                    transition={{ duration: 2, repeat: Infinity }}
                />
                <motion.div
                    className="absolute inset-0 rounded-full border border-violet-500/30"
                    animate={{ scale: [1, 1.6, 1], opacity: [0.5, 0, 0.5] }}
                    transition={{ duration: 2, delay: 0.5, repeat: Infinity }}
                />
            </div>

            <div className="text-center space-y-2">
                <p className="text-white/80 font-semibold text-sm">AI 에이전트 분석 중</p>
                <AnimatePresence mode="wait">
                    <motion.p
                        key={stepIdx}
                        initial={{ opacity: 0, y: 8 }}
                        animate={{ opacity: 1, y: 0 }}
                        exit={{ opacity: 0, y: -8 }}
                        className="text-purple-400 text-xs"
                    >
                        {steps[stepIdx]}
                    </motion.p>
                </AnimatePresence>
            </div>

            {/* 진행 점 */}
            <div className="flex gap-1.5">
                {steps.map((_, i) => (
                    <div
                        key={i}
                        className={`w-1.5 h-1.5 rounded-full transition-all duration-500 ${
                            i === stepIdx ? "bg-purple-400 scale-125" : "bg-white/20"
                        }`}
                    />
                ))}
            </div>
        </div>
    );
}

// ── 리포트 표시 패널 ──────────────────────────────────────────────────────────
function ReportPanel({
    report,
    onRerun,
}: {
    report: string;
    onRerun: () => void;
}) {
    const [collapsed, setCollapsed] = useState(false);

    return (
        <div className="space-y-3">
            <div className="flex items-center justify-between">
                <div className="flex items-center gap-2 text-xs text-emerald-400">
                    <CheckCircle2 className="w-3.5 h-3.5" />
                    <span>AI 감사 완료</span>
                </div>
                <div className="flex items-center gap-2">
                    <button
                        onClick={onRerun}
                        className="text-xs text-white/40 hover:text-white/70 transition-colors px-2 py-1 rounded-lg hover:bg-white/5"
                    >
                        다시 실행
                    </button>
                    <button
                        onClick={() => setCollapsed(v => !v)}
                        className="text-xs text-white/40 hover:text-white/70 transition-colors flex items-center gap-1 px-2 py-1 rounded-lg hover:bg-white/5"
                    >
                        {collapsed ? <ChevronDown className="w-3 h-3" /> : <ChevronUp className="w-3 h-3" />}
                        {collapsed ? "펼치기" : "접기"}
                    </button>
                </div>
            </div>

            <AnimatePresence>
                {!collapsed && (
                    <motion.div
                        initial={{ opacity: 0, height: 0 }}
                        animate={{ opacity: 1, height: "auto" }}
                        exit={{ opacity: 0, height: 0 }}
                        className="overflow-hidden"
                    >
                        <div className="bg-black/20 rounded-2xl p-5 border border-white/5 prose prose-invert max-w-none">
                            {renderMarkdown(report)}
                        </div>
                    </motion.div>
                )}
            </AnimatePresence>
        </div>
    );
}

// ── 메인 컴포넌트 ─────────────────────────────────────────────────────────────
export function AiAuditSection({
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    sajuReport: _sajuReport,
    birthYear,
    birthMonth,
    birthDay,
    birthHour,
    isMale,
}: AiAuditSectionProps) {
    const [apiKey, setApiKey] = useState(() =>
        sessionStorage.getItem("eon_gemini_key") ?? ""
    );
    const [auditState, setAuditState] = useState<AiAuditState>({
        status: "idle",
        result: null,
        error: null,
    });

    const handleApiKeyChange = useCallback((key: string) => {
        setApiKey(key);
        if (key) {
            sessionStorage.setItem("eon_gemini_key", key);
        } else {
            sessionStorage.removeItem("eon_gemini_key");
        }
    }, []);

    const runAudit = useCallback(async () => {
        if (!apiKey.trim()) return;

        setAuditState({ status: "loading", result: null, error: null });

        try {
            const response = await fetch("/api/ai-audit", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                    "X-Gemini-Api-Key": apiKey.trim(),
                },
                body: JSON.stringify({
                    year: birthYear,
                    month: birthMonth,
                    day: birthDay,
                    hour: birthHour,
                    isMale,
                    birthName: "분석 대상",
                }),
            });

            if (!response.ok) {
                const errData = await response.json().catch(() => ({ error: "서버 오류" }));
                throw new Error(errData.error ?? `HTTP ${response.status}`);
            }

            const data = await response.json();

            if (data.status === "error") {
                throw new Error(data.error ?? "알 수 없는 오류");
            }

            setAuditState({
                status: "success",
                result: data,
                error: null,
            });
        } catch (err) {
            setAuditState({
                status: "error",
                result: null,
                error: err instanceof Error ? err.message : String(err),
            });
        }
    }, [apiKey, birthYear, birthMonth, birthDay, birthHour, isMale]);

    const handleRerun = useCallback(() => {
        setAuditState({ status: "idle", result: null, error: null });
    }, []);

    return (
        <motion.div
            initial={{ opacity: 0, y: 16 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.4, delay: 0.1 }}
            className="glass p-6 rounded-[2rem] border border-purple-500/10"
        >
            {/* 헤더 */}
            <div className="flex items-start justify-between mb-6">
                <div className="flex items-center gap-3">
                    <div className="w-10 h-10 rounded-xl bg-purple-500/10 border border-purple-500/20 flex items-center justify-center">
                        <Bot className="w-5 h-5 text-purple-400" />
                    </div>
                    <div>
                        <h3 className="text-base font-bold text-white flex items-center gap-2">
                            AI 운명 감사 리포트
                            <span className="text-[10px] font-normal bg-purple-500/15 text-purple-300 border border-purple-500/20 px-2 py-0.5 rounded-full">
                                AGY Agent
                            </span>
                        </h3>
                        <p className="text-xs text-white/40 mt-0.5">
                            Gemini Flash + MCP Tool Calling 기반 자율 분석
                        </p>
                    </div>
                </div>
                <div className="flex items-center gap-1.5">
                    <Shield className="w-3.5 h-3.5 text-white/20" />
                    <span className="text-[10px] text-white/30">키 미저장</span>
                </div>
            </div>

            {/* 기능 뱃지 */}
            {auditState.status === "idle" && (
                <div className="flex flex-wrap gap-2 mb-5">
                    {["엔트로피 분석", "위상 스캔", "취약점 퍼징", "근본 원인 역추적"].map(label => (
                        <span
                            key={label}
                            className="flex items-center gap-1 text-[10px] bg-white/5 border border-white/8 text-white/40 px-2.5 py-1 rounded-full"
                        >
                            <Sparkles className="w-2.5 h-2.5" />
                            {label}
                        </span>
                    ))}
                </div>
            )}

            {/* 컨텐츠 영역 */}
            <AnimatePresence mode="wait">
                {auditState.status === "idle" && (
                    <motion.div key="idle" initial={{ opacity: 0 }} animate={{ opacity: 1 }} exit={{ opacity: 0 }}>
                        <ApiKeyPanel
                            apiKey={apiKey}
                            onApiKeyChange={handleApiKeyChange}
                            onRun={runAudit}
                            isLoading={false}
                        />
                    </motion.div>
                )}

                {auditState.status === "loading" && (
                    <motion.div key="loading" initial={{ opacity: 0 }} animate={{ opacity: 1 }} exit={{ opacity: 0 }}>
                        <LoadingState />
                    </motion.div>
                )}

                {auditState.status === "success" && auditState.result && (
                    <motion.div key="success" initial={{ opacity: 0 }} animate={{ opacity: 1 }}>
                        <ReportPanel report={auditState.result.report} onRerun={handleRerun} />
                    </motion.div>
                )}

                {auditState.status === "error" && (
                    <motion.div key="error" initial={{ opacity: 0 }} animate={{ opacity: 1 }} exit={{ opacity: 0 }}>
                        <div className="flex items-start gap-3 p-4 rounded-xl bg-red-500/5 border border-red-500/20 mb-4">
                            <AlertCircle className="w-4 h-4 text-red-400 flex-shrink-0 mt-0.5" />
                            <div>
                                <p className="text-sm font-semibold text-red-400 mb-1">분석 실패</p>
                                <p className="text-xs text-white/50">{auditState.error}</p>
                            </div>
                        </div>
                        <ApiKeyPanel
                            apiKey={apiKey}
                            onApiKeyChange={handleApiKeyChange}
                            onRun={runAudit}
                            isLoading={false}
                        />
                    </motion.div>
                )}
            </AnimatePresence>
        </motion.div>
    );
}
