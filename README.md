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

### 7. 🌌 Eon-Vedic (BPHS Native Engine)

베딕 점성학(Vedic Astrology)의 정밀 기법을 BPHS(Brihat Parashara Hora Shastra) 표준에 맞춰 구현한 초정밀 연산 엔진입니다.

- **Shadbala Precision**: 행성 간 전쟁(Yuddha Bala)의 실제 세기 차이 반영 및 특수 시각(Special Aspects)의 선형 보간(Gradient)을 통한 Graha Bala 정밀 계산.
- **Bhava Bala (House Strength)**: 하우스 주인(Lord), 방위(Dig), 그리고 길흉 행성의 시각(Drishti) 가중치를 결합한 다각적 하우스 강도 분석.
- **Ashtakavarga Refinement**: 행성별 bindu 감쇄(Reduction) 로직을 BPHS 표준에 완벽히 동기화하여 분석의 정확도를 확보했습니다.

---

## 🏗️ Project Architecture

```text
crates/
├── eon-saju/         # 사주 핵심 분석 엔진 (VM, TTD, DIE, Fuzzer 등)
├── eon-vedic/        # 베딕 점성학 분석 엔진 (Shadbala, Bhava Bala, Ashtakavarga)
├── eon-core/         # 공통 타입 및 오행/점성 기초 로직
├── eon-astro/        # Swiss Ephemeris 기반 초정밀 천문 연산
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
cargo run --package eon-vedic --example calculate_strength
```

---

## 🔮 Future Roadmap: The Universal OS

Eon은 사주를 넘어선 **컴파일러 기반 통합 분석**을 목표로 합니다.

- **Human Design Circuitry**: 바디그래프의 하드웨어 회로 분석 연동.
- **I Ching Logic Gates**: 주역의 64괘를 6비트 논리 게이트로 구현.
- **AI Analyst**: 정밀하게 정제된 No-AI 분석 데이터를 기반으로 한 대형 언어 모델(LLM)의 고도화된 통찰 리포트 생성.

---

## 📜 License

This project is licensed under the [MIT License](LICENSE).

---
> **"Debug your fate, Optimize your life."**
