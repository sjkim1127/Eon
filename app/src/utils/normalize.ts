import type { VedicAnalysisResult, VedicAnalysisReport, GocharaSummary } from "../types/vedic";
import type { SajuAnalysisResult, TransitResult } from "../types/saju";

function toCamelCase(str: string): string {
  return str.replace(/_([a-z0-9])/g, (g) => g[1].toUpperCase());
}

export function deepToCamelCase(obj: any): any {
  if (obj === null || typeof obj !== "object") return obj;
  if (Array.isArray(obj)) return obj.map(deepToCamelCase);

  const result: any = {};
  for (const key of Object.keys(obj)) {
    const camelKey = toCamelCase(key);
    result[camelKey] = deepToCamelCase(obj[key]);
  }
  return result;
}

/**
 * Normalizes a Vedic analysis report by ensuring all fields are present
 * and handling both snake_case and camelCase fallbacks for robustness.
 */
export function normalizeVedicReport(raw: any): VedicAnalysisReport {
  if (!raw) return {} as VedicAnalysisReport;

  const pk = raw.primaryKarakas ?? raw.primary_karakas ?? {};
  
  return {
    primaryKarakas: {
      atmakaraka: pk.atmakaraka ?? "Sun",
      amatyakaraka: pk.amatyakaraka ?? "Sun",
      bhratrukaraka: pk.bhratrukaraka ?? null,
      matrukaraka: pk.matrukaraka ?? null,
      pitrikaraka: pk.pitrikaraka ?? null,
      putrakaraka: pk.putrakaraka ?? null,
      gnatikaraka: pk.gnatikaraka ?? null,
      darakaraka: pk.darakaraka ?? "Sun",
    },
    houseSummary: (raw.houseSummary ?? raw.house_summary ?? []).map((h: any) => ({
      house: h.house ?? 0,
      rating: h.rating ?? "Average",
      totalScore: h.totalScore ?? h.total_score ?? 0,
      summary: h.summary ?? "",
      description: h.description ?? "",
      reasons: h.reasons ?? [],
    })),
    dashaFocus: raw.dashaFocus ?? raw.dasha_focus ?? "",
    dashaTimeline: normalizeDashaTimeline(raw.dashaTimeline ?? raw.dasha_timeline ?? []),
    yoginiTimeline: normalizeDashaTimeline(raw.yoginiTimeline ?? raw.yogini_timeline ?? []),
    charaDashaTimeline: (raw.charaDashaTimeline ?? raw.chara_dasha_timeline ?? []).map((p: any) => ({
      type: "sign",
      rasi: p.rasi ?? 1,
      startTime: p.startTime ?? p.start_time ?? "",
      endTime: p.endTime ?? p.end_time ?? "",
    })),
    allKarakas: (raw.allKarakas ?? raw.all_karakas ?? []).map((k: any) => ({
      planet: k.planet ?? "",
      role: k.role ?? "",
      degreeInRasi: k.degreeInRasi ?? k.degree_in_rasi ?? 0,
    })),
    nakshatraInfo: raw.nakshatraInfo ?? raw.nakshatra_info ?? "",
    overallStrengthScore: raw.overallStrengthScore ?? raw.overall_strength_score ?? 0,
    sadeSati: raw.sadeSati ?? raw.sade_sati ?? "None",
    yogas: (raw.yogas ?? raw.yogas ?? []).map((y: any) => ({
      name: y.name ?? "",
      yogaType: y.yogaType ?? y.yoga_type ?? "",
      description: y.description ?? "",
      planetsInvolved: y.planetsInvolved ?? y.planets_involved ?? [],
      quality: y.quality ?? "Medium",
    })),
    arudhaLagna: raw.arudhaLagna ?? raw.arudha_lagna ?? 1,
    upapadaLagna: raw.upapadaLagna ?? raw.upapada_lagna ?? 1,
    specialLagnasSummary: raw.specialLagnasSummary ?? raw.special_lagnas_summary ?? [],
    vargaInterpretations: (raw.vargaInterpretations ?? raw.varga_interpretations ?? []).map((v: any) => ({
      planet: v.planet ?? "",
      isVargottama: v.isVargottama ?? v.is_vargottama ?? false,
      isPushkarNavamsa: v.isPushkarNavamsa ?? v.is_pushkar_navamsa ?? false,
      d9Rasi: v.d9Rasi ?? v.d9_rasi ?? 1,
      d10Rasi: v.d10Rasi ?? v.d10_rasi ?? 1,
      d60Rasi: v.d60Rasi ?? v.d60_rasi ?? 1,
      summary: v.summary ?? "",
      description: v.description ?? "",
      reasons: v.reasons ?? [],
    })),
    d9MarriageAnalysis: raw.d9MarriageAnalysis ?? raw.d9_marriage_analysis ?? "",
    d10CareerAnalysis: raw.d10CareerAnalysis ?? raw.d10_career_analysis ?? "",
  };
}

function normalizeDashaTimeline(list: any[]): any[] {
  return list.map((p: any) => ({
    type: "planet",
    lord: p.lord ?? "",
    startTime: p.startTime ?? p.start_time ?? "",
    endTime: p.endTime ?? p.end_time ?? "",
    subDashas: normalizeDashaTimeline(p.subDashas ?? p.sub_dashas ?? []),
    name: p.name,
  }));
}

export function normalizeGocharaSummary(raw: any): GocharaSummary {
  if (!raw) return { transits: [], sadeSati: "None" };
  return {
    transits: (raw.transits ?? raw.transits ?? []).map((t: any) => ({
      planet: t.planet ?? "",
      currentRasi: t.currentRasi ?? t.current_rasi ?? 1,
      houseFromMoon: t.houseFromMoon ?? t.house_from_moon ?? 1,
      isBeneficTransit: t.isBeneficTransit ?? t.is_benefic_transit ?? false,
      isBlocked: t.isBlocked ?? t.is_blocked ?? false,
      murti: t.murti ?? "Unknown",
      summary: t.summary ?? "",
      description: t.description ?? "",
      reasons: t.reasons ?? [],
    })),
    sadeSati: raw.sadeSati ?? raw.sade_sati ?? "None",
  };
}

export function normalizeVedicResult(raw: any): VedicAnalysisResult {
  if (!raw) return {} as VedicAnalysisResult;
  return {
    meta: {
      precision: raw.meta?.precision ?? "Exact",
      inputTime: raw.meta?.inputTime ?? raw.meta?.input_time ?? "",
      correctedTime: raw.meta?.correctedTime ?? raw.meta?.corrected_time ?? "",
      isDst: raw.meta?.isDst ?? raw.meta?.is_dst ?? false,
      dstOffsetHours: raw.meta?.dstOffsetHours ?? raw.meta?.dst_offset_hours ?? null,
      analysisTimezone: raw.meta?.analysisTimezone ?? raw.meta?.analysis_timezone ?? "",
    },
    report: normalizeVedicReport(raw.report),
    tajikaReport: raw.tajikaReport ?? raw.tajika_report ? {
      yearLord: raw.tajikaReport?.yearLord ?? raw.tajika_report?.year_lord ?? null,
      munthaRasi: raw.tajikaReport?.munthaRasi ?? raw.tajika_report?.muntha_rasi ?? 1,
      sahams: raw.tajikaReport?.sahams ?? raw.tajika_report?.sahams ?? [],
      harshaBalaSummary: raw.tajikaReport?.harshaBalaSummary ?? raw.tajika_report?.harsha_bala_summary ?? [],
      summary: raw.tajikaReport?.summary ?? raw.tajika_report?.summary ?? "",
    } : null,
    chart: raw.chart,
    annualChart: raw.annualChart ?? raw.annual_chart,
    gochara: normalizeGocharaSummary(raw.gochara),
    vargaNakshatraReports: raw.vargaNakshatraReports ?? raw.varga_nakshatra_reports,
  };
}

export function normalizeTransitResult(raw: any): TransitResult {
  if (!raw) return {} as TransitResult;
  const cc = deepToCamelCase(raw);
  return {
    meta: cc.meta || {},
    yearlyLuck: cc.yearlyLuck || {},
    monthlyLuck: cc.monthlyLuck || {},
    monthlyLucks: cc.monthlyLucks || [],
    dailyLuck: cc.dailyLuck || {},
    hourlyLuck: cc.hourlyLuck || {},
    currentAge: cc.currentAge ?? 0,
    currentFrame: cc.currentFrame ?? null,
    nearbyDiagnostics: cc.nearbyDiagnostics || [],
  };
}

export function normalizeSajuResult(raw: any): SajuAnalysisResult {
  if (!raw) return {} as SajuAnalysisResult;
  const cc = deepToCamelCase(raw);
  
  if (!cc.report) cc.report = {};
  
  return {
    meta: cc.meta || {},
    report: {
      ...cc.report,
      timeline: cc.report.timeline || [],
      simulationFrames: cc.report.simulationFrames || [],
      majorLuck: cc.report.majorLuck ? {
        ...cc.report.majorLuck,
        cycles: cc.report.majorLuck.cycles || []
      } : null,
      strength: {
        ...(cc.report.strength || {}),
        deukRyeong: cc.report.strength?.deukRyeong || { acquired: false },
        deukJi: cc.report.strength?.deukJi || { acquired: false },
        deukSi: cc.report.strength?.deukSi || { acquired: false },
        deukSe: cc.report.strength?.deukSe || { acquired: false, bijieCount: 0, yinxingCount: 0, shishangCount: 0, caishengCount: 0, guanxingCount: 0, supportRatio: 0 },
      },
      spiritMarkers: {
        ...(cc.report.spiritMarkers || {}),
        mappedMarkers: cc.report.spiritMarkers?.mappedMarkers || [],
        markers: cc.report.spiritMarkers?.markers || [],
        auspicious: cc.report.spiritMarkers?.auspicious || [],
        inauspicious: cc.report.spiritMarkers?.inauspicious || [],
        auxShinsals: cc.report.spiritMarkers?.auxShinsals || [],
      },
      power: {
        ...(cc.report.power || {}),
        elementScores: cc.report.power?.elementScores || [],
        tenGodScores: cc.report.power?.tenGodScores || [],
      }
    },
    lints: cc.lints || [],
    entropy: cc.entropy || null,
    qiTopology: cc.qiTopology || null,
    loadDiagnostics: cc.loadDiagnostics || [],
    crashCount: cc.crashCount || 0,
    vulnerabilityReport: cc.vulnerabilityReport ? {
      ...cc.vulnerabilityReport,
      criticalVectors: cc.vulnerabilityReport.criticalVectors || []
    } : null,
    complexity: cc.complexity || null,
    relationships: cc.relationships ? {
      ...cc.relationships,
      mappedRelationships: cc.relationships.mappedRelationships || [],
      stemCombinations: cc.relationships.stemCombinations || [],
      stemClashes: cc.relationships.stemClashes || [],
      tripleCombinations: cc.relationships.tripleCombinations || [],
      seasonalCombinations: cc.relationships.seasonalCombinations || [],
      dominantSemiCombinations: cc.relationships.dominantSemiCombinations || [],
      weakSemiCombinations: cc.relationships.weakSemiCombinations || [],
      sixCombinations: cc.relationships.sixCombinations || [],
      branchClashes: cc.relationships.branchClashes || [],
      branchPunishments: cc.relationships.branchPunishments || [],
      branchHarms: cc.relationships.branchHarms || [],
      branchDestructions: cc.relationships.branchDestructions || [],
    } : undefined,
    voidAnalysis: cc.voidAnalysis ? {
      ...cc.voidAnalysis,
      voidBranches: cc.voidAnalysis.voidBranches || [],
      voidPositions: cc.voidAnalysis.voidPositions || [],
      voidTenGods: cc.voidAnalysis.voidTenGods || [],
      mappedVoids: cc.voidAnalysis.mappedVoids || [],
    } : undefined,
  };
}
