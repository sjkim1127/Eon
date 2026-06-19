use dioxus::prelude::*;
use eon_service::dto::{SajuAnalysisOutput, VedicAnalysisOutput, TransitAnalysisOutput, TierResult};

#[derive(Clone, PartialEq, Default)]
pub enum TaskStatus {
    #[default]
    Idle,
    Loading,
    Success,
    Error(String),
}

#[derive(Clone)]
pub struct AnalysisTaskState<T: Clone + 'static> {
    pub status: TaskStatus,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T: Clone + 'static> Default for AnalysisTaskState<T> {
    fn default() -> Self {
        Self {
            status: TaskStatus::Idle,
            data: None,
            error: None,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct FormState {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub is_lunar: bool,
    pub is_leap_month: bool,
    pub lat: f64,
    pub lon: f64,
    pub is_male: bool,
}

impl Default for FormState {
    fn default() -> Self {
        Self {
            year: 1990,
            month: 5,
            day: 15,
            hour: 10,
            minute: 0,
            is_lunar: false,
            is_leap_month: false,
            lat: 37.5665,
            lon: 126.9780,
            is_male: true,
        }
    }
}

#[derive(Clone, Default)]
pub struct AnalysisState {
    pub form: Signal<FormState>,
    pub saju: Signal<AnalysisTaskState<SajuAnalysisOutput>>,
    pub vedic: Signal<AnalysisTaskState<VedicAnalysisOutput>>,
    pub transit: Signal<AnalysisTaskState<TransitAnalysisOutput>>,
    pub tier: Signal<AnalysisTaskState<TierResult>>,
}

impl AnalysisState {
    pub fn new() -> Self {
        Self {
            form: Signal::new(FormState::default()),
            saju: Signal::new(AnalysisTaskState::default()),
            vedic: Signal::new(AnalysisTaskState::default()),
            transit: Signal::new(AnalysisTaskState::default()),
            tier: Signal::new(AnalysisTaskState::default()),
        }
    }
}
