"""
Eon AGY Agent — Google Antigravity SDK 기반 운명 감사 에이전트

FastMCP 서버를 Stdio Transport로 연결하고 Gemini Flash로
자율적인 Tool Calling을 통해 한국어 심층 감사 리포트를 생성합니다.
"""
from __future__ import annotations

import asyncio
import os
from pathlib import Path

from google.antigravity import Agent, LocalAgentConfig, types

# MCP 서버 스크립트 경로
MCP_SERVER_PATH = Path(__file__).parent / "eon_mcp_server.py"

# 세션 유지 디렉토리 설정
IS_VERCEL = "VERCEL" in os.environ or "NOW_BUILD" in os.environ
if IS_VERCEL:
    SAVE_DIR = "/tmp/eon_conversations"
else:
    SAVE_DIR = str(Path(__file__).parents[2] / ".gemini" / "conversations")

os.makedirs(SAVE_DIR, exist_ok=True)

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
) -> tuple[str, str]:
    """
    AGY 에이전트를 실행하여 사주 감사 리포트를 생성합니다.

    Args:
        year: 출생 연도 (양력)
        month: 출생 월
        day: 출생 일
        hour: 출생 시 (0~23)
        is_male: 남성 여부
        api_key: Gemini API 키
        birth_name: 분석 대상자 이름 (선택)

    Returns:
        tuple[str, str]: (마크다운 형식의 감사 리포트, 대화 ID)
    """
    mcp_servers = [
        types.McpStdioServer(
            name="eon-mcp-server",
            command="python3",
            args=[str(MCP_SERVER_PATH)],
        )
    ]

    config = LocalAgentConfig(
        api_key=api_key,
        system_instructions=SYSTEM_INSTRUCTIONS,
        mcp_servers=mcp_servers,
        save_dir=SAVE_DIR,
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

    async with Agent(config) as agent:
        response = await agent.chat(prompt)
        report = await response.text()
        conv_id = agent.conversation_id

    return report, conv_id or ""


async def run_chat(
    conversation_id: str,
    message: str,
    api_key: str,
    year: int | None = None,
    month: int | None = None,
    day: int | None = None,
    hour: int | None = None,
    is_male: bool | None = None,
    birth_name: str = "분석 대상",
) -> tuple[str, str]:
    """
    기존 대화를 불러와서 사용자의 질문에 답변합니다.
    세션이 만료된 경우 새 대화를 시작하고 초기 감사를 수행한 뒤 답변합니다.
    """
    mcp_servers = [
        types.McpStdioServer(
            name="eon-mcp-server",
            command="python3",
            args=[str(MCP_SERVER_PATH)],
        )
    ]

    # 세션 파일 존재 확인
    session_file = Path(SAVE_DIR) / f"traj-{conversation_id}"
    
    if not session_file.exists() and year is not None and month is not None and day is not None and hour is not None and is_male is not None:
        # 세션 유실 시 복구 및 재생
        config = LocalAgentConfig(
            api_key=api_key,
            save_dir=SAVE_DIR,
            system_instructions=SYSTEM_INSTRUCTIONS,
            mcp_servers=mcp_servers,
        )
        gender_str = "남성" if is_male else "여성"
        init_prompt = f"""
분석 대상: {birth_name} ({gender_str})
생년월일시: {year}년 {month}월 {day}일 {hour}시 (양력)

위 사주에 대해 전체 도구를 순서대로 호출하여 심층 감사 리포트를 작성해 주세요.
"""
        async with Agent(config) as agent:
            await agent.chat(init_prompt)
            response = await agent.chat(message)
            reply = await response.text()
            return reply, agent.conversation_id or ""
    else:
        config = LocalAgentConfig(
            api_key=api_key,
            conversation_id=conversation_id,
            save_dir=SAVE_DIR,
            system_instructions=SYSTEM_INSTRUCTIONS,
            mcp_servers=mcp_servers,
        )
        async with Agent(config) as agent:
            response = await agent.chat(message)
            reply = await response.text()
            return reply, conversation_id


if __name__ == "__main__":
    # 로컬 테스트용 실행
    import sys

    api_key = os.environ.get("GEMINI_API_KEY", "")
    if not api_key:
        print("GEMINI_API_KEY 환경 변수를 설정해주세요.", file=sys.stderr)
        sys.exit(1)

    result, conv_id = asyncio.run(
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
    print("Conversation ID:", conv_id)
