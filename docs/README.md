# Eon 사주 분석 엔진 문서 (Eon Saju Engine Docs)

Eon 엔진은 전통 명리학의 정밀한 계산과 현대적인 데이터 분석 기법을 결합한 고성능 사주 분석 엔진입니다. 본 문서는 엔진에 구현된 주요 로직과 모듈별 상세 명세를 제공합니다.

## 모듈 개요 (Module Overview)

엔진은 크게 **계산(Calculation)**, **분석(Analysis)**, **해석(Interpretation)**의 3단계 레이어로 구성되어 있습니다.

### 1. 계산 레이어 (`eon-core`, `eon-astro`, `eon-saju/pillars`)

- **천문 보정**: Swiss Ephemeris(NASA JPL 데이터)를 통한 태양 황경 정밀 산출 및 지역시 보정.
- **간지 산출**: 태양 황경 15도 간격의 24절기 정밀 판별 및 간지 결정.
- **초정밀 대운**: 분(Minute) 단위 절입 시각 추적을 통한 교운기(운이 바뀌는 시점) 산출.
- **자시 처리**: 조자시(23:00~01:00) 시스템 적용.

### 2. 분석 레이어 (`eon-saju/power`, `eon-saju/transformations`)

- **정밀 점수법**: 110점 가중치 시스템을 이용한 오행/십성 수치화.
- **합화(合化)**: 천간/지지 합에 따른 실질 오행 변화 추적.
- **보정 옵션**: 궁성 가중치 및 조후 보정(계절별 토의 성질 변화) 적용.

### 3. 해석 레이어 (`eon-saju/structure`, `eon-saju/yongshin`, `eon-saju/spirit_markers`)

- **격국(格局)**: 월령 지장간 투출 기준의 정격 및 특수격 판정.
- **용신(用神)**: 억부와 조후를 결합한 최적의 용신/희신 추천.
- **신살/관계**: 20여 종의 신살 및 합충형해(合沖刑害) 정밀 분석.

## 상세 문서 목록

1. [계산 로직 상세 (CALCULATION.md)](./CALCULATION.md) - 시간 보정과 간지 산출 원리
2. [분석 및 보정 상세 (ANALYSIS.md)](./ANALYSIS.md) - 110점법과 4가지 보정 모드
3. [격국 및 용신 상세 (INTERPRETATION.md)](./INTERPRETATION.md) - 사주 해석 및 용신 결정 알고리즘
