import { ganziDisplay, ganziHangul } from "./ganzi";
import { SIGN_NAMES } from "../constants";
import type { SajuAnalysisResult } from "../types";
import type { VedicAnalysisResult } from "../types";

const SEP = "\n---\n";

// ── 사주 섹션 ────────────────────────────────────────

export function buildSajuMarkdown(s: SajuAnalysisResult): string {
    const r = s.report;
    const lines: string[] = [];

    lines.push("# 사주명리 분석 리포트\n");

    // 보정 시간 / DST
    lines.push(`**보정된 출생 시간**: ${s.corrected_time}`);
    if (s.is_dst) lines.push(`**서머타임(DST)**: 적용 (${s.dst_offset_hours ?? 1}시간)`);
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
    lines.push("## 신강·신약 분석\n");
    lines.push(`- **일간**: ${st.day_master}`);
    lines.push(`- **신강/신약**: ${st.strength_type} (점수: ${st.strength_score})`);
    lines.push(`- **득령**: ${st.deuk_ryeong.acquired ? "✅" : "❌"}`);
    lines.push(`- **득지**: ${st.deuk_ji.acquired ? "✅" : "❌"}`);
    lines.push(`- **득시**: ${st.deuk_si.acquired ? "✅" : "❌"}`);
    lines.push(`- **득세 지지비율**: ${(st.deuk_se.support_ratio * 100).toFixed(1)}%`);
    lines.push("");

    // 용신
    const y = r.yongshin;
    lines.push("## 용신 분석\n");
    lines.push(`- **주 용신**: ${y.primary}`);
    lines.push(`- **보조 용신**: ${y.assistant}`);
    if (y.recommendations?.length) {
        for (const rec of y.recommendations) {
            lines.push(`- ${rec.yongshin_type} (${rec.element}): ${rec.reason}`);
        }
    }
    lines.push("");

    // 격국
    lines.push("## 격국\n");
    lines.push(`- **격국명**: ${r.structure.structure}`);
    lines.push(`- **사유**: ${r.structure.reason}`);
    if (r.structure.projected_stem) lines.push(`- **투출 천간**: ${r.structure.projected_stem}`);
    lines.push("");

    // 신살
    if (r.spirit_markers?.markers?.length) {
        lines.push("## 신살\n");
        lines.push("| 신살명 | 위치 | 천간/지지 |");
        lines.push("|---|---|---|");
        for (const m of r.spirit_markers.markers) {
            lines.push(`| ${m.marker} | ${m.position} | ${m.is_stem ? "천간" : "지지"} |`);
        }
        lines.push("");
    }

    // 대운
    if (r.major_luck?.cycles?.length) {
        lines.push("## 대운\n");
        lines.push(`- **순행/역행**: ${r.major_luck.direction}`);
        lines.push(`- **대운 시작**: ${r.major_luck.start_age}세\n`);
        lines.push("| 간지 | 시작 나이 | 종료 나이 | 천간 십신 | 지지 십신 |");
        lines.push("|---|---|---|---|---|");
        for (const c of r.major_luck.cycles) {
            lines.push(`| ${ganziDisplay(c.ganzi)} | ${c.start_age}세 | ${c.end_age}세 | ${c.stem_god} | ${c.branch_god} |`);
        }
        lines.push("");
    }

    // 골든타임
    if (r.golden_time) {
        lines.push("## 골든타임\n");
        lines.push(`- **기간**: ${r.golden_time.start_age}세 ~ ${r.golden_time.end_age}세`);
        lines.push(`- **평균 점수**: ${r.golden_time.average_score.toFixed(1)}`);
        lines.push(`- **설명**: ${r.golden_time.description}`);
        lines.push("");
    }

    // 운명 복잡도
    if (s.entropy) {
        lines.push("## 운명 복잡도\n");
        lines.push(`- **점수**: ${s.entropy.score.toFixed(3)}`);
        lines.push(`- **등급**: ${s.entropy.level}`);
        if (s.entropy.description) lines.push(`- **설명**: ${s.entropy.description}`);
        if (s.entropy.unpacker_element) lines.push(`- **해결 열쇠 오행**: ${s.entropy.unpacker_element}`);
        lines.push("");
    }

    // 에너지 흐름
    if (s.qi_topology) {
        lines.push("## 오행 에너지 흐름\n");
        lines.push(`- **전체 순환 지수**: ${(s.qi_topology.throughput * 100).toFixed(0)}%`);
        if (s.qi_topology.bottleneck) lines.push(`- **흐름 정체 오행**: ${s.qi_topology.bottleneck}`);
        lines.push("\n| 오행 | 용량 | 산출 |");
        lines.push("|---|---|---|");
        for (const n of s.qi_topology.nodes ?? []) {
            const el = typeof n.element === "string" ? n.element : ((n.element as any)?.hangul ?? "—");
            lines.push(`| ${el} | ${n.capacity.toFixed(1)} | ${n.output.toFixed(1)} |`);
        }
        lines.push("");
    }

    // 위기
    lines.push(`**시뮬레이션 위기 발생 횟수**: ${s.crash_count}번\n`);

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

    return lines.join("\n");
}

// ── 베딕 섹션 ────────────────────────────────────────

export function buildVedicMarkdown(v: VedicAnalysisResult): string {
    const r = v.report;
    const c = v.chart;
    const lines: string[] = [];

    lines.push("# 베딕 점성학 분석 리포트\n");

    // 카라카
    lines.push("## 주요 카라카\n");
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
            lines.push(`| ${roleKr[(k as any).role] ?? (k as any).role} | ${(k as any).planet} | ${((k as any).degree_in_rasi ?? 0).toFixed(2)}° |`);
        }
        lines.push("");
    }

    // 대운/낙샤트라/사데사티
    lines.push("## 현재 대운 & 주요 정보\n");
    lines.push(`- **대운 (Dasha)**: ${r.dasha_focus}`);
    lines.push(`- **낙샤트라**: ${r.nakshatra_info}`);
    lines.push(`- **사데사티**: ${r.sade_sati}`);
    lines.push(`- **전체 차트 강도**: ${Math.round(r.overall_strength_score)}/600`);
    lines.push("");

    // 하우스 요약
    if (r.house_summary?.length) {
        lines.push("## 하우스별 에너지\n");
        lines.push("| 하우스 | 점수 | 등급 |");
        lines.push("|---|---|---|");
        for (const h of r.house_summary) {
            lines.push(`| ${h.house} | ${Math.round(h.total_score)} | ${h.rating} |`);
        }
        lines.push("");
    }

    // BhavaStrength
    if (c?.bhava_strengths?.length) {
        lines.push("## 하우스 강점 상세\n");
        lines.push("| 하우스 | 주인 행성 힘 | 방위 힘 | 시선 영향 | 총점 |");
        lines.push("|---|---|---|---|---|");
        for (const b of c.bhava_strengths) {
            lines.push(`| ${(b as any).house} | ${((b as any).lord_score ?? 0).toFixed(1)} | ${((b as any).dig_score ?? 0).toFixed(1)} | ${((b as any).drishti_score ?? 0).toFixed(1)} | ${((b as any).total_score ?? 0).toFixed(1)} |`);
        }
        lines.push("");
    }

    // SAV
    if (c?.sav?.points) {
        const pts = (c.sav as any).points;
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
            const s6 = (score as any)?.shadvarga_score ?? 0;
            const s16 = (score as any)?.shodashavarga_score ?? 0;
            lines.push(`| ${planet} | ${s6.toFixed(1)} | ${s16.toFixed(1)} | ${((s6 + s16) / 2).toFixed(1)} |`);
        }
        lines.push("");
    }

    // Aspects
    if (c?.aspects?.length) {
        lines.push("## 행성 시선 (Drishti)\n");
        lines.push("| 행성 | 바라보는 하우스 |");
        lines.push("|---|---|");
        for (const a of c.aspects) {
            lines.push(`| ${(a as any).aspecting_planet} | ${((a as any).aspected_houses ?? []).map((h: number) => `H${h}`).join(", ")} |`);
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

    // 행성 위치
    lines.push("## 행성 위치 (사이드리얼)\n");
    lines.push("| 행성 | 라시 | 낙샤트라 | 파다 | 역행 | 소각 |");
    lines.push("|---|---|---|---|---|---|");
    const allPos = [...(c?.planets ?? []), ...(c?.ascendant ? [c.ascendant] : [])];
    for (const p of allPos) {
        const name = (p as any).planet ?? "ASC";
        const rasi = (SIGN_NAMES as any)?.[(p as any).rasi] ?? (p as any).rasi;
        lines.push(`| ${name} | ${rasi} | ${(p as any).nakshatra} | ${(p as any).pada} | ${(p as any).is_retrograde ? "℞" : ""} | ${(p as any).is_combust ? "☀" : ""} |`);
    }
    lines.push("");

    // Panchanga
    if (c?.panchanga) {
        const pan = c.panchanga;
        lines.push("## 판창가 (Panchanga)\n");
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

// ── 외부 API ──────────────────────────────────────

export function buildFullAnalysisMarkdown(
    sajuReport: SajuAnalysisResult | null,
    vedicReport: VedicAnalysisResult | null,
): string {
    const parts: string[] = [];

    if (sajuReport) {
        parts.push(buildSajuMarkdown(sajuReport));
    }

    if (vedicReport) {
        if (parts.length > 0) parts.push(SEP);
        parts.push(buildVedicMarkdown(vedicReport));
    }

    return parts.join("\n");
}
