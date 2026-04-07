import { ganziDisplay, ganziHangul } from "./ganzi";
import {
    SIGN_NAMES,
    STEM_INFO, ELEMENT_INFO, STRENGTH_INFO, TENGOD_INFO,
    STRUCTURE_INFO, SPIRIT_INFO, PILLAR_POS_INFO, YONGSHIN_TYPE_INFO,
} from "../constants";
import type { SajuAnalysisResult, VedicAnalysisResult, TransitResult } from "../types";
import { type TierResult } from "./tierScore";
import { formatSiderealPosition, buildNakshatraMarkdownRows } from "./vedicFormat";

/** 영문 태그명 → 한글 변환 맵 */
const TAG_KR: Record<string, string> = {
    CoreLuck: "핵심길운",
    GoodLuck: "길운",
    BadLuck: "흉운",
    Interrupt: "돌발변수",
    AuspiciousSpirit: "길신",
    BranchClash: "지지충",
    StemClash: "천간충",
    Punishment: "지지형",
    Harm: "지지해",
    SixCombination: "육합",
    TripleCombination: "삼합",
    SeasonalCombination: "방합",
    EscapedVoidClash: "공망탈출(충)",
    EscapedVoidSixCombo: "공망탈출(합)",
    EatingGodProducesWealth: "식신생재",
    HurtingOfficerMeetsOfficer: "상관견관",
    StrengthType: "신강·신약",
    DeukJi: "득지",
    Custom: "", // 노이즈 태그 — 출력 생략
};

/** TraceTag / LifeFrame.tags 가 문자열 혹은 { Key: null } 객체로 올 수 있음 — 항상 한글 문자열로 변환 */
const tagToStr = (tag: unknown): string => {
    let key = "";
    if (typeof tag === "string") key = tag;
    else if (tag && typeof tag === "object") {
        const keys = Object.keys(tag as object);
        key = keys.length > 0 ? keys[0] : "";
    } else {
        key = String(tag ?? "");
    }
    // TAG_KR 맵에 있으면 한글 사용 (빈 문자열이면 필터링됨)
    if (Object.prototype.hasOwnProperty.call(TAG_KR, key)) return TAG_KR[key];
    return key;
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
 */


// ── 사주 섹션 ────────────────────────────────────────

export function buildSajuMarkdown(s: SajuAnalysisResult): string {
    const r = s.report;
    const lines: string[] = [];

    lines.push("# 사주명리 분석 리포트\n");

    // 보정 시간 / DST
    lines.push(`**보정된 출생 시간**: ${s.meta.correctedTime} (입력 시간/시간대 보정을 반영한 기준 시간)`);
    if (s.meta.isDst) lines.push(`**서머타임(DST)**: 적용 (${s.meta.dstOffsetHours ?? 1}시간, 지역 규칙 기반 보정)`);
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
    const dayMasterKr = stemKr(st.dayMaster);
    const strengthTypeKr = STRENGTH_INFO[st.strengthType] ?? st.strengthType;
    lines.push("## 신강·신약 분석\n");
    lines.push(`- **일간**: ${dayMasterKr} (나의 기본 기운/성향의 중심)`);
    lines.push(`- **신강/신약**: ${strengthTypeKr} (점수: ${Math.round(st.strengthScore * 100) / 100}, 체력/버팀의 강도)`);
    lines.push(`- **득령(得令)**: ${st.deukRyeong.acquired ? "✅" : "❌"} (계절의 도움)`);
    lines.push(`- **득지(得地)**: ${st.deukJi.acquired ? "✅" : "❌"} (뿌리/근거의 도움)`);
    lines.push(`- **득시(得時)**: ${st.deukSi.acquired ? "✅" : "❌"} (시간대의 도움)`);
    const rawSr = st.deukSe?.supportRatio ?? 0;
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
            const ynTypeKr = YONGSHIN_TYPE_INFO[rec.yongshinType] ?? rec.yongshinType;
            lines.push(`- **${ynTypeKr}** (${elKr(rec.element)}): ${rec.summary}`);
            lines.push(`  - *설명*: ${rec.description}`);
            if (rec.reasons?.length) {
                lines.push(`  - *근거*: ${rec.reasons.join(", ")}`);
            }
        }
    }
    lines.push("");

    // 격국
    const structInfo = STRUCTURE_INFO[r.structure.structure];
    lines.push("## 격국 (구조/패턴)\n");
    lines.push(`- **격국명**: ${structInfo ? `${structInfo.hangul}(${structInfo.hanja})` : r.structure.structure}`);
    lines.push(`- **요약**: ${r.structure.summary}`);
    lines.push(`- **설명**: ${r.structure.description}`);
    if (r.structure.reasons?.length) {
        lines.push(`- **근거**: ${r.structure.reasons.join(", ")}`);
    }
    if (r.structure.projectedStem) lines.push(`- **투출 천간**: ${stemKr(r.structure.projectedStem)}`);
    lines.push("");

    // 신살 (Spirit Markers)
    if (r.spiritMarkers?.mappedMarkers?.length) {
        lines.push("## 신살 분석 (특수 패턴 태그)\n");
        for (const m of r.spiritMarkers.mappedMarkers) {
            const emoji = m.level === "Auspicious" ? "✨" : m.level === "Caution" ? "⚠️" : "•";
            const mKr = SPIRIT_INFO[m.marker]?.hangul ?? m.marker;
            const posKr = PILLAR_POS_INFO[m.position] ?? m.position;

            lines.push(`### ${emoji} ${mKr} (${posKr})`);
            lines.push(`- **요약**: ${m.summary}`);
            lines.push(`- **설명**: ${m.description}`);
            if (m.reasons?.length) {
                lines.push(`- **근거**: ${m.reasons.join(", ")}`);
            }
            lines.push("");
        }
    }

    // 대운
    const mlDir: Record<string, string> = { Forward: "순행 (順行)", Reverse: "역행 (逆行)" };
    if (r.majorLuck?.cycles?.length) {
        lines.push("## 대운 (10년 단위 큰 흐름)\n");
        lines.push(`- **순행/역행**: ${mlDir[r.majorLuck.direction] ?? r.majorLuck.direction}`);
        lines.push(`- **대운 시작**: ${r.majorLuck.startAge}세\n`);
        lines.push("| 간지 | 시작 나이 | 종료 나이 | 천간 십신 | 지지 십신 |");
        lines.push("|---|---|---|---|---|");
        for (const c of r.majorLuck.cycles) {
            lines.push(`| ${ganziDisplay(c.ganzi)} | ${c.startAge}세 | ${c.endAge}세 | ${tgKr(c.stemGod)} | ${tgKr(c.branchGod)} |`);
        }
        lines.push("");
    }

    // 골든타임
    if (r.goldenTime) {
        lines.push("## 골든타임 (점수 흐름상 ‘좋게 풀리기 쉬운’ 구간)\n");
        lines.push(`- **기간**: ${r.goldenTime.startAge}세 ~ ${r.goldenTime.endAge}세`);
        lines.push(`- **평균 점수**: ${r.goldenTime.averageScore.toFixed(1)}`);
        lines.push(`- **설명**: ${r.goldenTime.description}`);
        lines.push("");
    }

    // 운명 복잡도
    if (s.entropy) {
        lines.push("## 운명 복잡도 (삶의 변수/꼬임의 정도)\n");
        lines.push(`- **점수**: ${s.entropy.score.toFixed(3)} (높을수록 변수가 많고 복합적일 가능성)`);
        lines.push(`- **등급**: ${s.entropy.level}`);
        if (s.entropy.description) lines.push(`- **설명**: ${s.entropy.description}`);
        if (s.entropy.unpackerElement) lines.push(`- **해결 열쇠 오행**: ${s.entropy.unpackerElement}`);
        lines.push("");
    }

    // 에너지 흐름
    if (s.qiTopology) {
        lines.push("## 오행 에너지 흐름 (상생/상극의 순환 상태)\n");
        lines.push(`- **전체 순환 지수**: ${(s.qiTopology.throughput * 100).toFixed(0)}% (높을수록 흐름이 막힘 없이 원활)`);
        if (s.qiTopology.bottleneck) {
            const bnEl = typeof s.qiTopology.bottleneck === "string" ? elKr(s.qiTopology.bottleneck) : s.qiTopology.bottleneck;
            lines.push(`- **흐름 정체 오행**: ${bnEl}`);
        }
        lines.push("\n| 오행 | 용량 | 산출 |");
        lines.push("|---|---|---|");
        for (const n of s.qiTopology.nodes ?? []) {
            const elKey = typeof n.element === "string" ? n.element : ((n.element as any)?.english ?? "");
            const elLabel = ELEMENT_INFO[elKey] ? `${ELEMENT_INFO[elKey].hangul}(${ELEMENT_INFO[elKey].hanja})` : ((n.element as any)?.hangul ?? elKey);
            lines.push(`| ${elLabel} | ${n.capacity.toFixed(1)} | ${n.output.toFixed(1)} |`);
        }
        lines.push("");
    }

    // 인생 갈림길 분석 (Cyclomatic Complexity)
    if (s.complexity) {
        lines.push("## 인생 갈림길 분석 (중요 분기점 요약)\n");
        lines.push(`- **갈림길 지수**: ${s.complexity.cyclomaticComplexity} (중요 선택이 많을수록 값이 커집니다.)`);
        lines.push(`- **안정성 등급**: ${s.complexity.stabilityGrade} (전체 흐름의 안정/불안정 요약)`);
        lines.push(`- **변화의 불안정성**: ${Number.isFinite(s.complexity.entropy) ? s.complexity.entropy.toFixed(2) : "—"} (높을수록 변동·혼선 가능성)`);
        if (Array.isArray(s.complexity.decisionNodes) && s.complexity.decisionNodes.length > 0) {
            lines.push(`- **주요 분기점(나이)**: ${s.complexity.decisionNodes.slice(0, 40).join("세, ")}${s.complexity.decisionNodes.length > 40 ? `세 ... 외 ${s.complexity.decisionNodes.length - 40}개` : "세"}`);
        }
        lines.push("");
    }

    // 주의가 필요한 시기 (대운·세운 전수 조사)
    if (s.vulnerabilityReport?.criticalVectors?.length) {
        lines.push("## 주의가 필요한 시기 (대운·세운 전수 조사)\n");
        lines.push(`- **총 탐지 건수**: ${s.vulnerabilityReport.totalCrashes}개 (위험도가 높은 순으로 상위 항목만 정리)`);
        lines.push("");
        lines.push("| 위험도 점수 | 대운 | 세운 | 유형 | 주요 태그 |");
        lines.push("|---:|---|---|---|---|");
        const top = [...s.vulnerabilityReport.criticalVectors]
            .sort((a, b) => (a.crashScore ?? 9999) - (b.crashScore ?? 9999))
            .slice(0, 12);
        for (const v of top) {
            const major = v.vector?.major ? `${ganziDisplay(v.vector.major)} (${ganziHangul(v.vector.major)})` : "—";
            const yearly = v.vector?.yearly ? `${ganziDisplay(v.vector.yearly)} (${ganziHangul(v.vector.yearly)})` : "—";
            const tags = (v.tags ?? []).slice(0, 12).map(tagToStr).join(", ");
            lines.push(`| ${Number.isFinite(v.crashScore) ? v.crashScore.toFixed(1) : "—"} | ${major} | ${yearly} | ${v.vulnerabilityType ?? "—"} | ${tags || "—"} |`);
        }
        lines.push("");
    }

    // 정밀 분석(power)
    if (r.power) {
        lines.push("## 정밀 분석 요약 (오행·십신 분포)\n");
        lines.push(`- **우세 오행**: ${elKr(r.power.dominantElement)} (가장 강하게 나타나는 기운)`);
        lines.push(`- **우세 십신**: ${tgKr(r.power.dominantTenGod)} (행동/관계 패턴의 중심 역할)`);
        if (Array.isArray(r.power.elementScores) && r.power.elementScores.length) {
            lines.push("\n### 오행 점수\n");
            lines.push("| 오행 | 비중(%) | 점수 |");
            lines.push("|---|---:|---:|");
            for (const [el, pct, score] of r.power.elementScores) {
                lines.push(`| ${elKr(el as string)} | ${(pct ?? 0).toFixed(1)} | ${(score ?? 0).toFixed(1)} |`);
            }
            lines.push("");
        }
        if (Array.isArray(r.power.tenGodScores) && r.power.tenGodScores.length) {
            lines.push("### 십신 점수\n");
            lines.push("| 십신 | 비중(%) | 점수 |");
            lines.push("|---|---:|---:|");
            for (const [tg, pct, score] of r.power.tenGodScores) {
                lines.push(`| ${tgKr(tg as string)} | ${(pct ?? 0).toFixed(1)} | ${(score ?? 0).toFixed(1)} |`);
            }
            lines.push("");
        }
    }

    // 십신(기둥별)
    if (r.tenGods) {
        lines.push("## 십신 배치 (기둥별 역할)\n");
        lines.push("| 위치 | 천간 | 지지 |");
        lines.push("|---|---|---|");
        lines.push(`| 년주 | ${tgKr(r.tenGods.yearStem)} | ${tgKr(r.tenGods.yearBranch)} |`);
        lines.push(`| 월주 | ${tgKr(r.tenGods.monthStem)} | ${tgKr(r.tenGods.monthBranch)} |`);
        lines.push(`| 일주 | ${tgKr(r.tenGods.dayStem)} | ${tgKr(r.tenGods.dayBranch)} |`);
        lines.push(`| 시주 | ${tgKr(r.tenGods.hourStem)} | ${tgKr(r.tenGods.hourBranch)} |`);
        lines.push("");
    }

    // 위기 요약
    lines.push("## 시뮬레이션 위기 요약 (급격한 변동/충돌 신호)\n");
    lines.push(`- **위기 발생 횟수**: ${s.crashCount}번`);
    lines.push("");

    // 부하 진단
    if (s.loadDiagnostics?.length) {
        lines.push("## 인생 부하 진단 (과부하/다운 구간과 대응 전략)\n");
        lines.push("| 나이 | 상태 | 원인 | 대응 전략 |");
        lines.push("|---:|---|---|---|");
        for (const d of s.loadDiagnostics) {
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
    if (r.vmSummary) {
        lines.push("## VM 분석 요약\n");
        lines.push(r.vmSummary);
        lines.push("");
    }

    // 연도별 점수 타임라인
    if (r.timeline?.length) {
        lines.push("## 연도별 흐름 점수 타임라인 (100년 시뮬레이션)\n");
        lines.push("| 나이 | 연도 | 총점 | 재물 | 커리어 | 학업 | 건강 | 변동성 | 전환기 |");
        lines.push("|---:|---:|---:|---:|---:|---:|---:|---:|---|");
        for (const t of r.timeline) {
            lines.push(`| ${t.age} | ${t.year} | ${t.totalScore.toFixed(1)} | ${t.wealthScore.toFixed(1)} | ${t.careerScore.toFixed(1)} | ${t.academicScore.toFixed(1)} | ${t.healthScore.toFixed(1)} | ${t.volatilityIndex.toFixed(1)} | ${t.isTransitionPeriod ? "✅" : "—"} |`);
        }
        lines.push("");
    }

    // 시뮬레이션 프레임 요약
    if (r.simulationFrames?.length) {
        lines.push("## 시뮬레이션 프레임 요약 (대운·세운 기반 스냅샷)\n");
        lines.push("| 나이 | 세운 | 대운 | 점수 | 태그 |");
        lines.push("|---:|---|---|---:|---|");
        for (const f of r.simulationFrames) {
            const yearly = f.ganzi ? `${ganziDisplay(f.ganzi)} (${ganziHangul(f.ganzi)})` : "—";
            const major = f.majorGanzi ? `${ganziDisplay(f.majorGanzi)} (${ganziHangul(f.majorGanzi)})` : "—";
            const tags = (f.tags ?? []).map(tagToStr).filter(Boolean).join(", ");
            lines.push(`| ${f.age} | ${yearly} | ${major} | ${(f.score ?? 0).toFixed(1)} | ${tags || "—"} |`);
        }
        lines.push("");

        const topFrames = [...r.simulationFrames].sort((a, b) => (a.score ?? 9999) - (b.score ?? 9999)).slice(0, 10);
        lines.push("### ESIL 트레이스 (상위 위험 프레임 일부)\n");
        for (const f of topFrames) {
            lines.push(`- **${f.age}세** (${ganziDisplay(f.ganzi)} / 대운 ${ganziDisplay(f.majorGanzi)}) 점수 ${(f.score ?? 0).toFixed(1)}`);
            const esilRaw = (f.esilTrace ?? "").trim();
            if (esilRaw) {
                lines.push("");
                for (const tl of esilRaw.split("\n").filter(Boolean)) {
                    const parts = tl.includes(";") ? tl.split(";").map(s => s.trim()).filter(Boolean) : [tl.trim()];
                    for (const part of parts) {
                        lines.push(`  - ${formatEsilLine(part)}`);
                    }
                }
            }
        }
        lines.push("");
    }

    // 합충형해 (Relationships)
    if (r.relationships?.mappedRelationships?.length) {
        lines.push("## 합충형해 분석 (기둥 간의 상호작용)\n");
        for (const rel of r.relationships.mappedRelationships) {
            const emoji = rel.level === "Auspicious" ? "🤝" : rel.level === "Caution" ? "⚡" : "•";
            lines.push(`### ${emoji} ${rel.name} (${rel.positions.join(", ")})`);
            lines.push(`- **요약**: ${rel.summary}`);
            lines.push(`- **설명**: ${rel.description}`);
            if (rel.reasons?.length) {
                lines.push(`- **근거**: ${rel.reasons.join(", ")}`);
            }
            if (rel.transformedElement) {
                const el = ELEMENT_INFO[rel.transformedElement];
                lines.push(`- **변화**: ${el ? `${el.hangul}(${el.hanja})` : rel.transformedElement} 기운으로 변화`);
            }
            lines.push("");
        }
    }

    // 공망 (Void)
    if (s.voidAnalysis?.mappedVoids?.length) {
        lines.push("## 공망 분석 (Energy Void Analysis)\n");
        for (const v of s.voidAnalysis.mappedVoids) {
            lines.push(`### 🕳️ ${v.branch} (${v.position})`);
            lines.push(`- **요약**: ${v.summary}`);
            lines.push(`- **설명**: ${v.description}`);
            if (v.reasons?.length) {
                lines.push(`- **근거**: ${v.reasons.join(", ")}`);
            }
            lines.push("");
        }
    }

    return lines.join("\n");
}

// ── 베딕 섹션 ────────────────────────────────────────

export function buildVedicMarkdown(v: VedicAnalysisResult): string {
    const rr = v.report;
    const c = v.chart;
    const lines: string[] = [];

    lines.push("# 베딕 점성학 분석 리포트\n");

    // 카라카
    lines.push("## 주요 카라카 (인생의 핵심 역할을 나타내는 지표)\n");
    lines.push(`- **영혼 지표 (Atmakaraka)**: ${rr.primaryKarakas?.atmakaraka ?? "N/A"}`);
    lines.push(`- **직업 지표 (Amatyakaraka)**: ${rr.primaryKarakas?.amatyakaraka ?? "N/A"}`);
    lines.push(`- **파트너 지표 (Darakaraka)**: ${rr.primaryKarakas?.darakaraka ?? "N/A"}`);
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
            lines.push(`| ${roleKr[k.role] ?? k.role} | ${k.planet} | ${(k.degreeInRasi ?? 0).toFixed(2)}° |`);
        }
        lines.push("");
    }

    // 현재 대운 & 전체 강도
    lines.push("## 분석 요약: 현재 대운 및 차트 강도\n");
    lines.push(`- **현재 대운 (Dasha)**: ${rr.dashaFocus} (인생의 현재 단계에서 가장 강력한 영향을 미치는 기운)`);
    lines.push(`- **사데사티 (Sade Sati)**: ${rr.sadeSati} (토성의 월지 트랜짓 영향권 여부)`);
    lines.push(`- **전체 차트 강도**: ${Math.round(rr.overallStrengthScore)}/600 (중요 행성 및 하우스 강점 총합)`);
    lines.push("");

    // 판창가 (Panchanga)
    if (c?.panchanga) {
        const p = c.panchanga;
        lines.push("## 판창가 (Panchanga — 출생 시각의 천문 기상도)\n");
        lines.push(`- **요일 (Vara)**: ${p.vara} (Day Lord: ${p.dayLord})`);
        lines.push(`- **티티 (음력일)**: ${p.tithi} — ${p.tithiName}`);
        lines.push(`- **낙샤트라 번호**: No. ${p.nakshatra}`);
        lines.push(`- **판창가 요가 (Nitya Yoga)**: No. ${p.yoga}`);
        lines.push(`- **카라나 (Karana)**: ${p.karanaName}`);
        lines.push(`- **일주/시주**: ${p.dayLord} / ${p.hourLord}`);
        lines.push(`- **출생 시간대**: ${p.isDayBirth ? "주간(Day) 출생" : "야간(Night) 출생"}`);
        lines.push("");
    }

    // 다샤 타임라인
    if (rr.dashaTimeline?.length) {
        lines.push("## 다샤 타임라인 (Vimshottari Dasha — 마하다샤 & 안타르다샤)\n");
        lines.push("| 구분 | 행성 (Planet) | 시작 | 종료 |");
        lines.push("|---|---|---|---|");
        for (const maha of rr.dashaTimeline) {
            lines.push(`| **Mahadasha** | **${maha.lord}** | ${fmtYearMonth(maha.startTime)} | ${fmtYearMonth(maha.endTime)} |`);
            for (const antar of maha.subDashas ?? []) {
                lines.push(`| └ Antardasha | ${antar.lord} | ${fmtYearMonth(antar.startTime)} | ${fmtYearMonth(antar.endTime)} |`);
            }
        }
        lines.push("");
    }

    // 하우스 요약
    if (rr.houseSummary?.length) {
        lines.push("## 하우스별 에너지 (삶의 영역별 지원/강도)\n");
        lines.push("| 하우스 | 점수 | 등급 |");
        lines.push("|---|---|---|");
        for (const h of rr.houseSummary) {
            lines.push(`| ${h.house} | ${Math.round(h.totalScore)} | ${h.rating} |`);
        }
        lines.push("");
    }

    // 하우스 강도 (Bhava Strength)
    if (c?.bhavaStrengths?.length) {
        lines.push("## 하우스 강도 (Bhava Strength)\n");
        lines.push("| 하우스 | 강도 (Shadbala) |");
        lines.push("|---|---|");
        for (const s of c.bhavaStrengths) {
            lines.push(`| H${s.house} | ${s.totalScore.toFixed(2)} |`);
        }
        lines.push("");
    }

    // SAV
    if (c?.sav?.points) {
        const pts = c.sav.points;
        if (Array.isArray(pts) && pts.length === 12) {
            lines.push("## SAV (사르바아슈타카바르가) — 하우스 종합 점수\n");
            lines.push("| " + Array.from({ length: 12 }, (_, i) => `H${i + 1}`).join(" | ") + " |");
            lines.push("| " + Array.from({ length: 12 }, () => "---").join(" | ") + " |");
            lines.push("| " + pts.join(" | ") + " |");
            lines.push("");
        }
    }

    // BAV
    if (c?.bav?.length) {
        lines.push("## BAV (빈나슈타카바르가) — 행성별 하우스 기여 점수\n");
        lines.push("| 행성 | H1 | H2 | H3 | H4 | H5 | H6 | H7 | H8 | H9 | H10 | H11 | H12 |");
        lines.push("|---|---|---|---|---|---|---|---|---|---|---|---|---|");
        for (const b of c.bav) {
            lines.push(`| ${b.planet} | ${b.points.join(" | ")} |`);
        }
        lines.push("");
    }

    // 빔쇼파카 (Vimshopaka)
    if (c?.vimshopakaScores?.length) {
        lines.push("## 빔쇼파카 점수 (Vimshopaka)\n");
        lines.push("| 행성 | 점수 |");
        lines.push("|---|---|");
        for (const [pName, score] of c.vimshopakaScores) {
            const total = (score.shadvargaScore ?? 0) + (score.shodashavargaScore ?? 0);
            lines.push(`| ${pName} | ${total.toFixed(2)} |`);
        }
        lines.push("");
    }

    // 행성 상태 (Avasthas)
    if (c?.avasthas?.length) {
        lines.push("## 행성 상태 (아바스타)\n");
        lines.push("| 행성 | Baladi | Jagradi | 상태 |");
        lines.push("|---|---|---|---|");
        for (const a of c.avasthas) {
            const pData = (c.planets as any[]).find(p => p.planet === a.planet);
            const status = [pData?.isRetrograde ? "℞(역행)" : "", pData?.isCombust ? "☀(연소)" : ""].filter(Boolean).join(", ") || "정상";
            lines.push(`| ${a.planet} | ${a.baladi} | ${a.jagradadi} | ${status} |`);
        }
        lines.push("");
    }

    // 하우스 경계 (Cusps)
    if (c?.houseCusps?.length) {
        lines.push("## 하우스 경계 (Cusps)\n");
        lines.push("| 하우스 | 경계 (Cusp) |");
        lines.push("|---|---|");
        for (let i = 0; i < 12; i++) {
            lines.push(`| H${i + 1} | ${formatSiderealPosition(c.houseCusps[i])} |`);
        }
        lines.push("");
    }

    // 행성 좌상 (Aspects)
    if (c?.aspects?.length) {
        lines.push("## 행성 좌상 (Aspects)\n");
        for (const a of c.aspects) {
            lines.push(`- **${a.aspectingPlanet}** → ${(a.aspectedHouses ?? []).map((h: number) => `H${h}`).join(", ")}`);
        }
        lines.push("");
    }

    // 요가 (Yogas)
    if (rr.yogas?.length) {
        lines.push("## 베딕 요가 (주요 행성 조합)\n");
        lines.push("| 요가명 | 품질 | 관련 행성 | 설명 |");
        lines.push("|---|---|---|---|");
        for (const yoga of rr.yogas) {
            let q = typeof yoga.quality === "string" ? yoga.quality : "Medium";
            const qMap: Record<string, string> = { 
                VeryHigh: "매우 강함", veryHigh: "매우 강함",
                High: "강함", high: "강함",
                Medium: "보통", medium: "보통",
                Low: "약함", low: "약함" 
            };
            lines.push(`| ${yoga.name} | ${qMap[q] ?? q} | ${(yoga.planetsInvolved ?? []).join(", ")} | ${yoga.description} |`);
        }
        lines.push("");
    }

    // 낙샤트라 리포트 (통합)
    lines.push("## 낙샤트라 리포트 (Varga Charts)\n");
    const vMap = v.vargaNakshatraReports?.reports;
    if (vMap) {
        const keys = Object.keys(vMap);
        for (const vKey of keys) {
            const report = (vMap as any)[vKey];
            if (!report) continue;
            lines.push(`### ${report.vargaLabel} (${vKey})`);
            const lagna = report.lagnaRasi != null ? ` (라그나: ${SIGN_NAMES[report.lagnaRasi]})` : "";
            if (lagna) lines.push(lagna);
            const mdRows = buildNakshatraMarkdownRows(report.rows, vKey !== "rasi");
            for (const row of mdRows) lines.push(row);
            lines.push("");
        }
    }

    return lines.join("\n");
}

// ── ESIL 트레이스 포맷 변환 ────────────────────────────

const ESIL_PANIC_LABELS: Record<string, string> = {
    earth_overflow: "🌍 토 과부하",
    water_overflow: "🌊 수 과부하",
    fire_overflow: "🔥 화 과부하",
    wood_overflow: "🌳 목 과부하",
    metal_overflow: "⚙️ 금 과부하",
};

const formatEsilLine = (line: string): string => {
    let formatted = line;
    if (line.includes("infl:")) {
        formatted = line.replace(/([가-힣]+)_infl:([가-힣]),weight:(-?[0-9.]+),score\+=([-0-9.]+)/, (_, src, el, w, sc) => {
            const wVal = parseFloat(w);
            const scVal = parseFloat(sc);
            return `${src}의 ${el} 기운 유입 (영향 x${Math.abs(wVal)}) ➔ ${scVal >= 0 ? "+" : ""}${scVal}점`;
        });
    } else if (line.includes("shinsal:")) {
        formatted = line.replace(/shinsal:([가-힣]+),score:([0-9.-]+)/, "신살 [$1] ➔ $2점");
    } else if (line.includes("panic")) {
        const key = Object.keys(ESIL_PANIC_LABELS).find(k => line.includes(k));
        formatted = `⚠️ 패닉: ${key ? ESIL_PANIC_LABELS[key] : "치명적 타격"}`;
    }
    return formatted;
};

// ── 운세 섹션 ────────────────────────────────────────

export function buildTransitMarkdown(t: TransitResult): string {
    const lines: string[] = [];
    lines.push("# 현재 운세 리포트 (세운·월운)\n");
    lines.push(`- **현재 나이**: ${t.currentAge}세`);
    lines.push("");

    const y = t.yearlyLuck;
    lines.push("## 세운 (올해)\n");
    lines.push(`- **연도**: ${y.year}`);
    lines.push(`- **간지**: ${ganziDisplay(y.ganzi)} (${ganziHangul(y.ganzi)})`);
    lines.push(`- **십신**: ${tgKr(y.stemGod)} / ${tgKr(y.branchGod)}`);
    if (y.twelveStage) lines.push(`- **12운성**: ${y.twelveStage}`);
    lines.push("");

    const m = t.monthlyLuck;
    lines.push("## 월운 (이번 달)\n");
    lines.push(`- **연-월**: ${m.year}-${String(m.month).padStart(2, "0")}`);
    lines.push(`- **간지**: ${ganziDisplay(m.ganzi)} (${ganziHangul(m.ganzi)})`);
    lines.push(`- **십신**: ${tgKr(m.stemGod)} / ${tgKr(m.branchGod)}`);
    lines.push("");

    if (t.currentFrame) {
        const f = t.currentFrame;
        lines.push("## 현재 시뮬레이션 프레임\n");
        lines.push(`- **나이**: ${f.age}세`);
        lines.push(`- **점수**: ${(f.score ?? 0).toFixed(1)} / 100`);
        if (f.tags?.length) lines.push(`- **태그**: ${f.tags.map(tagToStr).join(", ")}`);
        lines.push("");
    }

    return lines.join("\n");
}

// ── 운명 티어 섹션 ──────────────────────────────────────

export function buildDestinyTierMarkdown(
    _saju: any, _vedic: any, _transit: any,
    tierResult?: TierResult | null,
): string {
    if (!tierResult) return "";
    const r = tierResult;
    const lines: string[] = [];

    lines.push("# 운명 티어 분석\n");
    lines.push(`- **운명 티어**: ${r.destinyTier.grade} (${r.destinyTier.label})`);
    lines.push(`- **잠재력 티어**: ${r.potentialTier.grade} (${r.potentialTier.label})`);
    lines.push(`- **종합 점수**: ${Math.round(r.destinyScore)}점`);
    lines.push("");

    if (r.strengths?.length) {
        lines.push("## 주요 강점\n");
        for (const s of r.strengths) lines.push(`- ${s}`);
        lines.push("");
    }

    if (r.weaknesses?.length) {
        lines.push("## 주의 리스크\n");
        for (const w of r.weaknesses) lines.push(`- ${w}`);
        lines.push("");
    }

    return lines.join("\n");
}

// ── 통합 리포트 ────────────────────────────────────────

export function buildFullAnalysisMarkdown(
    saju: SajuAnalysisResult | null,
    vedic: VedicAnalysisResult | null,
    transit: TransitResult | null,
    tier: TierResult | null,
): string {
    const parts: string[] = [];
    parts.push("# Eon 통합 리포트\n");
    parts.push(`- **생성 시각**: ${new Date().toLocaleString()}\n`);

    if (tier) {
        parts.push(buildDestinyTierMarkdown(null, null, null, tier));
        parts.push(SEP);
    }

    if (saju) {
        parts.push(buildSajuMarkdown(saju));
        parts.push(SEP);
    }

    if (transit) {
        parts.push(buildTransitMarkdown(transit));
        parts.push(SEP);
    }

    if (vedic) {
        parts.push(buildVedicMarkdown(vedic));
    }

    return parts.join("\n");
}
