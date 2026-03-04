import { ganziDisplay, ganziHangul } from "./ganzi";
import {
    SIGN_NAMES, VARGA_DEFS, ASHTA_LABELS, ASHTA_MAX,
    STEM_INFO, ELEMENT_INFO, STRENGTH_INFO, TENGOD_INFO,
    STRUCTURE_INFO, SPIRIT_INFO, PILLAR_POS_INFO, YONGSHIN_TYPE_INFO,
} from "../constants";
import type { SajuAnalysisResult } from "../types";
import type { VedicAnalysisResult } from "../types";
import type { TransitResult } from "../types";
import type { CompReport } from "../types";
import { computeTierResult, type TierResult } from "./tierScore";
import { getNakshatraInfo } from "./nakshatra";
import { formatSiderealPosition, buildNakshatraMarkdownRows } from "./vedicFormat";

/** TraceTag / LifeFrame.tags 가 문자열 혹은 { Key: null } 객체로 올 수 있음 — 항상 문자열로 변환 */
const tagToStr = (tag: unknown): string => {
    if (typeof tag === "string") return tag;
    if (tag && typeof tag === "object") {
        const keys = Object.keys(tag as object);
        return keys.length > 0 ? keys[0] : "";
    }
    return String(tag ?? "");
};

/** TenGod 영문 키 → 한글 변환 */
const tgKr = (key: string) => TENGOD_INFO[key]?.hangul ?? key;
/** Element 영문 키 → 한글(한자) */
const elKr = (key: string) => {
    const e = ELEMENT_INFO[key];
    return e ? `${e.hangul}(${e.hanja})` : key;
};
/** Stem 영문 키 → 한자·한글 */
const stemKr = (key: string) => {
    const s = STEM_INFO[key];
    return s ? `${s.hanja}(${s.hangul})` : key;
};

const SEP = "\n---\n";

/** ISO 8601 → "YYYY년 MM월" (다샤 타임라인용) */
const fmtYearMonth = (iso: string | null | undefined): string => {
    if (!iso) return "—";
    const d = new Date(iso);
    if (isNaN(d.getTime())) return iso.slice(0, 10);
    return `${d.getUTCFullYear()}년 ${String(d.getUTCMonth() + 1).padStart(2, "0")}월`;
};

/**
 * 백엔드 Varga 낙샤트라 데이터가 없을 때 프론트엔드에서 근사 계산합니다.
 * 공식: sidereal_deg × divisionCount mod 360 → 낙샤트라 위치
 * 정밀도는 Swiss Ephemeris 수준이 아니지만 참고 값으로 활용합니다.
 */
function computeFrontendVargaRows(
    allPos: Array<{ planet?: string; sidereal_deg?: number; is_retrograde?: boolean; is_combust?: boolean; [key: string]: unknown }>,
    divisionCount: number,
) {
    return allPos.map(p => {
        const name = (p.planet as string) ?? "ASC";
        const rawDeg = (p.sidereal_deg ?? 0) * divisionCount;
        const vargaDeg = ((rawDeg % 360) + 360) % 360;
        const ni = getNakshatraInfo(vargaDeg);
        // 분할 차트 사인/하우스는 라시 인덱스(0-based)로 계산
        const signIdx = Math.floor(vargaDeg / 30) + 1;
        return {
            planet: name,
            position_str: formatSiderealPosition(vargaDeg),
            sign: signIdx,
            house: 0,
            nakshatra: ni.pada,
            nakshatra_name: ni.name,
            pada: ni.pada,
            pada_range: ni.range,
            nakshatra_lord: ni.lord,
            pada_lord: ni.padaLord,
            deity: ni.deity,
            purpose: ni.purpose,
            is_retrograde: !!p.is_retrograde,
            is_combust: !!p.is_combust,
        };
    });
}

// ── 사주 섹션 ────────────────────────────────────────

export function buildSajuMarkdown(s: SajuAnalysisResult): string {
    const r = s.report;
    const lines: string[] = [];

    lines.push("# 사주명리 분석 리포트\n");

    // 보정 시간 / DST
    lines.push(`**보정된 출생 시간**: ${s.corrected_time} (입력 시간/시간대 보정을 반영한 기준 시간)`);
    if (s.is_dst) lines.push(`**서머타임(DST)**: 적용 (${s.dst_offset_hours ?? 1}시간, 지역 규칙 기반 보정)`);
    else lines.push(`**서머타임(DST)**: 미적용`);
    lines.push("");

    // 사주
    const p = r.pillars;
    lines.push("## 사주 (四柱)\n");
    lines.push("| 기둥 | 간지 (한자) | 간지 (한글) |");
    lines.push("|---|---|---|");
    lines.push(`| 년주 | ${ganziDisplay(p.year)} | ${ganziHangul(p.year)} |`);
    lines.push(`| 월주 | ${ganziDisplay(p.month)} | ${ganziHangul(p.month)} |`);
    lines.push(`| 일주 | ${ganziDisplay(p.day)} | ${ganziHangul(p.day)} |`);
    lines.push(`| 시주 | ${ganziDisplay(p.hour)} | ${ganziHangul(p.hour)} |`);
    lines.push("");

    // 신강/신약
    const st = r.strength;
    const dayMasterKr = stemKr(st.day_master);
    const strengthTypeKr = STRENGTH_INFO[st.strength_type] ?? st.strength_type;
    lines.push("## 신강·신약 분석\n");
    lines.push(`- **일간**: ${dayMasterKr} (나의 기본 기운/성향의 중심)`);
    lines.push(`- **신강/신약**: ${strengthTypeKr} (점수: ${Math.round(st.strength_score * 100) / 100}, 체력/버팀의 강도)`);
    lines.push(`- **득령(得令)**: ${st.deuk_ryeong.acquired ? "✅" : "❌"} (계절의 도움)`);
    lines.push(`- **득지(得地)**: ${st.deuk_ji.acquired ? "✅" : "❌"} (뿌리/근거의 도움)`);
    lines.push(`- **득시(得時)**: ${st.deuk_si.acquired ? "✅" : "❌"} (시간대의 도움)`);
    const rawSr = st.deuk_se?.support_ratio ?? 0;
    const supportPctForExport = rawSr > 1 ? rawSr : rawSr * 100;
    lines.push(`- **득세 지지비율**: ${supportPctForExport.toFixed(1)}%`);
    lines.push("");

    // 용신
    const y = r.yongshin;
    lines.push("## 용신 분석 (균형을 맞추는 방향)\n");
    lines.push(`- **주 용신**: ${elKr(y.primary)}`);
    lines.push(`- **보조 용신**: ${elKr(y.assistant)}`);
    if (y.recommendations?.length) {
        for (const rec of y.recommendations) {
            const ynTypeKr = YONGSHIN_TYPE_INFO[rec.yongshin_type] ?? rec.yongshin_type;
            lines.push(`- ${ynTypeKr} (${elKr(rec.element)}): ${rec.reason}`);
        }
    }
    lines.push("");

    // 격국
    const structInfo = STRUCTURE_INFO[r.structure.structure];
    lines.push("## 격국 (구조/패턴)\n");
    lines.push(`- **격국명**: ${structInfo ? `${structInfo.hangul}(${structInfo.hanja})` : r.structure.structure}`);
    lines.push(`- **사유**: ${r.structure.reason}`);
    if (r.structure.projected_stem) lines.push(`- **투출 천간**: ${stemKr(r.structure.projected_stem)}`);
    lines.push("");

    // 신살
    if (r.spirit_markers?.markers?.length) {
        lines.push("## 신살 (특수 패턴 태그)\n");
        lines.push("| 신살명 | 위치 | 천간/지지 |");
        lines.push("|---|---|---|");
        // 중복 제거
        const seenMarkers = new Set<string>();
        for (const m of r.spirit_markers.markers) {
            const key = `${m.marker}|${m.position}|${m.is_stem}`;
            if (seenMarkers.has(key)) continue;
            seenMarkers.add(key);
            const mKr = SPIRIT_INFO[m.marker]?.hangul ?? m.marker;
            const posKr = PILLAR_POS_INFO[m.position] ?? m.position;
            lines.push(`| ${mKr} | ${posKr} | ${m.is_stem ? "천간" : "지지"} |`);
        }
        lines.push("");
    }

    // 대운
    const mlDir: Record<string, string> = { Forward: "순행 (順行)", Reverse: "역행 (逆行)" };
    if (r.major_luck?.cycles?.length) {
        lines.push("## 대운 (10년 단위 큰 흐름)\n");
        lines.push(`- **순행/역행**: ${mlDir[r.major_luck.direction] ?? r.major_luck.direction}`);
        lines.push(`- **대운 시작**: ${r.major_luck.start_age}세\n`);
        lines.push("| 간지 | 시작 나이 | 종료 나이 | 천간 십신 | 지지 십신 |");
        lines.push("|---|---|---|---|---|");
        for (const c of r.major_luck.cycles) {
            lines.push(`| ${ganziDisplay(c.ganzi)} | ${c.start_age}세 | ${c.end_age}세 | ${tgKr(c.stem_god)} | ${tgKr(c.branch_god)} |`);
        }
        lines.push("");
    }

    // 골든타임
    if (r.golden_time) {
        lines.push("## 골든타임 (점수 흐름상 ‘좋게 풀리기 쉬운’ 구간)\n");
        lines.push(`- **기간**: ${r.golden_time.start_age}세 ~ ${r.golden_time.end_age}세`);
        lines.push(`- **평균 점수**: ${r.golden_time.average_score.toFixed(1)}`);
        lines.push(`- **설명**: ${r.golden_time.description}`);
        lines.push("");
    }

    // 운명 복잡도
    if (s.entropy) {
        lines.push("## 운명 복잡도 (삶의 변수/꼬임의 정도)\n");
        lines.push(`- **점수**: ${s.entropy.score.toFixed(3)} (높을수록 변수가 많고 복합적일 가능성)`);
        lines.push(`- **등급**: ${s.entropy.level}`);
        if (s.entropy.description) lines.push(`- **설명**: ${s.entropy.description}`);
        if (s.entropy.unpacker_element) lines.push(`- **해결 열쇠 오행**: ${s.entropy.unpacker_element}`);
        lines.push("");
    }

    // 에너지 흐름
    if (s.qi_topology) {
        lines.push("## 오행 에너지 흐름 (상생/상극의 순환 상태)\n");
        lines.push(`- **전체 순환 지수**: ${(s.qi_topology.throughput * 100).toFixed(0)}% (높을수록 흐름이 막힘 없이 원활)`);
        if (s.qi_topology.bottleneck) {
            const bnEl = typeof s.qi_topology.bottleneck === "string" ? elKr(s.qi_topology.bottleneck) : s.qi_topology.bottleneck;
            lines.push(`- **흐름 정체 오행**: ${bnEl}`);
        }
        lines.push("\n| 오행 | 용량 | 산출 |");
        lines.push("|---|---|---|");
        for (const n of s.qi_topology.nodes ?? []) {
            const elKey = typeof n.element === "string" ? n.element : ((n.element as { english?: string })?.english ?? "");
            const elLabel = ELEMENT_INFO[elKey] ? `${ELEMENT_INFO[elKey].hangul}(${ELEMENT_INFO[elKey].hanja})` : ((n.element as { hangul?: string })?.hangul ?? elKey);
            lines.push(`| ${elLabel} | ${n.capacity.toFixed(1)} | ${n.output.toFixed(1)} |`);
        }
        lines.push("");
    }

    // 인생 갈림길 분석 (Cyclomatic Complexity)
    if (s.complexity) {
        lines.push("## 인생 갈림길 분석 (중요 분기점 요약)\n");
        lines.push(`- **갈림길 지수**: ${s.complexity.cyclomatic_complexity} (중요 선택이 많을수록 값이 커집니다.)`);
        lines.push(`- **안정성 등급**: ${s.complexity.stability_grade} (전체 흐름의 안정/불안정 요약)`);
        lines.push(`- **변화의 불안정성**: ${Number.isFinite(s.complexity.entropy) ? s.complexity.entropy.toFixed(2) : "—"} (높을수록 변동·혼선 가능성)`);
        if (Array.isArray(s.complexity.decision_nodes) && s.complexity.decision_nodes.length > 0) {
            lines.push(`- **주요 분기점(나이)**: ${s.complexity.decision_nodes.slice(0, 40).join("세, ")}${s.complexity.decision_nodes.length > 40 ? `세 ... 외 ${s.complexity.decision_nodes.length - 40}개` : "세"}`);
        }
        lines.push("");
    }

    // 주의가 필요한 시기 (대운·세운 전수 조사)
    if (s.vulnerability_report?.critical_vectors?.length) {
        lines.push("## 주의가 필요한 시기 (대운·세운 전수 조사)\n");
        lines.push(`- **총 탐지 건수**: ${s.vulnerability_report.total_crashes}개 (위험도가 높은 순으로 상위 항목만 정리)`);
        lines.push("");
        lines.push("| 위험도 점수 | 대운 | 세운 | 유형 | 주요 태그 |");
        lines.push("|---:|---|---|---|---|");
        const top = [...s.vulnerability_report.critical_vectors]
            .sort((a, b) => (a.crash_score ?? 9999) - (b.crash_score ?? 9999))
            .slice(0, 12);
        for (const v of top) {
            const major = v.vector?.major ? `${ganziDisplay(v.vector.major)} (${ganziHangul(v.vector.major)})` : "—";
            const yearly = v.vector?.yearly ? `${ganziDisplay(v.vector.yearly)} (${ganziHangul(v.vector.yearly)})` : "—";
            const tags = (v.tags ?? []).slice(0, 8).map(tagToStr).join(", ");
            lines.push(`| ${Number.isFinite(v.crash_score) ? v.crash_score.toFixed(1) : "—"} | ${major} | ${yearly} | ${v.vulnerability_type ?? "—"} | ${tags || "—"} |`);
        }
        lines.push("");
    }

    // 정밀 분석(power)
    if (r.power) {
        lines.push("## 정밀 분석 요약 (오행·십신 분포)\n");
        lines.push(`- **우세 오행**: ${elKr(r.power.dominant_element)} (가장 강하게 나타나는 기운)`);
        lines.push(`- **우세 십신**: ${tgKr(r.power.dominant_ten_god)} (행동/관계 패턴의 중심 역할)`);
        if (Array.isArray(r.power.element_scores) && r.power.element_scores.length) {
            lines.push("\n### 오행 점수\n");
            lines.push("| 오행 | 비중(%) | 점수 |");
            lines.push("|---|---:|---:|");
            for (const [el, pct, score] of r.power.element_scores) {
                lines.push(`| ${elKr(el as string)} | ${(pct ?? 0).toFixed(1)} | ${(score ?? 0).toFixed(1)} |`);
            }
            lines.push("");
        }
        if (Array.isArray(r.power.ten_god_scores) && r.power.ten_god_scores.length) {
            lines.push("### 십신 점수\n");
            lines.push("| 십신 | 비중(%) | 점수 |");
            lines.push("|---|---:|---:|");
            for (const [tg, pct, score] of r.power.ten_god_scores) {
                lines.push(`| ${tgKr(tg as string)} | ${(pct ?? 0).toFixed(1)} | ${(score ?? 0).toFixed(1)} |`);
            }
            lines.push("");
        }
    }

    // 십신(기둥별)
    if (r.ten_gods) {
        lines.push("## 십신 배치 (기둥별 역할)\n");
        lines.push("| 위치 | 천간 | 지지 |");
        lines.push("|---|---|---|");
        lines.push(`| 년주 | ${tgKr(r.ten_gods.year_stem)} | ${tgKr(r.ten_gods.year_branch)} |`);
        lines.push(`| 월주 | ${tgKr(r.ten_gods.month_stem)} | ${tgKr(r.ten_gods.month_branch)} |`);
        lines.push(`| 일주 | ${tgKr(r.ten_gods.day_stem)} | ${tgKr(r.ten_gods.day_branch)} |`);
        lines.push(`| 시주 | ${tgKr(r.ten_gods.hour_stem)} | ${tgKr(r.ten_gods.hour_branch)} |`);
        lines.push("");
    }

    // 위기 요약
    lines.push("## 시뮬레이션 위기 요약 (급격한 변동/충돌 신호)\n");
    lines.push(`- **위기 발생 횟수**: ${s.crash_count}번`);
    lines.push("");

    // 부하 진단
    if (s.load_diagnostics?.length) {
        lines.push("## 인생 부하 진단 (과부하/다운 구간과 대응 전략)\n");
        lines.push("| 나이 | 상태 | 원인 | 대응 전략 |");
        lines.push("|---:|---|---|---|");
        for (const d of s.load_diagnostics) {
            lines.push(`| ${d.age}세 | ${d.status} | ${d.reason} | ${d.strategy} |`);
        }
        lines.push("");
    }

    // 린트
    if (s.lints?.length) {
        lines.push("## 사주 체크업 (Linter)\n");
        lines.push("| 심각도 | 코드 | 메시지 | 조언 |");
        lines.push("|---|---|---|---|");
        for (const l of s.lints) {
            lines.push(`| ${l.severity} | ${l.code} | ${l.message} | ${l.advice} |`);
        }
        lines.push("");
    }

    // VM 요약
    if (r.vm_summary) {
        lines.push("## VM 분석 요약\n");
        lines.push(r.vm_summary);
        lines.push("");
    }

    // 연도별 점수 타임라인
    if (r.timeline?.length) {
        lines.push("## 연도별 흐름 점수 타임라인 (100년 시뮬레이션)\n");
        lines.push("| 나이 | 연도 | 총점 | 재물 | 커리어 | 학업 | 건강 | 변동성 | 전환기 |");
        lines.push("|---:|---:|---:|---:|---:|---:|---:|---:|---|");
        for (const t of r.timeline) {
            lines.push(`| ${t.age} | ${t.year} | ${t.total_score.toFixed(1)} | ${t.wealth_score.toFixed(1)} | ${t.career_score.toFixed(1)} | ${t.academic_score.toFixed(1)} | ${t.health_score.toFixed(1)} | ${t.volatility_index.toFixed(1)} | ${t.is_transition_period ? "✅" : "—"} |`);
        }
        lines.push("");
    }

    // 시뮬레이션 프레임 요약 + 위험 프레임 ESIL 일부
    if (r.simulation_frames?.length) {
        lines.push("## 시뮬레이션 프레임 요약 (대운·세운 기반 스냅샷)\n");
        lines.push("| 나이 | 세운 | 대운 | 점수 | 태그 |");
        lines.push("|---:|---|---|---:|---|");
        for (const f of r.simulation_frames) {
            const yearly = f.ganzi ? `${ganziDisplay(f.ganzi)} (${ganziHangul(f.ganzi)})` : "—";
            const major = f.major_ganzi ? `${ganziDisplay(f.major_ganzi)} (${ganziHangul(f.major_ganzi)})` : "—";
            const tags = (f.tags ?? []).map(tagToStr).filter(Boolean).join(", ");
            lines.push(`| ${f.age} | ${yearly} | ${major} | ${(f.score ?? 0).toFixed(1)} | ${tags || "—"} |`);
        }
        lines.push("");

        const topFrames = [...r.simulation_frames].sort((a, b) => (a.score ?? 9999) - (b.score ?? 9999)).slice(0, 8);
        lines.push("### ESIL 트레이스 (상위 위험 프레임 일부)\n");
        for (const f of topFrames) {
            lines.push(`- **${f.age}세** (${ganziDisplay(f.ganzi)} / 대운 ${ganziDisplay(f.major_ganzi)}) 점수 ${(f.score ?? 0).toFixed(1)}`);
            const esilRaw = (f.esil_trace ?? "").trim();
            if (esilRaw) {
                lines.push("");
                for (const tl of esilRaw.split("\n").filter(Boolean)) {
                    // 세미콜론으로 구분된 경우도 처리
                    const parts = tl.includes(";") ? tl.split(";").map(s => s.trim()).filter(Boolean) : [tl.trim()];
                    for (const part of parts) {
                        const formatted = formatEsilLine(part);
                        lines.push(`  - ${formatted}`);
                    }
                }
            }
        }
        lines.push("");
    }

    // 기타(raw) - 구조가 고정되지 않아 JSON으로 덤프
    if (r.voids != null) {
        lines.push("## 기타: 공망/결손 정보 (Raw)\n");
        lines.push("```");
        lines.push(typeof r.voids === "string" ? r.voids : JSON.stringify(r.voids, null, 2));
        lines.push("```");
        lines.push("");
    }
    if (r.relationships != null) {
        lines.push("## 기타: 합충형해/관계 분석 (Raw)\n");
        lines.push("```");
        lines.push(typeof r.relationships === "string" ? r.relationships : JSON.stringify(r.relationships, null, 2));
        lines.push("```");
        lines.push("");
    }

    return lines.join("\n");
}

// ── 베딕 섹션 ────────────────────────────────────────

export function buildVedicMarkdown(v: VedicAnalysisResult): string {
    const r = v.report;
    const c = v.chart;
    const lines: string[] = [];

    lines.push("# 베딕 점성학 분석 리포트\n");

    // 카라카
    lines.push("## 주요 카라카 (인생의 핵심 역할을 나타내는 지표)\n");
    lines.push(`- **영혼 지표 (Atmakaraka)**: ${r.primary_karakas.atmakaraka}`);
    lines.push(`- **직업 지표 (Amatyakaraka)**: ${r.primary_karakas.amatyakaraka}`);
    lines.push(`- **파트너 지표 (Darakaraka)**: ${r.primary_karakas.darakaraka}`);
    lines.push("");

    // 8 카라카
    if (c?.karakas?.length) {
        lines.push("## 제미니 카라카 (8가지 인생 역할)\n");
        lines.push("| 역할 | 행성 | 도수 |");
        lines.push("|---|---|---|");
        const roleKr: Record<string, string> = {
            Atmakaraka: "영혼", Amatyakaraka: "직업", Bhratrukaraka: "형제",
            Matrukaraka: "어머니", Pitrikaraka: "아버지", Putrakaraka: "자식",
            Gnatikaraka: "경쟁자", Darakaraka: "배우자",
        };
        for (const k of c.karakas) {
            lines.push(`| ${roleKr[k.role] ?? k.role} | ${k.planet} | ${(k.degree_in_rasi ?? 0).toFixed(2)}° |`);
        }
        lines.push("");
    }

    // 대운/낙샤트라/사데사티
    lines.push("## 현재 대운 & 주요 정보 (현재 시점의 흐름)\n");
    lines.push(`- **대운 (Dasha)**: ${r.dasha_focus} (현재 집중되는 시기 구간)`);
    lines.push(`- **낙샤트라**: ${r.nakshatra_info}`);
    lines.push(`- **사데사티**: ${r.sade_sati}`);
    lines.push(`- **전체 차트 강도**: ${Math.round(r.overall_strength_score)}/600`);
    lines.push("");

    // 다샤 타임라인 — 마하다샤 + 안타르다샤 2단계
    if (Array.isArray(r.dasha_timeline) && r.dasha_timeline.length > 0) {
        lines.push("## 다샤 타임라인 (Vimshottari Dasha — 마하다샤 & 안타르다샤)\n");
        lines.push("> 다샤는 달(Moon)의 낙샤트라 위치를 기준으로 산출하는 베딕 시간 주기입니다.\n");
        lines.push("| 구분 | 행성 (Planet) | 시작 | 종료 |");
        lines.push("|---|---|---|---|");
        for (const maha of r.dasha_timeline) {
            lines.push(`| **Mahadasha** | **${maha.lord}** | ${fmtYearMonth(maha.start_time)} | ${fmtYearMonth(maha.end_time)} |`);
            for (const antar of maha.sub_dashas ?? []) {
                lines.push(`| └ Antardasha | ${antar.lord} | ${fmtYearMonth(antar.start_time)} | ${fmtYearMonth(antar.end_time)} |`);
            }
        }
        lines.push("");
    }

    // 하우스 요약
    if (r.house_summary?.length) {
        lines.push("## 하우스별 에너지 (삶의 영역별 지원/강도)\n");
        lines.push("| 하우스 | 점수 | 등급 |");
        lines.push("|---|---|---|");
        for (const h of r.house_summary) {
            lines.push(`| ${h.house} | ${Math.round(h.total_score)} | ${h.rating} |`);
        }
        lines.push("");
    }

    // BhavaStrength
    if (c?.bhava_strengths?.length) {
        lines.push("## 하우스 강점 상세 (점수 구성)\n");
        lines.push("| 하우스 | 주인 행성 힘 | 방위 힘 | 시선 영향 | 총점 |");
        lines.push("|---|---|---|---|---|");
        for (const b of c.bhava_strengths) {
            lines.push(`| ${b.house} | ${(b.lord_score ?? 0).toFixed(1)} | ${(b.dig_score ?? 0).toFixed(1)} | ${(b.drishti_score ?? 0).toFixed(1)} | ${(b.total_score ?? 0).toFixed(1)} |`);
        }
        lines.push("");
    }

    // SAV
    if (c?.sav?.points) {
        const pts = c.sav.points;
        if (Array.isArray(pts) && pts.length === 12) {
            lines.push("## SAV (사르바아슈타카바르가)\n");
            lines.push("| " + Array.from({ length: 12 }, (_, i) => `H${i + 1}`).join(" | ") + " |");
            lines.push("| " + Array.from({ length: 12 }, () => "---").join(" | ") + " |");
            lines.push("| " + pts.join(" | ") + " |");
            lines.push("");
        }
    }

    // Vimshopaka
    if (c?.vimshopaka_scores?.length) {
        lines.push("## 빔쇼파카 발라 (행성 종합 힘, 20점 만점)\n");
        lines.push("| 행성 | 6분할 점수 | 16분할 점수 | 평균 |");
        lines.push("|---|---|---|---|");
        for (const [planet, score] of c.vimshopaka_scores) {
            const s6 = score?.shadvarga_score ?? 0;
            const s16 = score?.shodashavarga_score ?? 0;
            lines.push(`| ${planet} | ${s6.toFixed(1)} | ${s16.toFixed(1)} | ${((s6 + s16) / 2).toFixed(1)} |`);
        }
        lines.push("");
    }

    // Avasthas (행성 상태)
    if (c?.avasthas?.length) {
        lines.push("## 행성 상태 (아바스타)\n");
        lines.push("| 행성 | Baladi | Jagradi |");
        lines.push("|---|---|---|");
        for (const a of c.avasthas) {
            lines.push(`| ${a.planet} | ${a.baladi} | ${a.jagradadi} |`);
        }
        lines.push("");
    }

    // 하우스 커스프
    if (Array.isArray(c?.house_cusps) && c.house_cusps.length) {
        lines.push("## 하우스 커스프 (경계 도수)\n");
        lines.push(c.house_cusps.map((deg, i) => `- H${i + 1}: ${(deg ?? 0).toFixed(2)}°`).join("\n"));
        lines.push("");
    }

    // Aspects
    if (c?.aspects?.length) {
        lines.push("## 행성 시선 (Drishti)\n");
        lines.push("| 행성 | 바라보는 하우스 |");
        lines.push("|---|---|");
        for (const a of c.aspects) {
            lines.push(`| ${a.aspecting_planet} | ${(a.aspected_houses ?? []).map((h: number) => `H${h}`).join(", ")} |`);
        }
        lines.push("");
    }

    // 요가
    if (r.yogas?.length) {
        lines.push("## 베딕 요가\n");
        lines.push("| 요가명 | 품질 | 관련 행성 | 설명 |");
        lines.push("|---|---|---|---|");
        for (const yoga of r.yogas) {
            const q = typeof yoga.quality === "string" ? yoga.quality : "Weak";
            lines.push(`| ${yoga.name} | ${q} | ${(yoga.planets_involved ?? []).join(", ")} | ${yoga.description} |`);
        }
        lines.push("");
    }

    // ── 낙샤트라 리포트 D1~D144 통합 섹션 ──
    lines.push("## 낙샤트라 리포트 (D1~D144 전체)\n");
    lines.push("> 컬럼: 행성 | 위치(사이드리얼) | 사인 | 하우스 | 낙샤트라(파다) | 파다 범위 | 낙샤트라 로드 | 파다 로드 | 신(Deity) | 목적(Purpose)\n");
    lines.push("> D1은 본 차트 기준, D2 이상은 해당 분할 좌표 기준입니다. (※ 백엔드 미제공 차트는 프론트엔드 근사값)\n");
    const allPos = [...(c?.planets ?? []), ...(c?.ascendant ? [c.ascendant] : [])];
    const vargaReportsMap = v.varga_nakshatra_reports?.reports;
    for (const vargaDef of VARGA_DEFS) {
        const rep = vargaReportsMap?.[vargaDef.id];
        if (rep?.rows?.length) {
            // 백엔드 Rust 계산 우선
            const lagna = rep.lagna_rasi ? ` (라그나: ${SIGN_NAMES[rep.lagna_rasi] ?? rep.lagna_rasi})` : "";
            lines.push(`### ${rep.varga_label} · ${vargaDef.name}${lagna}`);
            const showHouse = vargaDef.id !== "rasi";
            const mdRows = buildNakshatraMarkdownRows(rep.rows, showHouse);
            for (const row of mdRows) lines.push(row);
        } else {
            // 프론트엔드 근사 계산 폴백
            lines.push(`### ${vargaDef.label} · ${vargaDef.name} *(근사값)*`);
            const rows = computeFrontendVargaRows(allPos, vargaDef.divisionCount);
            const mdRows = buildNakshatraMarkdownRows(rows, vargaDef.id !== "rasi");
            for (const row of mdRows) lines.push(row);
        }
        lines.push("");
    }

    // 고차라(Gochara) 트랜싯
    if (v.gochara?.transits?.length) {
        lines.push("## 고차라 트랜싯 (현재 행성 이동)\n");
        lines.push("| 행성 | 현재 라시 | 달 기준 하우스 | 길/흉 | 차단 | Murti |");
        lines.push("|---|---|---:|---|---|---|");
        for (const t of v.gochara.transits) {
            const rasiName = SIGN_NAMES[t.current_rasi] ?? t.current_rasi;
            lines.push(`| ${t.planet} | ${rasiName} | ${t.house_from_moon} | ${t.is_benefic_transit ? "✅" : "—"} | ${t.is_blocked ? "✅" : "—"} | ${t.murti} |`);
        }
        lines.push("");
    }

    // Panchanga
    if (c?.panchanga) {
        const pan = c.panchanga;
        lines.push("## 판창가 (Panchanga, 날짜의 질감)\n");
        lines.push(`- **바라 (요일)**: ${pan.vara}`);
        lines.push(`- **티티 (음력일)**: ${pan.tithi} — ${pan.tithi_name}`);
        lines.push(`- **낙샤트라**: No. ${pan.nakshatra}`);
        lines.push(`- **카라나**: ${pan.karana_name}`);
        lines.push(`- **일주/시주 천주**: ${pan.day_lord} / ${pan.hour_lord}`);
        lines.push(`- **출생 시간대**: ${pan.is_day_birth ? "주간" : "야간"}`);
        lines.push("");
    }

    return lines.join("\n");
}

// ══ ESIL 트레이스 포맷 변환 (TransitTab.formatTraceLine 이식) ═══════════

function formatEsilLine(line: string): string {
    let formatted = line;
    if (line.includes("infl:")) {
        formatted = line
            .replace(/([가-힣]+)_infl:([가-힣]),weight:([0-9.]+),score\+=([0-9.]+)/, "$1의 $2 기운 유입 (영향력 x$3) ➔ +$4점")
            .replace(/([가-힣]+)_infl:([가-힣]),weight:([0-9.]+),impact:-([0-9.]+)/, "$1의 $2 기운 유입 (과부하 x$3) ➔ -$4점");
    } else if (line.includes("shinsal:")) {
        formatted = line.replace(/shinsal:([가-힣]+),score:([0-9.-]+)/, "신살 작용 [$1] ➔ $2점");
    } else if (line.includes("gilsin:")) {
        formatted = line.replace(/gilsin:([a-zA-Z]+),bonus:([0-9.]+)/, "길신 작용 [$1] ➔ +$2점");
    } else if (line.includes("lifecycle:")) {
        formatted = line.replace(/lifecycle:([^,]+),score:([0-9.-]+)/, "12운성 환경 [$1] ➔ $2점");
    } else if (line.includes("clash:") && !line.includes("stem_clash")) {
        formatted = line.replace(/clash:([^,]+),impact:([0-9.-]+)/, "지지 충돌 발생 [$1] ➔ $2점");
    } else if (line.includes("stem_clash:")) {
        formatted = line.replace(/stem_clash:([^,]+),penalty:([0-9.-]+)/, "천간 충돌 발생 [$1] ➔ $2점");
    } else if (line.includes("mem_dump:")) {
        formatted = line.replace(/mem_dump:([^,]+),bonus:([0-9.-]+)/, "잠재 에너지 발현 [$1] ➔ +$2점");
    } else if (line.includes("mem_corrupt:")) {
        formatted = line.replace(/mem_corrupt:([^,]+),penalty:([0-9.-]+)/, "에너지 교란 현상 [$1] ➔ $2점");
    } else if (line.includes("race_cond:")) {
        formatted = line.replace(/race_cond:([^,]+),penalty:([0-9.-]+)/, "에너지 우선순위 경합 [$1] ➔ $2점");
    } else if (line.includes("six_combo:")) {
        formatted = line.replace(/six_combo:([^,]+),bonus:([0-9.-]+)/, "육합 형성으로 파생 에너지 생성 [$1] ➔ +$2점");
    } else if (line.includes("three_combo:")) {
        formatted = line.replace(/three_combo:([^,]+),bonus:([0-9.-]+)/, "삼합 형성으로 강한 세력 구축 [$1] ➔ +$2점");
    } else if (line.includes("panic")) {
        formatted = "⚠️ 시스템 패닉: 치명적인 운세 타격 발생 " + line.split("panic")[1];
    } else if (line.includes("irq")) {
        formatted = "⛔ 인터럽트: 돌발 변수 발생 " + line.split("irq")[1];
    }
    return formatted;
}

// ── 운세(세운/월운) 섹션 ───────────────────────────

export function buildTransitMarkdown(t: TransitResult): string {
    const lines: string[] = [];
    lines.push("# 현재 운세 리포트 (세운·월운)\n");
    lines.push(`- **현재 나이(추정)**: ${t.current_age}세`);
    lines.push("");

    // 세운
    const y = t.yearly_luck;
    lines.push("## 세운 (올해의 흐름)\n");
    lines.push(`- **연도**: ${y.year}`);
    lines.push(`- **간지**: ${ganziDisplay(y.ganzi)} (${ganziHangul(y.ganzi)})`);
    lines.push(`- **천간 십신 / 지지 십신**: ${tgKr(y.stem_god)} / ${tgKr(y.branch_god)}`);
    if (y.twelve_stage) lines.push(`- **12운성**: ${y.twelve_stage}`);
    if (y.special_events?.length) lines.push(`- **특이 이벤트**: ${y.special_events.join(", ")}`);
    if (y.influence?.relations_with_natal?.length) {
        lines.push(`- **원국과의 관계**: ${y.influence.relations_with_natal.join(", ")}`);
    }
    lines.push("");

    // 월운 (이번 달)
    const m = t.monthly_luck;
    lines.push("## 월운 (이번 달의 흐름)\n");
    lines.push(`- **연-월**: ${m.year}-${String(m.month).padStart(2, "0")}`);
    lines.push(`- **간지**: ${ganziDisplay(m.ganzi)} (${ganziHangul(m.ganzi)})`);
    lines.push(`- **천간 십신 / 지지 십신**: ${tgKr(m.stem_god)} / ${tgKr(m.branch_god)}`);
    if (m.twelve_stage) lines.push(`- **12운성**: ${m.twelve_stage}`);
    if (m.influence?.relations_with_natal?.length) {
        lines.push(`- **원국과의 관계**: ${m.influence.relations_with_natal.join(", ")}`);
    }
    lines.push("");

    // 일운
    const dl = (t as unknown as Record<string, unknown>).daily_luck as Record<string, unknown> | undefined;
    if (dl) {
        lines.push("## 일운 (오늘의 흐름)\n");
        const dlGanzi = dl.ganzi as Parameters<typeof ganziDisplay>[0] | undefined;
        if (dlGanzi) lines.push(`- **간지**: ${ganziDisplay(dlGanzi)} (${ganziHangul(dlGanzi)})`);
        if (dl.stem_god) lines.push(`- **천간 십신**: ${dl.stem_god}`);
        if (dl.branch_god) lines.push(`- **지지 십신**: ${dl.branch_god}`);
        if (dl.twelve_stage) lines.push(`- **12운성**: ${dl.twelve_stage}`);
        lines.push("");
    }

    // 현재 프레임
    if (t.current_frame) {
        const f = t.current_frame;
        const scoreLabel = (f.score ?? 0) >= 80 ? "맑고 화창함" : (f.score ?? 0) >= 60 ? "가끔 구름" : (f.score ?? 0) >= 40 ? "흐림" : (f.score ?? 0) >= 20 ? "비" : "뇌우";
        lines.push("## 현재 프레임 (시뮬레이션 스냅샷)\n");
        lines.push(`- **나이**: ${f.age}세`);
        lines.push(`- **세운**: ${ganziDisplay(f.ganzi)} (${ganziHangul(f.ganzi)})`);
        lines.push(`- **대운**: ${ganziDisplay(f.major_ganzi)} (${ganziHangul(f.major_ganzi)})`);
        lines.push(`- **운세 점수**: ${(f.score ?? 0).toFixed(1)} / 100 (${scoreLabel})`);
        if (f.tags?.length) lines.push(`- **운세 태그**: ${f.tags.map(tagToStr).filter(Boolean).join(", ")}`);
        lines.push("");

        // ESIL 트레이스
        if (f.esil_trace?.trim()) {
            lines.push("### ESIL 트레이스 (운세 계산 상세)\n");
            const traceLines = f.esil_trace.trim().split("\n").filter(Boolean);
            for (const tl of traceLines) {
                const formatted = formatEsilLine(tl.trim());
                lines.push(`- ${formatted}`);
            }
            lines.push("");
        }
    }

    // 월별 운세 12개 (monthly_lucks)
    const monthlyAll = (t as unknown as Record<string, unknown>).monthly_lucks as typeof t.monthly_luck[] | undefined;
    if (Array.isArray(monthlyAll) && monthlyAll.length > 0) {
        lines.push("## 월별 운세 (이번 해 12개월)\n");
        lines.push("| 연-월 | 간지 | 천간 십신 | 지지 십신 | 12운성 | 원국 관계 |");
        lines.push("|---|---|---|---|---|---|");
        for (const ml of monthlyAll) {
            const ganziStr = `${ganziDisplay(ml.ganzi)} (${ganziHangul(ml.ganzi)})`;
            const relations = ml.influence?.relations_with_natal?.join(", ") ?? "—";
            lines.push(`| ${ml.year}-${String(ml.month).padStart(2, "0")} | ${ganziStr} | ${tgKr(ml.stem_god)} | ${tgKr(ml.branch_god)} | ${ml.twelve_stage ?? "—"} | ${relations} |`);
        }
        lines.push("");
    }

    // 주변 부하 진단
    if (Array.isArray(t.nearby_diagnostics) && t.nearby_diagnostics.length) {
        lines.push("## 주변 부하 진단 (근처 시기 경고/전략)\n");
        lines.push("| 나이 | 상태 | 원인 | 대응 전략 |");
        lines.push("|---:|---|---|---|");
        for (const d of t.nearby_diagnostics) {
            lines.push(`| ${d.age}세 | ${d.status} | ${d.reason} | ${d.strategy} |`);
        }
        lines.push("");
    }

    return lines.join("\n");
}

// ── 궁합 섹션 ────────────────────────────────────────

export function buildCompatibilityMarkdown(comp: CompReport): string {
    const lines: string[] = [];
    lines.push("# 궁합 분석 리포트\n");

    // 사주 궁합
    if (comp.saju) {
        const sj = comp.saju;
        const scoreColor = sj.sync_score > 70 ? "🟢" : sj.sync_score > 40 ? "🟡" : "🔴";
        lines.push("## 사주 궁합 결과\n");
        lines.push(`- **궁합 점수**: ${scoreColor} ${sj.sync_score?.toFixed(0) ?? "—"} / 100`);
        lines.push("");

        if (sj.synergies?.length) {
            lines.push("### 시너지 (긍정 작용)\n");
            for (const s of sj.synergies) lines.push(`- ✅ ${s}`);
            lines.push("");
        }
        if (sj.conflicts?.length) {
            lines.push("### 충돌 (부정 작용)\n");
            for (const s of sj.conflicts) lines.push(`- ⚡ ${s}`);
            lines.push("");
        }
        if (sj.deadlocks?.length) {
            lines.push("### 갈등 요소\n");
            for (const s of sj.deadlocks) lines.push(`- ⚠️ ${s}`);
            lines.push("");
        }
    }

    // 베딕 Ashta Kuta
    if (comp.vedic) {
        const vd = comp.vedic;
        const totalIcon = (vd.total_score ?? 0) >= 26 ? "🟢" : (vd.total_score ?? 0) >= 18 ? "🟡" : "🔴";
        lines.push("## 베딕 궁합 (Ashta Kuta)\n");
        lines.push(`- **총점**: ${totalIcon} ${vd.total_score?.toFixed(1) ?? "—"} / 36`);
        if (vd.message) lines.push(`- **종합 판정**: ${vd.message}`);
        lines.push("");

        lines.push("| 항목 | 점수 | 만점 | 비율 |");
        lines.push("|---|---:|---:|---:|");
        for (const [key, label] of Object.entries(ASHTA_LABELS)) {
            const val = (vd as unknown as Record<string, number | undefined>)[key];
            const max = ASHTA_MAX[key] ?? 1;
            const pct = val != null ? ((val / max) * 100).toFixed(0) : "—";
            lines.push(`| ${label} | ${val?.toFixed(1) ?? "—"} | ${max} | ${typeof val === "number" ? pct + "%" : "—"} |`);
        }
        lines.push("");
    }

    return lines.join("\n");
}

// ── 운명 티어 섹션 ──────────────────────────────────────

export function buildDestinyTierMarkdown(
    sajuReport: SajuAnalysisResult | null,
    vedicReport: VedicAnalysisResult | null,
    transitReport: TransitResult | null | undefined,
    tierResult?: TierResult | null,
): string {
    const result = tierResult ?? computeTierResult(sajuReport, vedicReport, transitReport ?? null);
    if (!result) return "";

    const lines: string[] = [];
    const hasTransit = !!transitReport?.current_frame;

    lines.push("# 운명 티어 요약\n");
    lines.push(`- **운명 티어**: ${result.destinyTier.grade} · ${result.destinyTier.label}`);
    lines.push(`- **잠재력 티어**: ${result.potentialTier.grade} · ${result.potentialTier.label}`);
    if (hasTransit) {
        lines.push(`- **원국 점수**: ${Math.round(result.natalScore)}`);
        lines.push(`- **현재 시점 점수**: ${Math.round(result.currentScore)}`);
    }
    lines.push(`- **종합 점수**: ${Math.round(result.destinyScore)} / 100`);
    lines.push("");

    lines.push("## 강점\n");
    if (result.strengths.length > 0) {
        for (const s of result.strengths) lines.push(`- ${s}`);
    } else {
        lines.push("- (추가 분석 필요)");
    }
    lines.push("");

    lines.push("## 약점\n");
    if (result.weaknesses.length > 0) {
        for (const w of result.weaknesses) lines.push(`- ${w}`);
    } else {
        lines.push("- 특별한 주의 시점 없음");
    }
    lines.push("");

    if (result.domainTiers.length > 0) {
        lines.push("## 분야별 티어\n");
        lines.push("| 영역 | 하우스 | 티어 |");
        lines.push("|---|---|---|");
        for (const d of result.domainTiers) {
            lines.push(`| ${d.domain} | H${d.house} | ${d.tier} |`);
        }
        lines.push("");
    }

    const potentialHigher = result.potentialScore > result.destinyScore;
    lines.push("## 인사이트\n");
    if (potentialHigher) {
        lines.push("- 잠재력이 운명 티어보다 높아 **성장 여지가 큽니다**. 용신·골든타임·우수 요가를 적극 활용하세요.");
    } else {
        lines.push("- 현재 흐름을 잘 활용하고 있습니다.");
    }
    lines.push(`- ${result.destinyTier.desc}`);
    lines.push("");

    return lines.join("\n");
}

// ── 외부 API ──────────────────────────────────────

export function buildFullAnalysisMarkdown(
    sajuReport: SajuAnalysisResult | null,
    vedicReport: VedicAnalysisResult | null,
    transitReport: TransitResult | null = null,
    compReport?: CompReport | null,
): string {
    const parts: string[] = [];

    parts.push("# Eon 통합 분석 리포트\n");
    parts.push(`- 생성 시각: ${new Date().toLocaleString()}`);
    parts.push("- 이 문서는 앱 화면의 분석 결과를 복사/공유하기 쉬운 형태로 정리한 것입니다.");
    parts.push("");

    const destinyTierMd = buildDestinyTierMarkdown(sajuReport, vedicReport, transitReport);
    if (destinyTierMd) {
        parts.push(destinyTierMd);
        parts.push(SEP);
    }

    if (sajuReport) {
        parts.push(buildSajuMarkdown(sajuReport));
    }

    if (transitReport) {
        if (parts.length > 0) parts.push(SEP);
        parts.push(buildTransitMarkdown(transitReport));
    }

    if (vedicReport) {
        if (parts.length > 0) parts.push(SEP);
        parts.push(buildVedicMarkdown(vedicReport));
    }

    if (compReport) {
        parts.push(SEP);
        parts.push(buildCompatibilityMarkdown(compReport));
    }

    return parts.join("\n");
}
