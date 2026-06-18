"""
Eon GenAI Agent — Groq SDK 기반 운명 감사 에이전트

Groq API를 활용하여 Llama 3 등을 백엔드로 사용하며,
사주 분석 함수를 직접 도구로 바인딩하고 대화형 감사 리포트를 작성합니다.
"""
from __future__ import annotations

import asyncio
import json
import os
import sys

# eon_mcp_server에서 도구 직접 임포트
sys.path.insert(0, os.path.dirname(__file__))
from eon_mcp_server import analyze_entropy, scan_topology, fuzz_luck_vulnerabilities, backtrace_root_cause

from groq import AsyncGroq

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

TOOLS = [
    {
        "type": "function",
        "function": {
            "name": "analyze_entropy",
            "description": "사주의 에너지 난독화 등급 및 엔트로피 점수를 분석합니다.",
            "parameters": {
                "type": "object",
                "properties": {
                    "year": {"type": "integer", "description": "출생 연도 (양력)"},
                    "month": {"type": "integer", "description": "출생 월"},
                    "day": {"type": "integer", "description": "출생 일"},
                    "hour": {"type": "integer", "description": "출생 시 (0~23)"}
                },
                "required": ["year", "month", "day", "hour"]
            }
        }
    },
    {
        "type": "function",
        "function": {
            "name": "scan_topology",
            "description": "오행 네트워크의 트래픽 흐름, 대역폭 및 병목 구간을 분석합니다.",
            "parameters": {
                "type": "object",
                "properties": {
                    "year": {"type": "integer", "description": "출생 연도 (양력)"},
                    "month": {"type": "integer", "description": "출생 월"},
                    "day": {"type": "integer", "description": "출생 일"},
                    "hour": {"type": "integer", "description": "출생 시 (0~23)"}
                },
                "required": ["year", "month", "day", "hour"]
            }
        }
    },
    {
        "type": "function",
        "function": {
            "name": "fuzz_luck_vulnerabilities",
            "description": "특정 대운 컨텍스트에서 발생할 수 있는 취약점(크래시)를 탐색합니다.",
            "parameters": {
                "type": "object",
                "properties": {
                    "year": {"type": "integer"},
                    "month": {"type": "integer"},
                    "day": {"type": "integer"},
                    "hour": {"type": "integer"},
                    "is_male": {"type": "boolean"},
                    "major_ganzi_index": {"type": "integer", "description": "대운 간지 인덱스 (0~59)"}
                },
                "required": ["year", "month", "day", "hour"]
            }
        }
    },
    {
        "type": "function",
        "function": {
            "name": "backtrace_root_cause",
            "description": "특정 나이의 특정 상태에 대한 근본 원인을 역추적합니다.",
            "parameters": {
                "type": "object",
                "properties": {
                    "year": {"type": "integer"},
                    "month": {"type": "integer"},
                    "day": {"type": "integer"},
                    "hour": {"type": "integer"},
                    "target_age": {"type": "integer"},
                    "target_tag": {"type": "string", "description": "추적할 태그 (예: '기신', '충', '형')"}
                },
                "required": ["year", "month", "day", "hour", "target_age"]
            }
        }
    }
]

AVAILABLE_FUNCTIONS = {
    "analyze_entropy": analyze_entropy,
    "scan_topology": scan_topology,
    "fuzz_luck_vulnerabilities": fuzz_luck_vulnerabilities,
    "backtrace_root_cause": backtrace_root_cause,
}

async def execute_tools(messages, client, model="llama-3.3-70b-versatile"):
    """
    도구 호출 루프를 실행하여 최종 응답을 얻습니다.
    """
    max_loops = 10
    loops = 0
    while loops < max_loops:
        response = await client.chat.completions.create(
            model=model,
            messages=messages,
            tools=TOOLS,
            tool_choice="auto",
        )
        
        response_message = response.choices[0].message
        
        # Groq SDK에서 message 객체를 dict로 변환 (tool_calls가 없을 수 있음)
        msg_dict = {
            "role": response_message.role,
        }
        if response_message.content:
            msg_dict["content"] = response_message.content
            
        if response_message.tool_calls:
            msg_dict["tool_calls"] = [
                {
                    "id": tc.id,
                    "type": tc.type,
                    "function": {
                        "name": tc.function.name,
                        "arguments": tc.function.arguments,
                    }
                } for tc in response_message.tool_calls
            ]
            
        messages.append(msg_dict)
        
        if not response_message.tool_calls:
            # 도구 호출이 끝나면 리포트 완료
            return response_message.content or "", messages

        # 도구 실행
        for tool_call in response_message.tool_calls:
            function_name = tool_call.function.name
            function_to_call = AVAILABLE_FUNCTIONS.get(function_name)
            if function_to_call:
                try:
                    function_args = json.loads(tool_call.function.arguments)
                    function_response = function_to_call(**function_args)
                    messages.append({
                        "tool_call_id": tool_call.id,
                        "role": "tool",
                        "name": function_name,
                        "content": json.dumps(function_response, ensure_ascii=False),
                    })
                except Exception as e:
                    messages.append({
                        "tool_call_id": tool_call.id,
                        "role": "tool",
                        "name": function_name,
                        "content": json.dumps({"error": str(e)}, ensure_ascii=False),
                    })
            else:
                messages.append({
                    "tool_call_id": tool_call.id,
                    "role": "tool",
                    "name": function_name,
                    "content": json.dumps({"error": f"Unknown function {function_name}"}),
                })
        
        loops += 1

    return "도구 호출이 너무 많아 종료되었습니다.", messages


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
    Groq 에이전트를 실행하여 사주 감사 리포트와 대화 기록을 생성합니다.
    """
    client = AsyncGroq(api_key=api_key)

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

    messages = [
        {"role": "system", "content": SYSTEM_INSTRUCTIONS},
        {"role": "user", "content": prompt}
    ]

    report, final_history = await execute_tools(messages, client)
    return report, final_history


async def run_chat(
    history_data: list[dict],
    message: str,
    api_key: str,
) -> tuple[str, list[dict]]:
    """
    기존 대화 기록을 기반으로 사용자의 추가 질문에 답변합니다.
    """
    client = AsyncGroq(api_key=api_key)

    messages = []
    # system instruction이 없으면 추가
    if not any(m.get("role") == "system" for m in history_data):
        messages.append({"role": "system", "content": SYSTEM_INSTRUCTIONS})
        
    messages.extend(history_data)
    messages.append({"role": "user", "content": message})

    reply, final_history = await execute_tools(messages, client)
    return reply, final_history


if __name__ == "__main__":
    # 로컬 테스트용 실행
    api_key = os.environ.get("GROQ_API_KEY", "")
    if not api_key:
        print("GROQ_API_KEY 환경 변수를 설정해주세요.", file=sys.stderr)
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
