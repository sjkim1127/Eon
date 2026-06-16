"""
Eon MCP Server — FastMCP 기반 사주 분석 도구 서버

eon-ai Rust 크레이트의 Tool Manifest를 Python으로 재구현하여
MCP 프로토콜로 노출합니다. AGY 에이전트가 Stdio Transport로 연결합니다.
"""
from __future__ import annotations

import json
import math
from mcp.server.fastmcp import FastMCP

mcp = FastMCP("EonDestinyServer")

# ──────────────────────────────────────────────────────────────────────────────
# 공통 유틸리티
# ──────────────────────────────────────────────────────────────────────────────

STEMS = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"]
BRANCHES = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"]
ELEMENTS = {
    "甲": "木", "乙": "木",
    "丙": "火", "丁": "火",
    "戊": "土", "己": "土",
    "庚": "金", "辛": "金",
    "壬": "水", "癸": "水",
    "子": "水", "丑": "土", "寅": "木", "卯": "木",
    "辰": "土", "巳": "火", "午": "火", "未": "土",
    "申": "金", "酉": "金", "戌": "土", "亥": "水",
}
ELEMENT_KR = {"木": "목(木)", "火": "화(火)", "土": "토(土)", "金": "금(金)", "水": "수(水)"}

def ganzi_from_year(year: int) -> tuple[str, str]:
    """연도에서 천간(天干)·지지(地支) 반환"""
    idx = (year - 4) % 60
    return STEMS[idx % 10], BRANCHES[idx % 12]

def saju_pillars(year: int, month: int, day: int, hour: int) -> dict:
    """단순화된 사주 4주 계산 (연주 기반 근사치)"""
    y_stem, y_branch = ganzi_from_year(year)
    m_idx = (month - 1 + (year - 4) * 12) % 60
    m_stem, m_branch = STEMS[m_idx % 10], BRANCHES[m_idx % 12]
    d_offset = (year * 365 + (year // 4) + month * 30 + day) % 60
    d_stem, d_branch = STEMS[d_offset % 10], BRANCHES[d_offset % 12]
    h_idx = (hour // 2) % 12
    h_stem_base = (d_offset % 5) * 2
    h_stem = STEMS[(h_stem_base + h_idx) % 10]
    h_branch = BRANCHES[h_idx]
    return {
        "year": {"stem": y_stem, "branch": y_branch},
        "month": {"stem": m_stem, "branch": m_branch},
        "day": {"stem": d_stem, "branch": d_branch},
        "hour": {"stem": h_stem, "branch": h_branch},
    }

def count_elements(pillars: dict) -> dict[str, int]:
    counts: dict[str, int] = {"木": 0, "火": 0, "土": 0, "金": 0, "水": 0}
    for pillar in pillars.values():
        for char in [pillar["stem"], pillar["branch"]]:
            elem = ELEMENTS.get(char, "")
            if elem in counts:
                counts[elem] += 1
    return counts


# ──────────────────────────────────────────────────────────────────────────────
# MCP 도구 정의
# ──────────────────────────────────────────────────────────────────────────────

@mcp.tool()
def analyze_entropy(
    year: int,
    month: int,
    day: int,
    hour: int,
) -> dict:
    """사주의 에너지 난독화 등급 및 엔트로피 점수를 분석합니다.

    오행 분포의 균일성(Shannon Entropy)을 계산하여 사주 구조의
    복잡도와 예측 불가능성을 수치화합니다.

    Args:
        year: 출생 연도 (양력)
        month: 출생 월
        day: 출생 일
        hour: 출생 시 (0~23)
    """
    pillars = saju_pillars(year, month, day, hour)
    counts = count_elements(pillars)
    total = sum(counts.values())
    if total == 0:
        return {"level": "Unknown", "score": 0.0, "distribution": counts}

    # Shannon Entropy H = -∑ P(x) * log2(P(x))
    entropy = 0.0
    for cnt in counts.values():
        if cnt > 0:
            p = cnt / total
            entropy -= p * math.log2(p)

    # 최대 엔트로피 = log2(5) ≈ 2.322 (5원소 균등 분포)
    max_entropy = math.log2(5)
    normalized = entropy / max_entropy  # 0~1

    if normalized >= 0.85:
        level = "CRITICAL (극도로 복잡)"
    elif normalized >= 0.65:
        level = "HIGH (높은 복잡도)"
    elif normalized >= 0.40:
        level = "MEDIUM (중간 복잡도)"
    else:
        level = "LOW (단순 구조)"

    dominant = max(counts, key=lambda k: counts[k])
    weak = min(counts, key=lambda k: counts[k])

    return {
        "entropy_score": round(normalized, 3),
        "raw_entropy": round(entropy, 3),
        "level": level,
        "distribution": {ELEMENT_KR.get(k, k): v for k, v in counts.items()},
        "dominant_element": ELEMENT_KR.get(dominant, dominant),
        "weakest_element": ELEMENT_KR.get(weak, weak),
        "pillars": pillars,
    }


@mcp.tool()
def scan_topology(
    year: int,
    month: int,
    day: int,
    hour: int,
) -> dict:
    """오행 네트워크의 트래픽 흐름, 대역폭 및 병목 구간을 분석합니다.

    오행 상생·상극 관계를 네트워크 위상으로 모델링하여
    에너지 흐름의 원활도와 병목 지점을 진단합니다.

    Args:
        year: 출생 연도 (양력)
        month: 출생 월
        day: 출생 일
        hour: 출생 시 (0~23)
    """
    pillars = saju_pillars(year, month, day, hour)
    counts = count_elements(pillars)

    # 상생(生) 관계: 木→火→土→金→水→木
    GENERATES = {"木": "火", "火": "土", "土": "金", "金": "水", "水": "木"}
    # 상극(剋) 관계: 木→土, 土→水, 水→火, 火→金, 金→木
    CONTROLS  = {"木": "土", "土": "水", "水": "火", "火": "金", "金": "木"}

    flow_analysis = []
    bottlenecks = []
    conflicts = []

    for src, tgt in GENERATES.items():
        src_cnt = counts.get(src, 0)
        tgt_cnt = counts.get(tgt, 0)
        bandwidth = src_cnt  # 에너지 공급량
        demand = tgt_cnt     # 수요량

        flow_ratio = (bandwidth / max(demand, 1)) if demand > 0 else bandwidth
        status = "원활" if 0.5 <= flow_ratio <= 2.0 else ("과잉 공급" if flow_ratio > 2.0 else "공급 부족")
        flow_analysis.append({
            "from": ELEMENT_KR.get(src, src),
            "to": ELEMENT_KR.get(tgt, tgt),
            "relation": "상생(生)",
            "bandwidth": bandwidth,
            "demand": demand,
            "status": status,
        })
        if status != "원활":
            bottlenecks.append(f"{ELEMENT_KR.get(src, src)} → {ELEMENT_KR.get(tgt, tgt)}: {status}")

    for attacker, target in CONTROLS.items():
        atk_cnt = counts.get(attacker, 0)
        tgt_cnt = counts.get(target, 0)
        if atk_cnt > 0 and tgt_cnt > 0:
            severity = "심각" if atk_cnt >= 2 and tgt_cnt <= 1 else "보통"
            conflicts.append({
                "attacker": ELEMENT_KR.get(attacker, attacker),
                "target": ELEMENT_KR.get(target, target),
                "relation": "상극(剋)",
                "severity": severity,
                "attacker_count": atk_cnt,
                "target_count": tgt_cnt,
            })

    overall = "안정" if len(bottlenecks) <= 1 and len([c for c in conflicts if c["severity"] == "심각"]) == 0 else "불안정"

    return {
        "overall_status": overall,
        "flow_analysis": flow_analysis,
        "bottlenecks": bottlenecks,
        "conflicts": conflicts,
        "conflict_count": len(conflicts),
        "bottleneck_count": len(bottlenecks),
    }


@mcp.tool()
def fuzz_luck_vulnerabilities(
    year: int,
    month: int,
    day: int,
    hour: int,
    is_male: bool = True,
    major_ganzi_index: int = 0,
) -> dict:
    """특정 대운 컨텍스트에서 발생할 수 있는 취약점(크래시)를 탐색합니다.

    퍼징(Fuzzing) 기법으로 대운과 원국의 충돌 지점을 탐색하여
    가장 위험한 세운 조합과 취약점 유형을 보고합니다.

    Args:
        year: 출생 연도 (양력)
        month: 출생 월
        day: 출생 일
        hour: 출생 시 (0~23)
        is_male: 남성 여부
        major_ganzi_index: 대운 간지 인덱스 (0~59)
    """
    pillars = saju_pillars(year, month, day, hour)
    base_counts = count_elements(pillars)

    # 대운 간지
    major_stem = STEMS[major_ganzi_index % 10]
    major_branch = BRANCHES[major_ganzi_index % 12]
    major_elem_stem = ELEMENTS.get(major_stem, "")
    major_elem_branch = ELEMENTS.get(major_branch, "")

    # 대운이 원국에 미치는 영향 분석
    CONTROLS = {"木": "土", "土": "水", "水": "火", "火": "金", "金": "木"}

    crashes = []
    for elem in [major_elem_stem, major_elem_branch]:
        if not elem:
            continue
        # 이 대운 원소가 원국의 어떤 원소를 극하는가
        controlled = CONTROLS.get(elem, "")
        if controlled and base_counts.get(controlled, 0) >= 1:
            score = max(5.0, 30.0 - base_counts.get(controlled, 0) * 5.0)
            crashes.append({
                "type": f"대운_극충 ({ELEMENT_KR.get(elem, elem)} → {ELEMENT_KR.get(controlled, controlled)})",
                "crash_score": round(score, 1),
                "severity": "치명적" if score <= 10 else ("위험" if score <= 20 else "주의"),
                "description": f"대운 {major_stem}{major_branch}의 {ELEMENT_KR.get(elem, elem)} 기운이 원국의 {ELEMENT_KR.get(controlled, controlled)}을(를) 극합니다.",
            })

    # 60개 세운 퍼징
    for ganzi_idx in range(60):
        s = STEMS[ganzi_idx % 10]
        b = BRANCHES[ganzi_idx % 12]
        s_elem = ELEMENTS.get(s, "")
        b_elem = ELEMENTS.get(b, "")
        base_elem = ELEMENTS.get(pillars["day"]["stem"], "")
        controlled_by_s = CONTROLS.get(s_elem, "")
        controlled_by_b = CONTROLS.get(b_elem, "")
        if controlled_by_s == base_elem or controlled_by_b == base_elem:
            score = 15.0 + (ganzi_idx % 20) * 0.5
            crashes.append({
                "type": f"일주_공격 (세운 {s}{b})",
                "crash_score": round(score, 1),
                "severity": "위험" if score <= 20 else "주의",
                "description": f"세운 {s}{b}이 일주 {pillars['day']['stem']}{pillars['day']['branch']}를 직접 충극합니다.",
            })

    crashes.sort(key=lambda x: x["crash_score"])
    critical = [c for c in crashes if c["crash_score"] <= 20]

    return {
        "major_ganzi": f"{major_stem}{major_branch}",
        "total_crashes": len(crashes),
        "critical_crashes": len(critical),
        "top_vulnerabilities": crashes[:5],
        "fuzz_summary": f"대운 {major_stem}{major_branch} 컨텍스트에서 총 {len(crashes)}개의 취약점 발견, 치명적/위험 {len(critical)}건",
    }


@mcp.tool()
def backtrace_root_cause(
    year: int,
    month: int,
    day: int,
    hour: int,
    target_age: int,
    target_tag: str = "기신",
) -> dict:
    """특정 나이의 특정 상태에 대한 근본 원인을 역추적합니다.

    TTD(Time-Travel Debugging) 방식으로 특정 나이의 상태를
    원국으로부터 역추적하여 근본 원인을 진단합니다.

    Args:
        year: 출생 연도 (양력)
        month: 출생 월
        day: 출생 일
        hour: 출생 시 (0~23)
        target_age: 분석할 나이
        target_tag: 추적할 태그 (예: '기신', '충', '형')
    """
    pillars = saju_pillars(year, month, day, hour)
    counts = count_elements(pillars)

    # 대운 계산 (근사: 남녀 미분)
    major_luck_age = 8  # 대운 시작 나이 (근사)
    current_major_idx = max(0, (target_age - major_luck_age) // 10)
    base_major_idx = (year - 4 + current_major_idx * 3) % 60
    major_stem = STEMS[base_major_idx % 10]
    major_branch = BRANCHES[base_major_idx % 12]

    CONTROLS = {"木": "土", "土": "水", "水": "火", "火": "金", "金": "木"}
    WEAKNESSES = {k: v for k, v in CONTROLS.items()}

    day_stem = pillars["day"]["stem"]
    day_elem = ELEMENTS.get(day_stem, "")

    # 근본 원인 분석
    root_causes = []

    # 1. 원국 구조적 취약점
    weak_elem = min(counts, key=lambda k: counts[k])
    if counts[weak_elem] == 0:
        root_causes.append({
            "layer": "원국 구조",
            "cause": f"{ELEMENT_KR.get(weak_elem, weak_elem)} 결핍",
            "description": f"원국에 {ELEMENT_KR.get(weak_elem, weak_elem)} 기운이 전혀 없어 해당 운에서 심한 충격을 받습니다.",
            "severity": "근본적 취약점",
        })

    # 2. 대운 충돌
    major_elem_s = ELEMENTS.get(major_stem, "")
    if CONTROLS.get(major_elem_s, "") == day_elem:
        root_causes.append({
            "layer": "대운 충격",
            "cause": f"대운 {major_stem}{major_branch}의 일주 공격",
            "description": f"현재 대운({major_stem}{major_branch})의 {ELEMENT_KR.get(major_elem_s, '')}이 일주 {day_stem}의 {ELEMENT_KR.get(day_elem, '')}을 극하고 있습니다.",
            "severity": "고위험",
        })

    # 3. 태그별 분석
    tag_analysis = {
        "기신": f"일간 {ELEMENT_KR.get(day_elem, '')}을 극하는 {ELEMENT_KR.get(CONTROLS.get(day_elem, ''), '')} 기운이 {target_age}세 운에서 강화됩니다.",
        "충": f"{target_age}세 무렵 대운·세운의 지지 충(冲) 작용이 원국 지지와 충돌을 일으킵니다.",
        "형": f"{target_age}세 운에서 삼형(三刑) 또는 자형(自刑) 패턴이 활성화됩니다.",
    }

    if not root_causes:
        root_causes.append({
            "layer": "복합 요인",
            "cause": "다중 스트레스 중첩",
            "description": tag_analysis.get(target_tag, f"{target_tag} 상태의 복합적 원인이 {target_age}세에 수렴합니다."),
            "severity": "중간",
        })

    return {
        "target_age": target_age,
        "target_tag": target_tag,
        "current_major_luck": f"{major_stem}{major_branch} ({major_luck_age + current_major_idx * 10}~{major_luck_age + (current_major_idx + 1) * 10}세)",
        "day_pillar": f"{pillars['day']['stem']}{pillars['day']['branch']} ({ELEMENT_KR.get(day_elem, '')} 일간)",
        "root_causes": root_causes,
        "backtrace_summary": f"{target_age}세 {target_tag} 상태의 근본 원인: {root_causes[0]['cause']}",
        "recommended_countermeasure": f"용신({ELEMENT_KR.get(CONTROLS.get(CONTROLS.get(day_elem, ''), ''), '일간 보강')}) 강화로 저항력을 높이세요.",
    }


if __name__ == "__main__":
    mcp.run()
