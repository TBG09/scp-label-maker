use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, clap::ValueEnum)]
pub enum ClassType {
    Safe,
    Euclid,
    EuclidPotentialKeter,
    Keter,
    Apollyon,
    Thaumiel,
    Neutralized,
    Explained,
}

impl ClassType {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Safe,
            Self::Euclid,
            Self::EuclidPotentialKeter,
            Self::Keter,
            Self::Apollyon,
            Self::Thaumiel,
            Self::Neutralized,
            Self::Explained,
        ]
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Safe => "SAFE",
            Self::Euclid => "EUCLID",
            Self::EuclidPotentialKeter => "EUCLID / POTENTIAL KETER",
            Self::Keter => "KETER",
            Self::Apollyon => "APOLLYON",
            Self::Thaumiel => "THAUMIEL",
            Self::Neutralized => "NEUTRALIZED",
            Self::Explained => "EXPLAINED",
        }
    }

    pub fn folder_name(&self) -> String {
        match self {
            Self::EuclidPotentialKeter => "euclid_potential_keter".to_string(),
            _ => self.as_str().to_lowercase(),
        }
    }

    pub fn label_path(&self, alternate: bool) -> String {
        let folder = self.folder_name();
        let variant = if alternate { "_2" } else { "" };
        format!(
            "resources/materials/{}/label{}.jpg",
            folder,
            variant
        )
    }

    pub fn ui_color(&self) -> [f32; 3] {
        match self {
            Self::Safe => [0.0, 0.8, 0.0],                
            Self::Euclid => [1.0, 0.6, 0.0],              
            Self::EuclidPotentialKeter => [0.8, 0.2, 0.1],
            Self::Keter => [0.9, 0.0, 0.0],               
            Self::Apollyon => [0.5, 0.0, 0.5],            
            Self::Thaumiel => [0.0, 0.0, 0.0],            
            Self::Neutralized => [0.5, 0.5, 0.5],         
            Self::Explained => [0.0, 0.5, 0.8],           
        }
    }
}

impl std::fmt::Display for ClassType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for ClassType {
    fn default() -> Self {
        Self::Safe
    }
}