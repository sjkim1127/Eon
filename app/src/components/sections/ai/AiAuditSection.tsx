import { useState, useCallback, useRef, useEffect } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { Bot, Sparkles, Key, Play, AlertCircle, Loader2, Send, MessageSquare, RotateCcw, HelpCircle } from "lucide-react";
import type { SajuAnalysisResult } from "../../../types/saju";
import type { AiAuditState, ChatMessage } from "../../../types/ai";

interface AiAuditSectionProps {
    sajuReport: SajuAnalysisResult;
    birthYear: number;
    birthMonth: number;
    birthDay: number;
    birthHour: number;
    isMale: boolean;
}

// ── 마크다운 기본 렌더러 (웹 메신저에 특화된 깔끔하고 가독성 높은 스타일) ──────────────────────────────
function renderMarkdown(text: string): React.ReactNode[] {
    const lines = text.split("\n");
    const elements: React.ReactNode[] = [];
    let keyIdx = 0;

    for (const line of lines) {
        const key = keyIdx++;
        if (line.startsWith("## ")) {
            elements.push(
                <h2 key={key} className="text-base font-bold text-white mt-6 mb-3 flex items-center gap-2 border-b border-white/10 pb-1.5" translate="no">
                    <Sparkles className="w-4 h-4 text-purple-400 shrink-0" />
                    {line.replace("## ", "")}
                </h2>
            );
        } else if (line.startsWith("### ")) {
            elements.push(
                <h3 key={key} className="text-sm font-bold text-purple-300 mt-4 mb-2 border-l-2 border-purple-500 pl-2.5" translate="no">
                    {line.replace("### ", "")}
                </h3>
            );
        } else if (line.startsWith("- ")) {
            elements.push(
                <li key={key} className="text-sm text-white/75 ml-4 list-disc mb-1.5 leading-relaxed" translate="no">
                    {line.replace("- ", "")}
                </li>
            );
        } else if (line.startsWith("**") && line.endsWith("**")) {
            elements.push(
                <p key={key} className="text-sm font-semibold text-purple-300 mb-2" translate="no">
                    {line.replace(/\*\*/g, "")}
                </p>
            );
        } else if (line.trim() === "---") {
            elements.push(<hr key={key} className="border-white/10 my-4" />);
        } else if (line.trim() !== "") {
            // 인라인 볼드 처리
            const parts = line.split(/\*\*(.*?)\*\*/g);
            elements.push(
                <p key={key} className="text-sm text-white/80 leading-relaxed mb-2.5" translate="no">
                    {parts.map((part, i) =>
                        i % 2 === 1
                            ? <strong key={i} className="text-purple-300 font-semibold">{part}</strong>
                            : part
                    )}
                </p>
            );
        } else {
            elements.push(<div key={key} className="h-1.5" />);
        }
    }
    return elements;
}

// ── API 키 입력 패널 (Gemini 스타일의 세련된 중앙 정렬 카드) ──────────────────────────────────────────────────────────
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
        <div className="max-w-md mx-auto my-8 md:my-16 bg-white/5 border border-white/10 rounded-[2.5rem] p-8 md:p-10 text-center space-y-6 shadow-2xl backdrop-blur-xl relative overflow-hidden">
            {/* 배경 빛 번짐 효과 */}
            <div className="absolute -top-24 -left-24 w-48 h-48 rounded-full bg-purple-500/10 blur-[80px] pointer-events-none" />
            <div className="absolute -bottom-24 -right-24 w-48 h-48 rounded-full bg-violet-500/10 blur-[80px] pointer-events-none" />

            <div className="mx-auto w-14 h-14 rounded-2xl bg-purple-500/10 border border-purple-500/25 flex items-center justify-center shadow-inner">
                <Bot className="w-7 h-7 text-purple-400" />
            </div>

            <div className="space-y-2">
                <h3 className="text-lg font-bold text-white tracking-tight">Destiny AI 감사관</h3>
                <p className="text-xs text-white/45 leading-relaxed px-2">
                    Groq Llama 3와 연동하여 사주 구조의 엔트로피 스캔, 오행 위상 트래픽 분석, 취약점 퍼징 리포트를 생성합니다.
                </p>
            </div>

            <div className="space-y-3">
                <div className="relative">
                    <input
                        id="ai-audit-api-key"
                        type={showKey ? "text" : "password"}
                        value={apiKey}
                        onChange={e => onApiKeyChange(e.target.value)}
                        placeholder="Groq API 키 입력 (gsk_...)"
                        className="w-full bg-black/20 border border-white/10 rounded-2xl px-4 py-3 text-sm text-white/80 placeholder-white/25 focus:outline-none focus:border-purple-500/50 focus:ring-1 focus:ring-purple-500/30 transition-all pr-10"
                    />
                    <button
                        onClick={() => setShowKey(v => !v)}
                        className="absolute right-3.5 top-1/2 -translate-y-1/2 text-white/30 hover:text-white/60 transition-colors text-xs"
                        aria-label={showKey ? "키 숨기기" : "키 보기"}
                    >
                        {showKey ? "숨기기" : "보기"}
                    </button>
                </div>

                <div className="flex items-start gap-2.5 p-3 rounded-xl bg-purple-500/5 border border-purple-500/10 text-left">
                    <Key className="w-3.5 h-3.5 text-purple-400 mt-0.5 shrink-0" />
                    <span className="text-[10px] text-white/40 leading-normal">
                        API 키는 <a href="https://console.groq.com/keys" target="_blank" rel="noopener noreferrer" className="text-purple-400 underline hover:text-purple-300">Groq Console</a>에서 발급받으실 수 있습니다. 입력된 키는 로컬 세션에만 임시 보관되며 수집되지 않습니다.
                    </span>
                </div>
            </div>

            <button
                id="ai-audit-run-btn"
                onClick={onRun}
                disabled={!apiKey.trim() || isLoading}
                className="w-full flex items-center justify-center gap-2 py-3 rounded-2xl bg-gradient-to-r from-purple-600 to-indigo-600 hover:from-purple-500 hover:to-indigo-500 text-white text-sm font-semibold disabled:opacity-40 disabled:cursor-not-allowed transition-all shadow-lg shadow-purple-900/20 active:scale-98"
            >
                {isLoading ? (
                    <Loader2 className="w-4.5 h-4.5 animate-spin" />
                ) : (
                    <Play className="w-4 h-4 fill-white" />
                )}
                {isLoading ? "감사 분석 로딩 중..." : "AI 감사관 시동"}
            </button>
        </div>
    );
}

// ── 로딩 애니메이션 (레이더/스캐너 테마의 몰입형 시각화) ───────────────────────────────────────────────────────────
function LoadingState() {
    const steps = [
        "분석 엔진 초기화 중...",
        "에너지 엔트로피(복잡도) 계측 중...",
        "오행 트래픽 위상 경로 스캔 중...",
        "대운 시나리오 취약점 퍼징 실행 중...",
        "근본 원인 탐색 및 리포트 조립 중...",
    ];
    const [stepIdx, setStepIdx] = useState(0);

    useEffect(() => {
        const interval = setInterval(() => {
            setStepIdx(i => (i + 1) % steps.length);
        }, 2500);
        return () => clearInterval(interval);
    }, [steps.length]);

    return (
        <div className="flex flex-col items-center justify-center py-20 gap-8">
            <div className="relative flex items-center justify-center">
                {/* 레이더 회전 스캔 이펙트 */}
                <div className="w-20 h-20 rounded-full bg-purple-500/5 border border-purple-500/20 flex items-center justify-center relative z-10 shadow-inner">
                    <Bot className="w-9 h-9 text-purple-400" />
                </div>
                {/* 동심원 파동 */}
                <motion.div
                    className="absolute w-20 h-20 rounded-full border border-purple-500/40"
                    animate={{ scale: [1, 1.8, 1], opacity: [0.6, 0, 0.6] }}
                    transition={{ duration: 2.5, repeat: Infinity, ease: "easeInOut" }}
                />
                <motion.div
                    className="absolute w-20 h-20 rounded-full border border-violet-500/20"
                    animate={{ scale: [1, 2.5, 1], opacity: [0.4, 0, 0.4] }}
                    transition={{ duration: 2.5, delay: 0.8, repeat: Infinity, ease: "easeInOut" }}
                />
            </div>

            <div className="text-center space-y-2.5 z-10">
                <div className="flex items-center justify-center gap-1.5 text-white/50 text-xs font-medium tracking-wider uppercase">
                    <Loader2 className="w-3.5 h-3.5 animate-spin text-purple-400" />
                    <span>Agent Scan Running</span>
                </div>
                <AnimatePresence mode="wait">
                    <motion.p
                        key={stepIdx}
                        initial={{ opacity: 0, y: 10 }}
                        animate={{ opacity: 1, y: 0 }}
                        exit={{ opacity: 0, y: -10 }}
                        className="text-white/85 font-semibold text-sm max-w-[260px] mx-auto leading-relaxed h-6"
                    >
                        {steps[stepIdx]}
                    </motion.p>
                </AnimatePresence>
            </div>

            {/* 인디케이터 점 */}
            <div className="flex gap-2">
                {steps.map((_, i) => (
                    <div
                        key={i}
                        className={`w-1.5 h-1.5 rounded-full transition-all duration-300 ${
                            i === stepIdx ? "bg-purple-400 scale-125 shadow-glow" : "bg-white/10"
                        }`}
                    />
                ))}
            </div>
        </div>
    );
}

// ── 타이핑 인디케이터 ─────────────────────────────────────────────────────────────
function TypingIndicator() {
    return (
        <div className="flex items-start gap-3.5 mb-4">
            <div className="w-8.5 h-8.5 rounded-xl bg-purple-500/10 border border-purple-500/20 flex items-center justify-center shrink-0 mt-0.5 shadow-sm">
                <Bot className="w-4.5 h-4.5 text-purple-400" />
            </div>
            <div className="bg-white/5 border border-white/5 rounded-2xl rounded-tl-none px-4 py-3 flex gap-1.5 items-center shadow-inner">
                <span className="w-1.5 h-1.5 bg-purple-400/80 rounded-full animate-bounce" style={{ animationDelay: "0ms" }} />
                <span className="w-1.5 h-1.5 bg-purple-400/80 rounded-full animate-bounce" style={{ animationDelay: "150ms" }} />
                <span className="w-1.5 h-1.5 bg-purple-400/80 rounded-full animate-bounce" style={{ animationDelay: "300ms" }} />
            </div>
        </div>
    );
}

// ── 대화형 메인 패널 (ChatGPT / Claude Web 메신저 스타일) ──────────────────────────────────────────────────────────
function ChatPanel({
    report,
    initialHistory,
    apiKey,
    onRerun,
}: {
    report: string;
    initialHistory: any[];
    apiKey: string;
    onRerun: () => void;
}) {
    // 최초 감사 보고서를 첫 번째 어시스턴트 메시지로 메시지 스트림에 자동 등록
    const [messages, setMessages] = useState<ChatMessage[]>(() => {
        if (initialHistory && initialHistory.length > 0) {
            const parsed: ChatMessage[] = [];
            for (const content of initialHistory) {
                const role = content.role === "model" ? "assistant" : "user";
                const part = content.parts?.[0];
                const text = part?.text;
                if (text) {
                    parsed.push({ role, content: text, timestamp: new Date() });
                }
            }
            if (parsed.length > 0) return parsed;
        }
        return [
            {
                role: "assistant",
                content: report,
                timestamp: new Date(),
            }
        ];
    });

    const [chatInput, setChatInput] = useState("");
    const [isChatLoading, setIsChatLoading] = useState(false);
    const [history, setHistory] = useState<any[]>(initialHistory);
    const [error, setError] = useState<string | null>(null);

    const chatEndRef = useRef<HTMLDivElement>(null);
    const scrollContainerRef = useRef<HTMLDivElement>(null);

    // 스크롤 동기화
    useEffect(() => {
        if (scrollContainerRef.current) {
            scrollContainerRef.current.scrollTo({
                top: scrollContainerRef.current.scrollHeight,
                behavior: "smooth"
            });
        }
    }, [messages, isChatLoading]);

    const handleSend = useCallback(async (textToSend?: string) => {
        const query = (textToSend ?? chatInput).trim();
        if (!query || isChatLoading) return;

        if (!textToSend) {
            setChatInput("");
        }
        setError(null);

        // 메시지 추가
        const userMsg: ChatMessage = { role: "user", content: query, timestamp: new Date() };
        setMessages(prev => [...prev, userMsg]);
        setIsChatLoading(true);

        try {
            const response = await fetch("/api/ai_audit", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                    "X-Groq-Api-Key": apiKey.trim(),
                },
                body: JSON.stringify({
                    action: "chat",
                    history: history,
                    message: query,
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

            // 답장 수신
            const assistantMsg: ChatMessage = {
                role: "assistant",
                content: data.reply,
                timestamp: new Date(),
            };
            setMessages(prev => [...prev, assistantMsg]);
            if (data.history) {
                setHistory(data.history);
            }
        } catch (err) {
            setError(err instanceof Error ? err.message : String(err));
        } finally {
            setIsChatLoading(false);
        }
    }, [chatInput, isChatLoading, history, apiKey]);

    const SUGGESTIONS = [
        "올해 전체적인 직장/사업운이 어떤가요?",
        "제 사주에서 건강이나 사고를 조심해야 할 시점은?",
        "오행 위상 분석 결과의 병목과 해결책을 구체적으로 알려주세요.",
        "사주 엔트로피를 조율하여 평형 상태를 만드는 보완법은?",
    ];

    return (
        <div className="flex flex-col h-[520px] md:h-[620px] bg-black/25 border border-white/10 rounded-[2rem] shadow-2xl relative overflow-hidden backdrop-blur-xl">
            {/* 상단 액션바 */}
            <div className="flex items-center justify-between px-6 py-4 border-b border-white/5 bg-white/3">
                <div className="flex items-center gap-2 text-xs text-purple-400 font-medium">
                    <MessageSquare className="w-3.5 h-3.5" />
                    <span>감사관 분석 스레드</span>
                </div>
                <button
                    onClick={onRerun}
                    className="text-xs text-white/40 hover:text-white/70 transition-colors flex items-center gap-1.5 px-3 py-1.5 rounded-xl hover:bg-white/5 border border-white/5"
                >
                    <RotateCcw className="w-3.5 h-3.5" />
                    스레드 초기화
                </button>
            </div>

            {/* 대화 스트림 영역 */}
            <div
                ref={scrollContainerRef}
                className="flex-1 overflow-y-auto p-6 space-y-6 scrollbar-thin scrollbar-thumb-white/10"
            >
                {messages.map((msg, i) => (
                    <motion.div
                        key={i}
                        initial={{ opacity: 0, y: 12 }}
                        animate={{ opacity: 1, y: 0 }}
                        transition={{ duration: 0.3 }}
                        className="w-full"
                    >
                        {msg.role === "user" ? (
                            /* 사용자 말풍선 */
                            <div className="flex justify-end w-full">
                                <div className="bg-gradient-to-br from-purple-600 to-indigo-600 rounded-2xl rounded-tr-none px-4.5 py-3 text-sm text-white font-medium max-w-[75%] shadow-md shadow-purple-950/20 leading-relaxed break-words" translate="no">
                                    {msg.content}
                                </div>
                            </div>
                        ) : (
                            /* AI 어시스턴트 메시지 (문서 뷰어 형식의 세련된 디자인) */
                            <div className="flex gap-3.5 w-full items-start">
                                <div className="w-8.5 h-8.5 rounded-xl bg-purple-500/10 border border-purple-500/20 flex items-center justify-center shrink-0 mt-0.5 shadow-sm">
                                    <Bot className="w-4.5 h-4.5 text-purple-400" />
                                </div>
                                <div className="flex-1 bg-white/5 border border-white/5 rounded-2xl rounded-tl-none p-5 shadow-inner text-white/90 leading-relaxed max-w-[85%] prose prose-invert break-words">
                                    {renderMarkdown(msg.content)}
                                </div>
                            </div>
                        )}
                    </motion.div>
                ))}

                {isChatLoading && <TypingIndicator />}
                <div ref={chatEndRef} />
            </div>

            {/* 하단 입력 & 조작 영역 */}
            <div className="p-6 border-t border-white/5 bg-white/3 space-y-4">
                {/* 에러 메시지 */}
                {error && (
                    <div className="flex items-center gap-2 p-3 rounded-xl bg-red-500/5 border border-red-500/15 text-xs text-red-400">
                        <AlertCircle className="w-4 h-4 flex-shrink-0" />
                        <span>오류 발생: {error}</span>
                    </div>
                )}

                {/* 추천 질문 (대화가 1개만 있을 때 노출) */}
                {messages.length === 1 && !isChatLoading && (
                    <div className="space-y-2">
                        <p className="text-[10px] text-white/35 font-semibold tracking-wider uppercase pl-1">Suggested Questions</p>
                        <div className="flex flex-wrap gap-2">
                            {SUGGESTIONS.map((s, idx) => (
                                <button
                                    key={idx}
                                    onClick={() => handleSend(s)}
                                    className="text-xs bg-white/5 hover:bg-purple-500/10 border border-white/5 hover:border-purple-500/25 text-white/50 hover:text-purple-300 px-3.5 py-2 rounded-2xl transition-all duration-200 text-left cursor-pointer active:scale-95"
                                >
                                    {s}
                                </button>
                            ))}
                        </div>
                    </div>
                )}

                {/* 메시지 입력 폼 */}
                <div className="max-w-3xl mx-auto w-full flex gap-2">
                    <div className="flex-1 relative flex items-center gap-2 bg-black/20 border border-white/10 rounded-2xl p-1.5 focus-within:border-purple-500/50 focus-within:ring-1 focus-within:ring-purple-500/30 transition-all">
                        <input
                            type="text"
                            value={chatInput}
                            onChange={e => setChatInput(e.target.value)}
                            onKeyDown={e => {
                                if (e.key === "Enter" && !e.nativeEvent.isComposing) {
                                    void handleSend();
                                }
                            }}
                            placeholder="감사 리포트 혹은 사주 보완에 대해 질문해보세요..."
                            disabled={isChatLoading}
                            className="flex-1 bg-transparent border-0 px-3.5 py-2 text-sm text-white placeholder-white/30 focus:outline-none focus:ring-0 disabled:opacity-50"
                        />
                        <button
                            onClick={() => void handleSend()}
                            disabled={!chatInput.trim() || isChatLoading}
                            className="w-9 h-9 rounded-xl bg-purple-600 hover:bg-purple-500 text-white flex items-center justify-center transition-colors shadow-md disabled:opacity-40 disabled:scale-100 active:scale-95 shrink-0"
                            aria-label="보내기"
                        >
                            <Send className="w-4 h-4 fill-white" />
                        </button>
                    </div>
                </div>
            </div>
        </div>
    );
}

// ── 메인 감사 섹션 컴포넌트 ─────────────────────────────────────────────────────────────
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
        sessionStorage.getItem("eon_groq_key") ?? ""
    );
    const [auditState, setAuditState] = useState<AiAuditState>({
        status: "idle",
        result: null,
        error: null,
    });

    const handleApiKeyChange = useCallback((key: string) => {
        setApiKey(key);
        if (key) {
            sessionStorage.setItem("eon_groq_key", key);
        } else {
            sessionStorage.removeItem("eon_groq_key");
        }
    }, []);

    const runAudit = useCallback(async () => {
        if (!apiKey.trim()) return;

        setAuditState({ status: "loading", result: null, error: null });

        try {
            const response = await fetch("/api/ai_audit", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                    "X-Groq-Api-Key": apiKey.trim(),
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
            className="w-full space-y-6"
        >
            {/* 헤더 */}
            <div className="flex items-center justify-between">
                <div className="flex items-center gap-3">
                    <div className="w-10 h-10 rounded-2xl bg-purple-500/10 border border-purple-500/20 flex items-center justify-center shadow-inner">
                        <Bot className="w-5 h-5 text-purple-400" />
                    </div>
                    <div>
                        <h3 className="text-base font-bold text-white flex items-center gap-2">
                            AI 운명 감사 리포트
                            <span className="text-[10px] font-normal bg-purple-500/15 text-purple-300 border border-purple-500/20 px-2.5 py-0.5 rounded-full">
                                AGY Agent
                            </span>
                        </h3>
                        <p className="text-xs text-white/40 mt-0.5">
                            Groq Llama 3 + MCP Tool Calling 기반 대화형 자율 분석관
                        </p>
                    </div>
                </div>
                <div className="flex items-center gap-2 px-3 py-1.5 rounded-xl bg-white/5 border border-white/8 text-[10px] text-white/40">
                    <HelpCircle className="w-3.5 h-3.5 text-white/30" />
                    <span>API 키 임시 보관</span>
                </div>
            </div>

            {/* 컨텐츠 영역 */}
            <AnimatePresence mode="wait">
                {auditState.status === "idle" ? (
                    <motion.div key="idle" initial={{ opacity: 0 }} animate={{ opacity: 1 }} exit={{ opacity: 0 }}>
                        <ApiKeyPanel
                            apiKey={apiKey}
                            onApiKeyChange={handleApiKeyChange}
                            onRun={runAudit}
                            isLoading={false}
                        />
                    </motion.div>
                ) : auditState.status === "loading" ? (
                    <motion.div key="loading" initial={{ opacity: 0 }} animate={{ opacity: 1 }} exit={{ opacity: 0 }}>
                        <LoadingState />
                    </motion.div>
                ) : auditState.status === "success" && auditState.result ? (
                    <motion.div key="success" initial={{ opacity: 0 }} animate={{ opacity: 1 }}>
                        <ChatPanel
                            report={auditState.result.report}
                            initialHistory={auditState.result.history ?? []}
                            apiKey={apiKey}
                            onRerun={handleRerun}
                        />
                    </motion.div>
                ) : auditState.status === "error" ? (
                    <motion.div key="error" initial={{ opacity: 0 }} animate={{ opacity: 1 }} exit={{ opacity: 0 }}>
                        <div className="flex items-start gap-3 p-4 rounded-xl bg-red-500/5 border border-red-500/20 mb-4 max-w-md mx-auto">
                            <AlertCircle className="w-4.5 h-4.5 text-red-400 shrink-0 mt-0.5" />
                            <div>
                                <p className="text-sm font-semibold text-red-400 mb-1">감사 실행 중 에러</p>
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
                ) : null}
            </AnimatePresence>
        </motion.div>
    );
}
