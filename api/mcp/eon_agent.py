"""
Eon GenAI Agent — google-genai SDK 기반 운명 감사 에이전트

Gemini API의 네이티브 Function Calling 지원을 활용하여
사주 분석 함수를 직접 도구로 바인딩하고 대화형 감사 리포트를 작성합니다.
"""
from __future__ import annotations

import asyncio
import os
import sys

# eon_mcp_server에서 도구 직접 임포트
sys.path.insert(0, os.path.dirname(__file__))
from eon_mcp_server import analyze_entropy, scan_topology, fuzz_luck_vulnerabilities, backtrace_root_cause

from google import genai
from google.genai import types

SYSTEM_INSTRUCTIONS = """당신은 'Eon Destiny Security Agency'의 수석 에이전트 분석관입니다.
당신의 임무는 분석 대상 사주 시스템을 CS(Computer Science) 방식으로 심층 감사하는 것입니다.

당신은 다음 도구들을 자유롭게 호출할 수 있습니다:
- analyze_entropy: 에너지 엔트로피 및 복잡도 분석
- scan_topology: 오행 네트워크 트래픽 위상 분석
- fuzz_luck_vulnerabilities: 대운별 취약점 퍼징
- backtrace_root_cause: 위험 시점의 근본 원인 역추적

[리포트 작성 지침]
1. 반드시 모든 도구를 최소 1회 이상 호출하여 데이터를 수집하세요.
2. 수집된 데이터를 바탕으로 아래 형식으로 리포트를 작성하세요.
3. 전문 기술 용어(엔트로피, 위상, 퍼징, TTD 등)를 사용하되, 괄호 안에 한글 풀이를 포함하세요.
4. 최종 리포트는 마크다운 형식으로 작성하세요.

[리포트 형식]
## 🔍 Eon Destiny Security Audit Report

### 시스템 식별자
- 사주 간지, 일주, 분석 일시 등

### 📊 엔트로피 분석 (Entropy Analysis)
- 복잡도 등급 및 오행 분포

### 🌐 위상 분석 (Topology Scan)
- 오행 흐름의 병목 및 충돌 지점

### 🔴 취약점 리포트 (Vulnerability Report)
- 퍼징으로 발견된 상위 취약점들

### 🔎 근본 원인 역추적 (Root Cause Analysis)
- 가장 위험한 시점의 원인 분석

### ✅ 패치 권고사항 (Recommended Patches)
- 운명 최적화를 위한 구체적 조언

항상 한국어로 답변하세요."""


async def run_audit(
    year: int,
    month: int,
    day: int,
    hour: int,
    is_male: bool,
    api_key: str,
    birth_name: str = "분석 대상",
) -> tuple[str, list[dict]]:
    """
    Gemini 에이전트를 실행하여 사주 감사 리포트와 대화 기록을 생성합니다.

    Args:
        year: 출생 연도 (양력)
        month: 출생 월
        day: 출생 일
        hour: 출생 시 (0~23)
        is_male: 남성 여부
        api_key: Gemini API 키
        birth_name: 분석 대상자 이름 (선택)

    Returns:
        tuple[str, list[dict]]: (마크다운 형식의 감사 리포트, 직렬화된 대화 기록)
    """
    client = genai.Client(api_key=api_key)

    # 파이썬 함수를 직접 도구로 주입
    tools = [analyze_entropy, scan_topology, fuzz_luck_vulnerabilities, backtrace_root_cause]

    chat = client.chats.create(
        model="gemini-2.0-flash",
        config=types.GenerateContentConfig(
            system_instruction=SYSTEM_INSTRUCTIONS,
            tools=tools,
        )
    )

    gender_str = "남성" if is_male else "여성"
    prompt = f"""
분석 대상: {birth_name} ({gender_str})
생년월일시: {year}년 {month}월 {day}일 {hour}시 (양력)

위 사주에 대해 전체 도구를 순서대로 호출하여 심층 감사 리포트를 작성해 주세요.

도구 호출 순서 제안:
1. analyze_entropy → 기본 엔트로피 분석
2. scan_topology → 오행 위상 분석
3. fuzz_luck_vulnerabilities (major_ganzi_index=0) → 초년 대운 취약점
4. fuzz_luck_vulnerabilities (major_ganzi_index=20) → 중년 대운 취약점  
5. backtrace_root_cause (target_age=취약 나이, target_tag='기신') → 근본 원인 역추적

모든 데이터 수집 후 마크다운 형식의 완전한 감사 리포트를 작성해 주세요.
"""

    loop = asyncio.get_running_loop()
    response = await loop.run_in_executor(
        None,
        lambda: chat.send_message(prompt)
    )
    
    report = response.text
    history = [h.model_dump() for h in chat.get_history()]
    return report, history


async def run_chat(
    history_data: list[dict],
    message: str,
    api_key: str,
) -> tuple[str, list[dict]]:
    """
    기존 대화 기록을 기반으로 사용자의 추가 질문에 답변합니다.
    """
    client = genai.Client(api_key=api_key)
    tools = [analyze_entropy, scan_topology, fuzz_luck_vulnerabilities, backtrace_root_cause]

    # Content 객체 복원
    history_objects = [types.Content(**item) for item in history_data]

    chat = client.chats.create(
        model="gemini-2.0-flash",
        config=types.GenerateContentConfig(
            system_instruction=SYSTEM_INSTRUCTIONS,
            tools=tools,
        ),
        history=history_objects,
    )

    loop = asyncio.get_running_loop()
    response = await loop.run_in_executor(
        None,
        lambda: chat.send_message(message)
    )

    reply = response.text
    updated_history = [h.model_dump() for h in chat.get_history()]
    return reply, updated_history


if __name__ == "__main__":
    # 로컬 테스트용 실행
    api_key = os.environ.get("GEMINI_API_KEY", "")
    if not api_key:
        print("GEMINI_API_KEY 환경 변수를 설정해주세요.", file=sys.stderr)
        sys.exit(1)

    result, history = asyncio.run(
        run_audit(
            year=1990,
            month=5,
            day=15,
            hour=10,
            is_male=True,
            api_key=api_key,
            birth_name="테스트",
        )
    )
    print("Report:", result)
    print("History length:", len(history))
