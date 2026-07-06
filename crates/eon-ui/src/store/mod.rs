pub mod db;
use crate::i18n::Locale;
use dioxus::prelude::*;
use eon_service::dto::{
    AnalysisInput, HumanDesignAnalysisOutput, IChingAnalysisOutput, QimenAnalysisOutput, SajuAnalysisOutput, TierResult,
    TransitAnalysisOutput, VedicAnalysisOutput, WesternAnalysisOutput, ZwdsAnalysisOutput,
};

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

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
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
    pub use_night_rat_hour: bool,
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
            use_night_rat_hour: false,
        }
    }
}

impl FormState {
    pub fn to_analysis_input(&self) -> AnalysisInput {
        AnalysisInput {
            year: self.year,
            month: self.month,
            day: self.day,
            hour: self.hour,
            minute: self.minute,
            is_lunar: self.is_lunar,
            is_leap_month: self.is_leap_month,
            lat: self.lat,
            lon: self.lon,
            timezone: "Asia/Seoul".to_string(),
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
    pub compat: Signal<AnalysisTaskState<eon_service::dto::VedicCompatibilityOutput>>,
    pub zwds: Signal<AnalysisTaskState<ZwdsAnalysisOutput>>,
    pub iching: Signal<AnalysisTaskState<IChingAnalysisOutput>>,
    pub western: Signal<AnalysisTaskState<WesternAnalysisOutput>>,
    pub human_design: Signal<AnalysisTaskState<HumanDesignAnalysisOutput>>,
    pub qimen: Signal<AnalysisTaskState<QimenAnalysisOutput>>,
    pub locale: Signal<Locale>,
}

impl AnalysisState {
    pub fn new() -> Self {
        Self {
            form: Signal::new(FormState::default()),
            saju: Signal::new(AnalysisTaskState::default()),
            vedic: Signal::new(AnalysisTaskState::default()),
            transit: Signal::new(AnalysisTaskState::default()),
            tier: Signal::new(AnalysisTaskState::default()),
            compat: Signal::new(AnalysisTaskState::default()),
            zwds: Signal::new(AnalysisTaskState::default()),
            iching: Signal::new(AnalysisTaskState::default()),
            western: Signal::new(AnalysisTaskState::default()),
            human_design: Signal::new(AnalysisTaskState::default()),
            qimen: Signal::new(AnalysisTaskState::default()),
            locale: Signal::new(Locale::Ko),
        }
    }
}
