use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AyanamsaSystem {
    Lahiri = 1,      // SWE_SIDM_LAHIRI
    FaganBradley = 0, // SWE_SIDM_FAGAN_BRADLEY
    Raman = 3,       // SWE_SIDM_RAMAN
    Krishnamurti = 5, // SWE_SIDM_KRISHNAMURTI
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeCalculation {
    MeanNode, // SE_MEAN_NODE (10)
    TrueNode, // SE_TRUE_NODE (11)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HouseSystem {
    WholeSign,
    Sripati,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct VedicConfig {
    pub ayanamsa: AyanamsaSystem,
    pub node_calc: NodeCalculation,
    pub house_system: HouseSystem,
    // Add Varga related config if needed, e.g. which vargas to calculate
    // For now, we will calculate all requested ones or default set.
}

impl Default for VedicConfig {
    fn default() -> Self {
        Self {
            ayanamsa: AyanamsaSystem::Lahiri,
            node_calc: NodeCalculation::MeanNode,
            house_system: HouseSystem::WholeSign,
        }
    }
}
