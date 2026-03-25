import { useEffect, useRef } from "react";
import type { TabId } from "../types";

// ── 탭 청크 로더 ─────────────────────────────────────────
const loadSajuTab = () => import("../components/tabs/SajuTab");
const loadVedicChartsTab = () => import("../components/tabs/VedicChartsTab");
const loadStrengthTab = () => import("../components/tabs/StrengthTab");
const loadTransitTab = () => import("../components/tabs/TransitTab");
const loadSimulationTab = () => import("../components/tabs/SimulationTab");
const loadDestinyTierTab = () => import("../components/tabs/DestinyTierTab");


const TAB_LOADERS: Record<TabId, () => Promise<unknown>> = {
    saju: loadSajuTab,
    vedic_charts: loadVedicChartsTab,
    strength: loadStrengthTab,
    transit: loadTransitTab,
    simulation: loadSimulationTab,
    destiny_tier: loadDestinyTierTab,
};

const ALL_TABS: TabId[] = [
    "saju", "vedic_charts", "strength",
    "transit", "simulation", "destiny_tier",
];

// 마르코프 체인 폴백 (데이터 부족 시 도메인 지식 기반 우선순위)
const FALLBACK_NEXT_TABS: Record<TabId, TabId[]> = {
    saju: ["strength", "simulation", "transit"],
    vedic_charts: ["strength", "saju"],
    strength: ["transit", "saju"],
    transit: ["simulation", "saju"],
    simulation: ["transit"],
    destiny_tier: ["saju", "strength"],
};

// 리포트 존재 여부에 따른 탭 우선순위 가중치
const REPORT_READY_BONUS: Partial<Record<TabId, number>> = {
    saju: 4, strength: 4, vedic_charts: 2, transit: 2, destiny_tier: 5,
};
const TRANSIT_READY_BONUS: Partial<Record<TabId, number>> = {
    transit: 5, strength: 2,
};

export interface TabPrefetcherDeps {
    hasReport: boolean;
    hasTransit: boolean;
}

/**
 * 마르코프 체인 기반 탭 프리패칭 훅.
 * 탭 전환 패턴을 학습하여 다음 방문 가능성이 높은 탭의 JS 청크를 미리 로드합니다.
 *
 * @returns prefetchTab — 특정 탭 청크를 즉시 사전 로드하는 함수 (Sidebar hover 등에 사용)
 */
export function useTabPrefetcher(activeTab: TabId, deps: TabPrefetcherDeps) {
    const { hasReport, hasTransit } = deps;

    // 전이 빈도 행렬 (from → to 탭의 전환 횟수)
    const transitionRef = useRef<Record<TabId, Record<TabId, number>>>(
        ALL_TABS.reduce((acc, from) => {
            acc[from] = ALL_TABS.reduce((inner, to) => {
                inner[to] = 0;
                return inner;
            }, {} as Record<TabId, number>);
            return acc;
        }, {} as Record<TabId, Record<TabId, number>>)
    );
    const prevTabRef = useRef<TabId | null>(null);

    const prefetchTab = (tab: TabId) => {
        void TAB_LOADERS[tab]?.();
    };

    /** 현재 탭에서 다음 이동 가능성이 높은 탭을 최대 2개 선택 */
    const getDomainWeightedTabs = (currentTab: TabId): TabId[] => {
        const transitions = transitionRef.current[currentTab];

        const scores = ALL_TABS.reduce((acc, tab) => {
            if (tab === currentTab) return acc;

            const learnedScore = transitions[tab] ?? 0;
            const fallbackScore = FALLBACK_NEXT_TABS[currentTab].includes(tab) ? 2 : 0;
            const reportScore = hasReport ? (REPORT_READY_BONUS[tab] ?? 0) : 0;
            const transitScore = hasTransit ? (TRANSIT_READY_BONUS[tab] ?? 0) : 0;

            acc[tab] = learnedScore + fallbackScore + reportScore + transitScore;
            return acc;
        }, {} as Partial<Record<TabId, number>>);

        return ALL_TABS
            .filter((tab) => tab !== currentTab)
            .sort((a, b) => (scores[b] ?? 0) - (scores[a] ?? 0))
            .slice(0, 2);
    };

    // 초기 마운트: idle 시점에 가장 자주 쓰는 탭 3개를 사전 로드
    useEffect(() => {
        const preload = () => {
            void loadSajuTab();
            void loadStrengthTab();
            void loadVedicChartsTab();
        };

        if (typeof window !== "undefined" && "requestIdleCallback" in window) {
            const id = window.requestIdleCallback(preload, { timeout: 1200 });
            return () => window.cancelIdleCallback(id);
        }

        const id = setTimeout(preload, 600);
        return () => clearTimeout(id);
    }, []);

    // 탭 전환 감지 → 전이 행렬 업데이트 → 다음 탭 프리패칭
    useEffect(() => {
        const prev = prevTabRef.current;
        if (prev && prev !== activeTab) {
            transitionRef.current[prev][activeTab] += 1;
        }
        prevTabRef.current = activeTab;

        const candidates = getDomainWeightedTabs(activeTab);
        for (const nextTab of candidates) {
            prefetchTab(nextTab);
        }
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [activeTab, hasReport, hasTransit]);

    return { prefetchTab };
}
