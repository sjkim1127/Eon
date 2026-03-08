import { TierResult } from "../../utils/tierScore";

export const TIER_GRADES_UI = [
  { grade: "S+", label: "천기",    color: "from-orange-300 to-amber-500",    desc: "극희귀 최상의 조합" },
  { grade: "S",  label: "천운",    color: "from-amber-400 to-yellow-600",    desc: "사주와 별운이 모두 유리한 극상의 조합" },
  { grade: "A+", label: "대길상",  color: "from-lime-400 to-emerald-500",    desc: "용신·대운·요가가 거의 완벽하게 지원하는 강운" },
  { grade: "A",  label: "대길",    color: "from-emerald-400 to-green-600",   desc: "전반적으로 아주 강한 기운의 조합" },
  { grade: "B+", label: "길상",    color: "from-sky-400 to-celestial-cyan",  desc: "균형이 잡히고 강점이 뚜렷하게 빛나는 운세" },
  { grade: "B",  label: "길",      color: "from-celestial-cyan to-indigo-500", desc: "전반적으로 안정적이고 활용 가능한 운세" },
  { grade: "C+", label: "중상",    color: "from-violet-400 to-purple-500",   desc: "보통 이상의 기운, 노력으로 충분히 도약 가능" },
  { grade: "C",  label: "중평",    color: "from-slate-400 to-slate-600",     desc: "일부 어려움이 있으나 극복 가능한 조합" },
  { grade: "D+", label: "성장예비", color: "from-orange-400 to-rose-500",    desc: "성장 여지가 많으며 빠른 상향 가능" },
  { grade: "D",  label: "다다익선", color: "from-rose-400 to-pink-600",      desc: "성장 여지가 많은 시기, 주의 시점 활용 권장" },
] as const;

export const PROFILE_META: Record<string, { icon: string; label: string; color: string }> = {
  growth:   { icon: "🌱", label: "성장형",  color: "bg-emerald-500/20 text-emerald-300 border-emerald-500/40" },
  balanced: { icon: "⚖️", label: "균형형",  color: "bg-celestial-cyan/20 text-celestial-cyan border-celestial-cyan/40" },
  stable:   { icon: "🏛️", label: "안정형",  color: "bg-slate-500/20 text-slate-300 border-slate-500/40" },
};

export const RISK_META: Record<string, { icon: string; label: string; color: string; bg: string }> = {
  low:    { icon: "🟢", label: "리스크 낮음", color: "text-emerald-300", bg: "bg-emerald-500/15 border-emerald-500/30" },
  medium: { icon: "🟡", label: "리스크 보통", color: "text-amber-300",   bg: "bg-amber-500/15 border-amber-500/30"   },
  high:   { icon: "🔴", label: "리스크 높음", color: "text-rose-300",    bg: "bg-rose-500/15 border-rose-500/30"     },
};

export const TIER_SCORE_MAP: Record<string, number> = {
  "S+": 10, S: 9, "A+": 8, A: 7, "B+": 6, B: 5, "C+": 4, C: 3, "D+": 2, D: 1,
};

export function buildInsightBlocks(result: TierResult): { title: string; icon: string; text: string; color: string }[] {
  const { destinyTier, potentialTier, profile, riskLevel, growthGap,
    strengths, weaknesses, domainTiers, sajuResult, vedicResult, transitResult,
    natalScore, currentScore, destinyScore } = result;
  const grade = destinyTier.grade;
  const blocks: { title: string; icon: string; text: string; color: string }[] = [];

  // 1. 종합 판정
  const base = grade.replace("+", "") as string;
  const baseMap: Record<string, string> = {
    "S+": `사주와 별운이 완전히 일치하는 극희귀 최상의 조합입니다(${Math.round(destinyScore)}점). 모든 조건이 이상적으로 결합된 천기(天機) 수준의 운세입니다.`,
    S:  `사주와 별운이 서로 보완하며 극상의 기운을 이룹니다(${Math.round(destinyScore)}점). 대부분의 조건이 이상적으로 결합된 희귀한 조합입니다.`,
    "A+": `용신·대운·요가가 거의 완벽하게 지원하는 강한 차트입니다(${Math.round(destinyScore)}점). 적극적인 도전과 확장이 결실을 맺기 매우 좋은 환경입니다.`,
    A:  `전반적으로 매우 강한 차트입니다(${Math.round(destinyScore)}점). 용신·대운·요가가 유리하게 맞물리는 시기에 적극적인 도전이 빛납니다.`,
    "B+": `균형이 잡혀 있고 강점이 뚜렷하게 빛나는 운세입니다(${Math.round(destinyScore)}점). 강점 분야를 주력으로 삼으면 기대 이상의 결과를 낼 수 있습니다.`,
    B:  `전반적으로 안정적이고 활용 가능한 운세입니다(${Math.round(destinyScore)}점). 강점을 살리고 주의 시점을 사전에 파악해 보완하면 좋은 결과를 기대할 수 있습니다.`,
    "C+": `보통 이상의 기운으로 노력에 따라 충분히 도약 가능합니다(${Math.round(destinyScore)}점). 골든타임·용신 방향을 정확하게 파악하고 실행하는 것이 키포인트입니다.`,
    C:  `일부 어려운 구간이 있으나 충분히 극복 가능합니다(${Math.round(destinyScore)}점). 주의 시점과 골든타임·대운 흐름을 함께 참고하세요.`,
    "D+": `성장 여지가 많으며 조건이 갖춰지면 빠른 상향이 가능합니다(${Math.round(destinyScore)}점). 지금은 기반을 다지고 골든타임을 기다리는 준비 단계입니다.`,
    D:  `성장 여지가 많은 시기입니다(${Math.round(destinyScore)}점). 주의 구간을 피하고 용신·요가가 도와주는 구간을 집중 활용하면 큰 변화를 만들 수 있습니다.`,
  };
  blocks.push({ title: "종합 판정", icon: "🏆", text: baseMap[grade] ?? baseMap[base] ?? "", color: "text-celestial-gold" });

  // 2. 원국 vs 현재 운세 비교
  const diffText = (() => {
    const diff = currentScore - natalScore;
    if (Math.abs(diff) < 5) return `원국(${Math.round(natalScore)}점)과 현재 운세(${Math.round(currentScore)}점)가 거의 일치합니다. 타고난 흐름 그대로 안정적으로 진행 중입니다.`;
    if (diff > 15) return `현재 운세(${Math.round(currentScore)}점)가 원국(${Math.round(natalScore)}점)보다 현저히 높습니다. 지금이 바로 행동해야 할 최적의 타이밍입니다.`;
    if (diff > 0) return `현재 운세(${Math.round(currentScore)}점)가 원국(${Math.round(natalScore)}점)보다 소폭 우세합니다. 적극적인 실행이 효과적인 시기입니다.`;
    if (diff < -15) return `현재 운세(${Math.round(currentScore)}점)가 원국(${Math.round(natalScore)}점)보다 눈에 띄게 낮습니다. 리스크를 줄이고 내실을 다지는 수성(守城) 전략을 권합니다.`;
    return `현재 운세(${Math.round(currentScore)}점)가 원국(${Math.round(natalScore)}점)보다 소폭 낮습니다. 과도한 확장보다 준비와 기반 강화에 집중하세요.`;
  })();
  blocks.push({ title: "원국 vs 현재 운세", icon: "⚖️", text: diffText, color: "text-celestial-cyan" });

  // 3. 프로필 기반 조언
  const profileMap: Record<string, string> = {
    growth: "현재 성장 가속 구간(35세 미만)에 있습니다. 용신 오행과 연계된 색상·방위·직업군을 환경에 적극 반영하고, 골든타임 내 학습·도전·투자를 집중하세요.",
    stable: "안정형 원국(56세 이상)으로 타고난 흐름이 흔들리지 않습니다. 새로운 모험보다 리스크 관리와 꾸준한 실천, 후진 양성에 집중하면 더욱 탄탄한 기반을 만들 수 있습니다.",
    balanced: "원국과 현재 운세가 균형을 이루고 있습니다. 강점 분야를 적극 공략하면서도 약점 구간의 방어를 소홀히 하지 마세요.",
  };
  blocks.push({ title: "운세 프로필", icon: profile === "growth" ? "🌱" : profile === "stable" ? "🏛️" : "⚖️", text: profileMap[profile], color: "text-emerald-300" });

  // 4. 강점 요약
  if (strengths.length > 0) {
    const strengthText = strengths.map(s => `· ${s}`).join("  ") + "  이 요소들이 운명 티어를 지탱하는 핵심 기둥입니다.";
    blocks.push({ title: "핵심 강점", icon: "✨", text: strengthText, color: "text-amber-300" });
  }

  // 5. 약점 & 리스크
  const riskMap: Record<string, string> = {
    high: "⚠️ 현재 리스크 요소가 복합적으로 집중된 구간입니다. 주의 시점 탭의 대운·세운 충돌 지점을 반드시 사전에 파악하고, 중요한 계약·투자·이직 결정을 신중히 내리세요.",
    medium: "일부 주의가 필요구간이 있습니다. 역량 탭의 부하 시점을 확인하고, 체력 관리와 재정 안전망을 점검하세요.",
    low: "리스크 요인이 적어 안정적인 환경입니다. 이 여유를 기반 강화와 장기 포트폴리오 구축에 활용하세요.",
  };
  const weaknessPart = weaknesses.length > 0 ? `  주요 약점: ${weaknesses.join(", ")}.` : "";
  blocks.push({ title: "리스크 & 약점", icon: riskLevel === "high" ? "🔴" : riskLevel === "medium" ? "🟡" : "🟢", text: riskMap[riskLevel] + weaknessPart, color: riskLevel === "high" ? "text-rose-300" : riskLevel === "medium" ? "text-amber-300" : "text-emerald-300" });

  // 6. 잠재력 격차 조언
  const potGrade = potentialTier.grade;
  const potText = (() => {
    if (growthGap > 20) return `잠재력 티어(${potGrade})가 운명 티어(${grade})보다 +${Math.round(growthGap)}pt 크게 앞섭니다. 용신 오행 강화, 우수 요가 활성화, 골든타임 집중 활용으로 단기간에 큰 상향이 가능합니다.`;
    if (growthGap > 10) return `잠재력(${potGrade})이 운명 티어(${grade})보다 +${Math.round(growthGap)}pt 앞서 있습니다. 일관된 노력과 주의 시점 회피로 꾸준히 격차를 좁혀가세요.`;
    if (growthGap > 3) return `잠재력(+${Math.round(growthGap)}pt 여유)을 점진적으로 발현 중입니다. 현재 방향을 유지하면 자연스럽게 상향됩니다.`;
    if (growthGap < -5) return `운명 티어(${grade})가 잠재력(${potGrade})보다 앞서 있습니다. 현재의 흐름이 매우 효율적임을 의미합니다.`;
    return "잠재력과 운명 티어가 거의 일치합니다. 현재의 흐름을 잘 유지하고 있습니다.";
  })();
  blocks.push({ title: "잠재력 격차 분석", icon: "🚀", text: potText, color: "text-purple-300" });

  // 7. 분야별 집중 공략
  const topDomains = [...domainTiers].filter(d => d.tier === "S" || d.tier === "A").slice(0, 3);
  const weakDomains = [...domainTiers].filter(d => d.tier === "D" || d.tier === "C").slice(0, 3);
  if (topDomains.length > 0 || weakDomains.length > 0) {
    const domainParts: string[] = [];
    if (topDomains.length > 0) domainParts.push(`강점 분야: ${topDomains.map(d => `${d.domain}(${d.tier})`).join(", ")} — 이 영역을 주력 무대로 삼으세요.`);
    if (weakDomains.length > 0) domainParts.push(`보완 분야: ${weakDomains.map(d => `${d.domain}(${d.tier})`).join(", ")} — 과도한 집중보다 방어적 관리를 권합니다.`);
    blocks.push({ title: "분야별 집중 전략", icon: "🎯", text: domainParts.join("  "), color: "text-sky-300" });
  }

  // 8. 사주 vs 베딕 점수 균형
  const sajuS = sajuResult.score;
  const vedicS = vedicResult.score;
  const transitS = transitResult.score;
  const balanceParts: string[] = [];
  if (Math.abs(sajuS - vedicS) > 20) {
    if (sajuS > vedicS) balanceParts.push(`사주 원국(${sajuS}점)이 베딕 차트(${vedicS}점)보다 강합니다. 동양 명리 기반의 판단이 더 정확할 수 있습니다.`);
    else balanceParts.push(`베딕 차트(${vedicS}점)가 사주 원국(${sajuS}점)보다 강합니다. 베딕 요가와 다샤 흐름을 우선 참고하세요.`);
  } else {
    balanceParts.push(`사주(${sajuS}점)·베딕(${vedicS}점) 두 체계가 균형 있게 같은 방향을 가리킵니다. 신뢰도가 높은 분석 결과입니다.`);
  }
  if (transitS >= 70) balanceParts.push(`현재 운세 점수(${transitS}점)가 높아 지금이 행동하기 좋은 시기입니다.`);
  else if (transitS < 40) balanceParts.push(`현재 운세 점수(${transitS}점)가 낮습니다. 중요 결정은 운세 점수가 회복된 후 미루는 것을 권합니다.`);
  blocks.push({ title: "분석 체계 균형", icon: "🔬", text: balanceParts.join("  "), color: "text-indigo-300" });

  return blocks;
}
