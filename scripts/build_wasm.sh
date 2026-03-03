#!/bin/bash
# WASM 빌드 스크립트 — Vercel 배포용
# wasm-pack이 생성하는 .gitignore를 삭제하여 Git 트래킹 유지

set -e

export PATH="$HOME/.cargo/bin:$PATH"

echo "🔨 Building WASM..."
cd "$(dirname "$0")/../crates/eon-wasm"
wasm-pack build --target web --out-dir ../../app/src/lib/wasm

# wasm-pack이 자동 생성하는 .gitignore 삭제 (Git 트래킹 방해 방지)
GITIGNORE="../../app/src/lib/wasm/.gitignore"
if [ -f "$GITIGNORE" ]; then
    rm "$GITIGNORE"
    echo "🗑️  Removed auto-generated .gitignore from wasm output"
fi

echo "✅ WASM build complete! Don't forget to commit app/src/lib/wasm/"
