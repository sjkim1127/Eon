// ──────────────────────────────────────────────
// 사주 관련 타입 정의
// ──────────────────────────────────────────────

/** Rust enum 문자열 → 한자/한글 정보 */
export interface HanjaHangul {
  hanja: string;
  hangul: string;
}

/** 천간 (Heavenly Stem) 키 */
export type StemKey =
  | "Jia" | "Yi" | "Bing" | "Ding" | "Wu"
  | "Ji" | "Geng" | "Xin" | "Ren" | "Gui";

/** 지지 (Earthly Branch) 키 */
export type BranchKey =
  | "Zi" | "Chou" | "Yin" | "Mao" | "Chen" | "Si"
  | "Wu" | "Wei" | "Shen" | "You" | "Xu" | "Hai";

/** 오행 키 */
export type ElementKey = "Wood" | "Fire" | "Earth" | "Metal" | "Water";

/** 신강/신약/중화 */
export type StrengthKey = "Strong" | "Weak" | "Balanced";

/** 간지 객체 */
export interface GanZi {
  stem: string;
  branch: string;
}

/** 주주(柱) 위치 */
export type PillarPosition = "Year" | "Month" | "Day" | "Hour";

/** 사주 분석 리포트 (Rust 직렬화 결과) */
export interface SajuReport {
  pillars: {
    year: GanZi;
    month: GanZi;
    day: GanZi;
    hour: GanZi;
  };
  strength_analysis: {
    day_master: string;
    day_master_element: string;
    strength: string;
    scores: Record<string, number>;
  };
  yongshin_analysis: {
    yongshin: string;
    yongshin_element: string;
    kibshin: string;
    kibshin_element: string;
  };
  structure_analysis: {
    structure: string;
    description: string;
  };
  spirit_markers: Array<{
    spirit: string;
    pillar_position: string;
    is_positive: boolean;
    description: string;
  }>;
  major_luck_cycles: Array<{
    ganzi: GanZi;
    start_age: number;
    end_age: number;
    element: string;
  }>;
  golden_time: {
    best_hours: string[];
    reason: string;
  };
  simulation_frames: Array<{
    age: number;
    score: number;
    ganzi: GanZi;
    tags: string[];
  }>;
}

/** 운세(대운) 리포트 */
export interface TransitReport {
  yearly_luck: {
    ganzi: GanZi;
    element: string;
    influences: string[];
  };
  monthly_luck: {
    ganzi: GanZi;
    element: string;
    influences: string[];
  };
  life_frames: Array<{
    age: number;
    score: number;
    ganzi: GanZi;
    tags: string[];
    status: string;
    reason: string;
    strategy: string;
    esil_trace?: string;
    register_state?: Record<string, number>;
  }>;
  nearby_diagnostics: Array<{
    age: number;
    status: string;
    reason: string;
    strategy: string;
  }>;
}

/** 궁합 (CompatibilityAudit) */
export interface CompatibilityAudit {
  sync_score: number;
  synergies: string[];
  conflicts: string[];
  deadlocks: string[];
}
