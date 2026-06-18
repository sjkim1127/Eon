"""
Vercel 서버리스 함수 — POST /api/ai-audit

사주 정보와 Groq API 키를 받아 AGY 에이전트를 실행하고
AI 감사 리포트를 반환합니다.

API 키는 클라이언트에서 요청 헤더로 전달받아 서버에 저장하지 않습니다.
"""
from __future__ import annotations

import asyncio
import json
import sys
import os
from http.server import BaseHTTPRequestHandler

# Vercel Python 런타임 경로 설정
sys.path.insert(0, os.path.join(os.path.dirname(__file__), "mcp"))

from eon_agent import run_audit, run_chat


class handler(BaseHTTPRequestHandler):
    def do_OPTIONS(self):
        self.send_response(200)
        self._set_cors_headers()
        self.end_headers()

    def do_POST(self):
        try:
            content_length = int(self.headers.get("Content-Length", 0))
            body = self.rfile.read(content_length)
            data = json.loads(body)
        except (json.JSONDecodeError, ValueError) as e:
            self._error(400, f"잘못된 요청 형식: {e}")
            return

        # Groq API 키: 요청 헤더에서 수신 (서버에 저장하지 않음)
        api_key = self.headers.get("X-Groq-Api-Key", "").strip()
        if not api_key:
            self._error(401, "X-Groq-Api-Key 헤더가 필요합니다.")
            return

        action = data.get("action", "audit")

        if action == "chat":
            # 대화 모드
            history = data.get("history")
            message = data.get("message")
            if history is None or not message:
                self._error(400, "대화 모드에서는 history와 message가 필수입니다.")
                return

            try:
                reply, updated_history = asyncio.run(
                    run_chat(
                        history_data=history,
                        message=message,
                        api_key=api_key,
                    )
                )
            except Exception as e:
                self._error(500, f"에이전트 대화 실행 오류: {e}")
                return

            self.send_response(200)
            self._set_cors_headers()
            self.send_header("Content-Type", "application/json; charset=utf-8")
            self.end_headers()
            response_body = json.dumps(
                {"reply": reply, "history": updated_history, "status": "success"},
                ensure_ascii=False,
            )
            self.wfile.write(response_body.encode("utf-8"))
            return

        else:
            # 기본 모드: 최초 심층 감사 리포트 생성
            # 필수 파라미터 검증
            required = ["year", "month", "day", "hour"]
            for field in required:
                if field not in data:
                    self._error(400, f"필수 파라미터 누락: {field}")
                    return

            try:
                year = int(data["year"])
                month = int(data["month"])
                day = int(data["day"])
                hour = int(data["hour"])
                is_male = bool(data.get("isMale", True))
                birth_name = str(data.get("birthName", "분석 대상"))
            except (ValueError, TypeError) as e:
                self._error(400, f"파라미터 타입 오류: {e}")
                return

            # 범위 검증
            if not (1 <= month <= 12 and 1 <= day <= 31 and 0 <= hour <= 23):
                self._error(400, "날짜/시간 범위가 올바르지 않습니다.")
                return

            try:
                report, history = asyncio.run(
                    run_audit(
                        year=year,
                        month=month,
                        day=day,
                        hour=hour,
                        is_male=is_male,
                        api_key=api_key,
                        birth_name=birth_name,
                    )
                )
            except Exception as e:
                self._error(500, f"에이전트 실행 오류: {e}")
                return

            self.send_response(200)
            self._set_cors_headers()
            self.send_header("Content-Type", "application/json; charset=utf-8")
            self.end_headers()
            response_body = json.dumps(
                {"report": report, "history": history, "status": "success"},
                ensure_ascii=False,
            )
            self.wfile.write(response_body.encode("utf-8"))

    def _set_cors_headers(self):
        self.send_header("Access-Control-Allow-Origin", "*")
        self.send_header("Access-Control-Allow-Methods", "POST, OPTIONS")
        self.send_header("Access-Control-Allow-Headers", "Content-Type, X-Groq-Api-Key")

    def _error(self, code: int, message: str):
        self.send_response(code)
        self._set_cors_headers()
        self.send_header("Content-Type", "application/json; charset=utf-8")
        self.end_headers()
        body = json.dumps({"error": message, "status": "error"}, ensure_ascii=False)
        self.wfile.write(body.encode("utf-8"))

    def log_message(self, format, *args):
        pass  # Vercel 로그 억제
