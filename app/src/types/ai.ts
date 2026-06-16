// AI 감사 관련 타입 정의

export interface AiAuditRequest {
    year: number;
    month: number;
    day: number;
    hour: number;
    isMale: boolean;
    birthName?: string;
}

export interface AiAuditResult {
    report: string;    // 마크다운 형식의 감사 리포트
    status: "success" | "error";
    error?: string;
    history?: any[];   // 직렬화된 대화 히스토리
}

export type AiAuditStatus = "idle" | "loading" | "success" | "error";

export interface AiAuditState {
    status: AiAuditStatus;
    result: AiAuditResult | null;
    error: string | null;
}

export interface ChatMessage {
    role: "user" | "assistant";
    content: string;
    timestamp: Date;
}
