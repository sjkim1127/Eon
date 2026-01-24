# 🌌 Eon: The Destiny Reversing Engine

> **"Fate is not a prophecy; it is an Executable Binary."**

`Eon`은 사주 명리학을 포함한 다양한 운명 체계를 **시스템 공학(System Engineering)** 및 **리버스 엔지니어링(Reverse Engineering)**의 관점에서 재해석하고 분석하는 차세대 운명 분석 플랫폼입니다.

단순한 점술 소프트웨어를 넘어, 인생의 데이터를 소스 코드로 간주하고 이를 실행, 디버깅, 최적화할 수 있는 강력한 툴체인을 제공합니다.

---

## 🛠️ The Reversing Toolkit (Core Features)

Eon은 리버스 엔지니어링 도구에서 영감을 받은 6가지 핵심 분석 엔진을 탑재하고 있습니다.

### 1. 🖥️ Saju-VM (Deterministic Execution)

사주 팔자를 가상 머신의 레지스터와 인스트럭션으로 변환하여 실행합니다. 100년의 인생 경로를 0.01초 만에 시뮬레이션하고 매년의 상태 스냅샷(`LifeFrame`)을 생성합니다.

### 2. 🕰️ Destiny TTD (Time Travel Debugging)

윈도우 디버거의 TTD(Time Travel Debugging) 철학을 적용했습니다.

- **Backtrace**: 특정 인생의 위기 지점에서 역방향으로 실행을 추적하여 근본 원인(`Root Cause`)인 '운의 진입점'을 찾아냅니다.
- **Life Diff**: 환경 변수(시간, 장소)를 수정한 평행 우주를 시뮬레이션하고 두 인생 경로 사이의 델타값을 비교합니다.

### 3. 🔍 DIE (Destiny It Easy)

인명 도구 **DIE(Detect It Easy)**를 사주에 이식했습니다.

- **Entropy Analysis**: 오행의 무질서도를 Shannon Entropy로 계산하여 인생의 복잡도를 정량화합니다.
- **Packer Detection**: 에너지가 묶여(Packed) 발현되지 않는 상태를 탐지하고, 이를 해제할 '언패킹 키(Unpacker)'를 제시합니다.

### 4. 🛡️ Destiny Fuzzer (Security Audit)

대운과 세운의 모든 조합을 무차별 대입(Fuzzing)하여 시스템 '크래시'(치명적 흉운)가 발생하는 취약점을 사전에 탐지하고 취약점 리포트를 생성합니다.

### 5. 🕸️ Qi Topology & LoadBalancer

인생의 에너지 유량을 네트워크 트래픽으로 관리합니다.

- **Topology**: 오행 네트워크의 대역폭(Bandwidth)과 병목(Bottleneck) 구간을 분석합니다.
- **LoadBalancer**: 운의 급격한 변화(Traffic Spike) 시 시스템 과부하를 막기 위한 제어 전략을 제안합니다.

### 6. 📝 Destiny Linter & Complexity

- **Static Analysis**: 사주 구조의 결함을 린트 에러(Warning/Error)로 출력합니다.
- **Cyclomatic Complexity**: 인생 경로의 결정 노드 수와 복잡성을 계산하여 '삶의 난이도'를 등급화합니다.

---

## 🏗️ Project Architecture

```text
crates/
├── eon-saju/         # 핵심 분석 엔진 (VM, TTD, DIE, Fuzzer, Topology 등)
├── eon-core/         # 공통 타입 및 오행 로직
├── eon-astro/        # Swiss Ephemeris 기반 초정밀 천문 연산 (예정)
└── eon-data/         # 만세력 및 정적 리소스 데이터
```

---

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/) (latest stable)

### Installation & Demo

저장소를 클론한 후, 모든 분석 엔진이 연쇄적으로 작동하는 통합 예제를 실행해 보세요.

```bash
git clone https://github.com/sjkim1127/Eon.git
cd Eon
cargo run --example verify_user
```

---

## 🔮 Future Roadmap: The Universal OS

Eon은 사주를 넘어선 **컴파일러 기반 통합 분석**을 목표로 합니다.

- **Vedic AST**: 베딕 점성학의 정밀 스케줄러 통합.
- **Human Design Circuitry**: 바디그래프의 하드웨어 회로 분석 연동.
- **I Ching Logic Gates**: 주역의 64괘를 6비트 논리 게이트로 구현.
- **AI Analyst**: 정밀하게 정제된 No-AI 분석 데이터를 기반으로 한 대형 언어 모델(LLM)의 고도화된 통찰 리포트 생성.

---

## 📜 License

This project is licensed under the [MIT License](LICENSE).

---
> **"Debug your fate, Optimize your life."**
