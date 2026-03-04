#!/bin/bash
# WASM 빌드 스크립트 — Vercel 배포 및 로컬 개발 동시 지원
#
# 출력 디렉토리가 두 곳인 이유:
#   - pkg/              → node_modules/eon-wasm 심볼릭 링크 대상 (로컬 npm 패키지)
#   - app/src/lib/wasm/ → git 트래킹 (Vercel 배포 시 사용)

set -e

export PATH="$HOME/.cargo/bin:$PATH"

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
CRATE_DIR="$SCRIPT_DIR/../crates/eon-wasm"

echo "🔨 Building WASM (pkg/ — 로컬 npm 패키지)..."
cd "$CRATE_DIR"
wasm-pack build --target web

echo "📋 Syncing to app/src/lib/wasm/ (Vercel 배포용)..."
OUT_DIR="$SCRIPT_DIR/../app/src/lib/wasm"
mkdir -p "$OUT_DIR"
cp pkg/eon_wasm_bg.wasm "$OUT_DIR/"
cp pkg/eon_wasm.js     "$OUT_DIR/"
cp pkg/eon_wasm.d.ts   "$OUT_DIR/"
cp pkg/eon_wasm_bg.wasm.d.ts "$OUT_DIR/"
cp pkg/package.json    "$OUT_DIR/"

# wasm-pack이 app/src/lib/wasm/ 에 .gitignore 생성 시 삭제
GITIGNORE="$OUT_DIR/.gitignore"
if [ -f "$GITIGNORE" ]; then
    rm "$GITIGNORE"
    echo "🗑️  Removed auto-generated .gitignore from wasm output"
fi

echo "✅ WASM build complete!"
echo "   pkg/              → node_modules/eon-wasm (로컬 개발)"
echo "   app/src/lib/wasm/ → git 트래킹 (Vercel 배포)"
echo ""
echo "⚠️  다음 단계: git add app/src/lib/wasm/ && git commit"
