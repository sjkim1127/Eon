import type { VedicAnalysisResult, VedicAnalysisReport, GocharaSummary } from "../types/vedic";
import type { SajuAnalysisResult, TransitResult } from "../types/saju";

/**
 * Normalizes a Vedic analysis report by ensuring all fields are present.
 */
export function normalizeVedicReport(raw: any): VedicAnalysisReport {
  if (!raw) return {} as VedicAnalysisReport;
  const cc = raw;

  return {
    ...cc,
    primaryKarakas: {
      atmakaraka: cc.primaryKarakas?.atmakaraka ?? "Sun",
      amatyakaraka: cc.primaryKarakas?.amatyakaraka ?? "Sun",
      bhratrukaraka: cc.primaryKarakas?.bhratrukaraka ?? null,
      matrukaraka: cc.primaryKarakas?.matrukaraka ?? null,
      pitrikaraka: cc.primaryKarakas?.pitrikaraka ?? null,
      putrakaraka: cc.primaryKarakas?.putrakaraka ?? null,
      gnatikaraka: cc.primaryKarakas?.gnatikaraka ?? null,
      darakaraka: cc.primaryKarakas?.darakaraka ?? "Sun",
    },
    houseSummary: cc.houseSummary ?? [],
    dashaFocus: cc.dashaFocus ?? "",
    dashaTimeline: cc.dashaTimeline ?? [],
    yoginiTimeline: cc.yoginiTimeline ?? [],
    charaDashaTimeline: cc.charaDashaTimeline ?? [],
    allKarakas: cc.allKarakas ?? [],
    nakshatraInfo: cc.nakshatraInfo ?? "",
    overallStrengthScore: cc.overallStrengthScore ?? 0,
    sadeSati: cc.sadeSati ?? "None",
    yogas: cc.yogas ?? [],
    arudhaLagna: cc.arudhaLagna ?? 1,
    upapadaLagna: cc.upapadaLagna ?? 1,
    specialLagnasSummary: cc.specialLagnasSummary ?? [],
    vargaInterpretations: cc.vargaInterpretations ?? [],
    d9MarriageAnalysis: cc.d9MarriageAnalysis ?? "",
    d10CareerAnalysis: cc.d10CareerAnalysis ?? "",
  };
}

export function normalizeGocharaSummary(raw: any): GocharaSummary {
  if (!raw) return { transits: [], sadeSati: "None" };
  const cc = raw;
  return {
    transits: cc.transits ?? [],
    sadeSati: cc.sadeSati ?? "None",
  };
}

export function normalizeVedicResult(raw: any): VedicAnalysisResult {
  if (!raw) return {} as VedicAnalysisResult;
  const cc = raw;
  
  return {
    ...cc,
    meta: cc.meta || {},
    report: normalizeVedicReport(raw.report),
    tajikaReport: cc.tajikaReport ?? null,
    chart: cc.chart,
    annualChart: cc.annualChart ?? null,
    gochara: normalizeGocharaSummary(raw.gochara),
    vargaNakshatraReports: cc.vargaNakshatraReports,
  };
}

export function normalizeTransitResult(raw: any): TransitResult {
  if (!raw) return {} as TransitResult;
  const cc = raw;
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
  const cc = raw;
  
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
