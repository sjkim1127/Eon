use crate::connection::HumanDesignConnectionResult;
use crate::HumanDesignResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReturnType {
    Solar,
    Saturn,
    Chiron,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanDesignTransitResult {
    pub natal_chart: HumanDesignResult,
    pub transit_chart: HumanDesignResult,
    pub composite_connection: HumanDesignConnectionResult,
    pub target_date: String,
    pub is_return: Option<ReturnType>,
}
