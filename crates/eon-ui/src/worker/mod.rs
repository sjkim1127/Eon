#![allow(dead_code)]

use eon_service::dto::{
    SajuAnalysisInput, SajuAnalysisOutput, VedicAnalysisInput, VedicAnalysisOutput,
};
use eon_service::facade;
use gloo_worker::{HandlerId, Worker, WorkerScope};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum WorkerInput {
    RunSaju(SajuAnalysisInput),
    RunVedic(VedicAnalysisInput),
}

#[derive(Serialize, Deserialize)]
pub enum WorkerOutput {
    SajuResult(Result<SajuAnalysisOutput, String>),
    VedicResult(Result<VedicAnalysisOutput, String>),
}

pub struct AnalysisWorker {}

impl Worker for AnalysisWorker {
    type Message = ();
    type Input = WorkerInput;
    type Output = WorkerOutput;

    fn create(_scope: &WorkerScope<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _scope: &WorkerScope<Self>, _msg: Self::Message) {}

    fn received(&mut self, scope: &WorkerScope<Self>, msg: Self::Input, id: HandlerId) {
        match msg {
            WorkerInput::RunSaju(input) => {
                let res = facade::analyze_saju(input).map_err(|e| format!("{:?}", e));
                scope.respond(id, WorkerOutput::SajuResult(res));
            }
            WorkerInput::RunVedic(input) => {
                let res = facade::analyze_vedic(input).map_err(|e| format!("{:?}", e));
                scope.respond(id, WorkerOutput::VedicResult(res));
            }
        }
    }
}
