import { ganziDisplay, ganziHangul } from "./ganzi";
import { SIGN_NAMES, VARGA_DEFS } from "../constants";
import type { SajuAnalysisResult } from "../types";
import type { VedicAnalysisResult } from "../types";
import type { TransitResult } from "../types";
import { computeTierResult, type TierResult } from "./tierScore";
import { getNakshatraInfo } from "./nakshatra";
import { formatSiderealPosition, buildNakshatraMarkdownRows } from "./vedicFormat";

const SEP = "\n---\n";

const fmtIsoShort = (iso: string | null | undefined) => {
    if (!iso) return "—";
    return iso.replace("T", " ").replace("Z", " UTC");
};

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
    lines.push("## 신강·신약 분석\n");
    lines.push(`- **일간**: ${st.day_master} (나의 기본 기운/성향의 중심)`);
    lines.push(`- **신강/신약**: ${st.strength_type} (점수: ${st.strength_score}, 체력/버팀의 강도)`);
    lines.push(`- **득령(得令)**: ${st.deuk_ryeong.acquired ? "✅" : "❌"} (계절의 도움)`);
    lines.push(`- **득지(得地)**: ${st.deuk_ji.acquired ? "✅" : "❌"} (뿌리/근거의 도움)`);
    lines.push(`- **득시(得時)**: ${st.deuk_si.acquired ? "✅" : "❌"} (시간대의 도움)`);
    lines.push(`- **득세 지지비율**: ${(st.deuk_se.support_ratio * 100).toFixed(1)}%`);
    lines.push("");

    // 용신
    const y = r.yongshin;
    lines.push("## 용신 분석 (균형을 맞추는 방향)\n");
    lines.push(`- **주 용신**: ${y.primary}`);
    lines.push(`- **보조 용신**: ${y.assistant}`);
    if (y.recommendations?.length) {
        for (const rec of y.recommendations) {
            lines.push(`- ${rec.yongshin_type} (${rec.element}): ${rec.reason}`);
        }
    }
    lines.push("");

    // 격국
    lines.push("## 격국 (구조/패턴)\n");
    lines.push(`- **격국명**: ${r.structure.structure}`);
    lines.push(`- **사유**: ${r.structure.reason}`);
    if (r.structure.projected_stem) lines.push(`- **투출 천간**: ${r.structure.projected_stem}`);
    lines.push("");

    // 신살
    if (r.spirit_markers?.markers?.length) {
        lines.push("## 신살 (특수 패턴 태그)\n");
        lines.push("| 신살명 | 위치 | 천간/지지 |");
        lines.push("|---|---|---|");
        for (const m of r.spirit_markers.markers) {
            lines.push(`| ${m.marker} | ${m.position} | ${m.is_stem ? "천간" : "지지"} |`);
        }
        lines.push("");
    }

    // 대운
    if (r.major_luck?.cycles?.length) {
        lines.push("## 대운 (10년 단위 큰 흐름)\n");
        lines.push(`- **순행/역행**: ${r.major_luck.direction} (대운이 흘러가는 방향)`);
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
        if (s.qi_topology.bottleneck) lines.push(`- **흐름 정체 오행**: ${s.qi_topology.bottleneck}`);
        lines.push("\n| 오행 | 용량 | 산출 |");
        lines.push("|---|---|---|");
        for (const n of s.qi_topology.nodes ?? []) {
            const el = typeof n.element === "string" ? n.element : ((n.element as { hangul?: string })?.hangul ?? "—");
            lines.push(`| ${el} | ${n.capacity.toFixed(1)} | ${n.output.toFixed(1)} |`);
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
            const tags = (v.tags ?? []).slice(0, 8).join(", ");
            lines.push(`| ${Number.isFinite(v.crash_score) ? v.crash_score.toFixed(1) : "—"} | ${major} | ${yearly} | ${v.vulnerability_type ?? "—"} | ${tags || "—"} |`);
        }
        lines.push("");
    }

    // 정밀 분석(power)
    if (r.power) {
        lines.push("## 정밀 분석 요약 (오행·십신 분포)\n");
        lines.push(`- **우세 오행**: ${r.power.dominant_element} (가장 강하게 나타나는 기운)`);
        lines.push(`- **우세 십신**: ${r.power.dominant_ten_god} (행동/관계 패턴의 중심 역할)`);
        if (Array.isArray(r.power.element_scores) && r.power.element_scores.length) {
            lines.push("\n### 오행 점수\n");
            lines.push("| 오행 | 비중(%) | 점수 |");
            lines.push("|---|---:|---:|");
            for (const [el, pct, score] of r.power.element_scores) {
                lines.push(`| ${el} | ${(pct ?? 0).toFixed(1)} | ${(score ?? 0).toFixed(1)} |`);
            }
            lines.push("");
        }
        if (Array.isArray(r.power.ten_god_scores) && r.power.ten_god_scores.length) {
            lines.push("### 십신 점수\n");
            lines.push("| 십신 | 비중(%) | 점수 |");
            lines.push("|---|---:|---:|");
            for (const [tg, pct, score] of r.power.ten_god_scores) {
                lines.push(`| ${tg} | ${(pct ?? 0).toFixed(1)} | ${(score ?? 0).toFixed(1)} |`);
            }
            lines.push("");
        }
    }

    // 십신(기둥별)
    if (r.ten_gods) {
        lines.push("## 십신 배치 (기둥별 역할)\n");
        lines.push("| 위치 | 천간 | 지지 |");
        lines.push("|---|---|---|");
        lines.push(`| 년주 | ${r.ten_gods.year_stem} | ${r.ten_gods.year_branch} |`);
        lines.push(`| 월주 | ${r.ten_gods.month_stem} | ${r.ten_gods.month_branch} |`);
        lines.push(`| 일주 | ${r.ten_gods.day_stem} | ${r.ten_gods.day_branch} |`);
        lines.push(`| 시주 | ${r.ten_gods.hour_stem} | ${r.ten_gods.hour_branch} |`);
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
            const tags = (f.tags ?? []).join(", ");
            lines.push(`| ${f.age} | ${yearly} | ${major} | ${(f.score ?? 0).toFixed(1)} | ${tags || "—"} |`);
        }
        lines.push("");

        const topFrames = [...r.simulation_frames].sort((a, b) => (a.score ?? 9999) - (b.score ?? 9999)).slice(0, 8);
        lines.push("### ESIL 트레이스 (상위 위험 프레임 일부)\n");
        for (const f of topFrames) {
            lines.push(`- **${f.age}세** (${ganziDisplay(f.ganzi)} / 대운 ${ganziDisplay(f.major_ganzi)}) 점수 ${(f.score ?? 0).toFixed(1)}`);
            lines.push("```");
            lines.push((f.esil_trace ?? "").trim() || "—");
            lines.push("```");
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

    // 다샤 타임라인
    if (Array.isArray(r.dasha_timeline) && r.dasha_timeline.length > 0) {
        lines.push("## 다샤 타임라인 (삶의 시기 구분)\n");
        lines.push("| 주인(Planet) | 시작 | 종료 | 하위 기간 수 |");
        lines.push("|---|---|---|---:|");
        for (const d of r.dasha_timeline) {
            lines.push(`| ${d.lord} | ${fmtIsoShort(d.start_time)} | ${fmtIsoShort(d.end_time)} | ${(d.sub_dashas ?? []).length} |`);
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

    // 행성 위치 — D1 낙샤트라 전체 리포트
    lines.push("## D1 낙샤트라 리포트 (전체 행성)\n");
    lines.push("> D1 낙샤트라는 본 차트 기준입니다.\n");
    const allPos = [...(c?.planets ?? []), ...(c?.ascendant ? [c.ascendant] : [])];
    const d1Report = v.varga_nakshatra_reports?.reports?.["rasi"];
    if (d1Report?.rows?.length) {
        // 백엔드 데이터 우선 사용 (Rust 계산과 완전 일치)
        const mdRows = buildNakshatraMarkdownRows(d1Report.rows, false);
        for (const row of mdRows) lines.push(row);
    } else {
        // 폴백: 프론트엔드 계산 (Swiss Ephemeris 정밀도 없음)
        const rows = allPos.map(p => {
            const name = p.planet ?? "ASC";
            const ni = getNakshatraInfo(p.sidereal_deg ?? 0);
            return {
                planet: name,
                position_str: formatSiderealPosition(p.sidereal_deg ?? 0),
                nakshatra_name: ni.name,
                pada: ni.pada,
                pada_range: ni.range,
                nakshatra_lord: ni.lord,
                pada_lord: ni.padaLord,
                deity: ni.deity,
                purpose: ni.purpose,
                is_retrograde: p.is_retrograde,
                is_combust: p.is_combust,
            };
        });
        const mdRows = buildNakshatraMarkdownRows(rows, false);
        for (const row of mdRows) lines.push(row);
    }
    lines.push("");

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

    // 분할 차트 낙샤트라 리포트 (D-Charts)
    lines.push("## 분할 차트 낙샤트라 리포트 (Varga D-Charts)\n");
    lines.push("> 분할 차트 낙샤트라는 해당 분할 좌표 기준입니다.\n");
    const vargaReportsMap = v.varga_nakshatra_reports?.reports;
    if (vargaReportsMap && Object.keys(vargaReportsMap).length > 0) {
        // 백엔드 Varga 낙샤트라 리포트 — 전체 8(+사인·하우스) 컬럼
        for (const vargaDef of VARGA_DEFS) {
            const rep = vargaReportsMap[vargaDef.id];
            if (!rep?.rows?.length) continue;
            const lagna = rep.lagna_rasi ? ` (라그나: ${SIGN_NAMES[rep.lagna_rasi] ?? rep.lagna_rasi})` : "";
            lines.push(`### ${rep.varga_label}${lagna}`);
            const mdRows = buildNakshatraMarkdownRows(rep.rows, true);
            for (const row of mdRows) lines.push(row);
            lines.push("");
        }
    } else {
        // 폴백: 라시(Sign)만 2컬럼
        for (const varga of VARGA_DEFS) {
            lines.push(`### ${varga.label}: ${varga.name}`);
            lines.push("| 행성 | 라시 (Sign) |");
            lines.push("|---|---|");
            for (const p of allPos) {
                const name = p.planet ?? "ASC";
                const rasiIdx = (p as Record<string, unknown>)[varga.key];
                if (rasiIdx !== undefined && rasiIdx !== null) {
                    const rasiName = typeof rasiIdx === "number" ? (SIGN_NAMES[rasiIdx] ?? rasiIdx) : rasiIdx;
                    lines.push(`| ${name} | ${rasiName} |`);
                } else {
                    lines.push(`| ${name} | — |`);
                }
            }
            lines.push("");
        }
    }

    return lines.join("\n");
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
    lines.push(`- **천간 십신 / 지지 십신**: ${y.stem_god} / ${y.branch_god}`);
    if (y.twelve_stage) lines.push(`- **12운성**: ${y.twelve_stage}`);
    if (y.special_events?.length) lines.push(`- **특이 이벤트**: ${y.special_events.join(", ")}`);
    if (y.influence?.relations_with_natal?.length) {
        lines.push(`- **원국과의 관계**: ${y.influence.relations_with_natal.join(", ")}`);
    }
    lines.push("");

    // 월운
    const m = t.monthly_luck;
    lines.push("## 월운 (이번 달의 흐름)\n");
    lines.push(`- **연-월**: ${m.year}-${String(m.month).padStart(2, "0")}`);
    lines.push(`- **간지**: ${ganziDisplay(m.ganzi)} (${ganziHangul(m.ganzi)})`);
    lines.push(`- **천간 십신 / 지지 십신**: ${m.stem_god} / ${m.branch_god}`);
    if (m.twelve_stage) lines.push(`- **12운성**: ${m.twelve_stage}`);
    if (m.influence?.relations_with_natal?.length) {
        lines.push(`- **원국과의 관계**: ${m.influence.relations_with_natal.join(", ")}`);
    }
    lines.push("");

    // 현재 프레임
    if (t.current_frame) {
        const f = t.current_frame;
        lines.push("## 현재 프레임 (시뮬레이션 스냅샷)\n");
        lines.push(`- **나이**: ${f.age}세`);
        lines.push(`- **세운**: ${ganziDisplay(f.ganzi)} (${ganziHangul(f.ganzi)})`);
        lines.push(`- **대운**: ${ganziDisplay(f.major_ganzi)} (${ganziHangul(f.major_ganzi)})`);
        lines.push(`- **점수**: ${(f.score ?? 0).toFixed(1)}`);
        if (f.tags?.length) lines.push(`- **태그**: ${f.tags.join(", ")}`);
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

    return parts.join("\n");
}
