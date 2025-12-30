use serde::{Deserialize, Serialize};
use super::ClassType;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, clap::ValueEnum)]
pub enum Hazard {
    AutonomousObject,
    BiologicalHazard,
    Cognitohazard,
    ElectricShock,
    ExistentialThreat,
    InconsistentTopology,
    IndirectInjuryHazard,
    MemeticHazard,
    NonstandardSpacetime,
    Shapeshifting,
    RadioactivityHazard,
    SelfReplicating,
    SentientViolent,
    SentientObject,
}

use std::fmt;

impl fmt::Display for Hazard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

impl Hazard {
    pub fn all() -> Vec<Self> {
        vec![
            Self::AutonomousObject,
            Self::BiologicalHazard,
            Self::Cognitohazard,
            Self::ElectricShock,
            Self::ExistentialThreat,
            Self::InconsistentTopology,
            Self::IndirectInjuryHazard,
            Self::MemeticHazard,
            Self::NonstandardSpacetime,
            Self::Shapeshifting,
            Self::RadioactivityHazard,
            Self::SelfReplicating,
            Self::SentientViolent,
            Self::SentientObject,
        ]
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::AutonomousObject => "Autonomous Object",
            Self::BiologicalHazard => "Biological Hazard",
            Self::Cognitohazard => "Cognitohazard",
            Self::ElectricShock => "Electric Shock",
            Self::ExistentialThreat => "Existential Threat",
            Self::InconsistentTopology => "Inconsistent Topology",
            Self::IndirectInjuryHazard => "Indirect Injury Hazard",
            Self::MemeticHazard => "Memetic Hazard",
            Self::NonstandardSpacetime => "Nonstandard Spacetime",
            Self::Shapeshifting => "Shapeshifting",
            Self::RadioactivityHazard => "Radioactivity Hazard",
            Self::SelfReplicating => "Self Replicating",
            Self::SentientViolent => "Sentient and Violent",
            Self::SentientObject => "Sentient Object",
        }
    }

    pub fn file_name(&self) -> &'static str {
        match self {
            Self::AutonomousObject => "autonomous_object",
            Self::BiologicalHazard => "biological_hazard",
            Self::Cognitohazard => "cognitohazard",
            Self::ElectricShock => "electric_shock",
            Self::ExistentialThreat => "existential_threat",
            Self::InconsistentTopology => "inconsistent_topology",
            Self::IndirectInjuryHazard => "indirect_injury_hazard",
            Self::MemeticHazard => "memetic_hazard",
            Self::NonstandardSpacetime => "nonstandard_spacetime",
            Self::Shapeshifting => "shapeshifting",
            Self::RadioactivityHazard => "radioactivity_hazard",
            Self::SelfReplicating => "self_replicating",
            Self::SentientViolent => "sentient_violent",
            Self::SentientObject => "sentient_object",
        }
    }

    pub fn icon_path(&self, class: &ClassType) -> String {
        format!(
            "resources/materials/{}/warnings/{}.png",
            class.folder_name(),
            self.file_name()
        )
    }
}
