use cosmic::{
    cosmic_config::{self, cosmic_config_derive::CosmicConfigEntry, CosmicConfigEntry},
    cosmic_theme::palette::Srgba,
};
use serde::{Deserialize, Serialize};

use crate::{
    fl,
    sensors::{disks::DisksVariant},
};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq)]
pub enum ColorVariant {
    Color1,
    Color2,
    Color3,
    Color4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GraphKind {
    Ring,
    Line,
}

impl From<usize> for GraphKind {
    fn from(index: usize) -> Self {
        match index {
            0 => GraphKind::Ring,
            1 => GraphKind::Line,
            _ => panic!("Invalid index for SvgKind"),
        }
    }
}

impl From<GraphKind> for usize {
    fn from(kind: GraphKind) -> Self {
        match kind {
            GraphKind::Ring => 0,
            GraphKind::Line => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceKind {
    Cpu(GraphKind),
    Memory(GraphKind),
    Network(NetworkVariant),
    Disks(DisksVariant),
}

impl std::fmt::Display for DeviceKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DeviceKind::Cpu(_) => write!(f, "{}", fl!("sensor-cpu")),
            DeviceKind::Memory(_) => write!(f, "{}", fl!("sensor-memory")),
            DeviceKind::Network(_) => write!(f, "{}", fl!("sensor-network")),
            DeviceKind::Disks(_) => write!(f, "{}", fl!("sensor-disks")),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, CosmicConfigEntry, PartialEq, Eq)]
#[version = 1]
pub struct GraphColors {
    pub color1: Srgba<u8>,
    pub color2: Srgba<u8>,
    pub color3: Srgba<u8>,
    pub color4: Srgba<u8>,
}

impl Default for GraphColors {
    fn default() -> Self {
        Self {
            color1: Srgba::from_components((0x2b, 0x2b, 0x2b, 0xff)),
            color2: Srgba::from_components((255, 255, 255, 255)),
            color3: Srgba::from_components((85, 85, 85, 255)),
            color4: Srgba::from_components((255, 6, 0, 255)),
        }
    }
}

impl GraphColors {
    pub fn new(kind: DeviceKind) -> Self {
        match kind {
            DeviceKind::Cpu(_) => GraphColors::default(),
            DeviceKind::Memory(_) => GraphColors {
                color4: Srgba::from_components((187, 41, 187, 255)),
                ..Default::default()
            },
            DeviceKind::Network(k) => match k {
                NetworkVariant::Combined => GraphColors {
                    color1: Srgba::from_components((0x2b, 0x2b, 0x2b, 255)),
                    color2: Srgba::from_components((47, 141, 255, 255)),
                    color3: Srgba::from_components((255, 0, 0, 255)),
                    color4: Srgba::from_components((0x2b, 0x2b, 0x2b, 255)),
                },
                NetworkVariant::Download => GraphColors {
                    color1: Srgba::from_components((0x2b, 0x2b, 0x2b, 255)),
                    color2: Srgba::from_components((47, 141, 255, 255)),
                    color3: Srgba::from_components((255, 0, 0, 0)),
                    color4: Srgba::from_components((0x2b, 0x2b, 0x2b, 255)),
                },
                NetworkVariant::Upload => GraphColors {
                    color1: Srgba::from_components((0x2b, 0x2b, 0x2b, 255)),
                    color2: Srgba::from_components((47, 141, 255, 0)),
                    color3: Srgba::from_components((255, 0, 0, 255)),
                    color4: Srgba::from_components((0x2b, 0x2b, 0x2b, 255)),
                },
            },
            DeviceKind::Disks(k) => match k {
                DisksVariant::Combined => GraphColors {
                    color1: Srgba::from_components((0x2b, 0x2b, 0x2b, 255)),
                    color2: Srgba::from_components((47, 141, 255, 255)),
                    color3: Srgba::from_components((255, 0, 0, 255)),
                    color4: Srgba::from_components((0x2b, 0x2b, 0x2b, 255)),
                },
                DisksVariant::Write => GraphColors {
                    color1: Srgba::from_components((0x2b, 0x2b, 0x2b, 255)),
                    color2: Srgba::from_components((47, 141, 255, 255)),
                    color3: Srgba::from_components((255, 0, 0, 0)),
                    color4: Srgba::from_components((0x2b, 0x2b, 0x2b, 255)),
                },
                DisksVariant::Read => GraphColors {
                    color1: Srgba::from_components((0x2b, 0x2b, 0x2b, 255)),
                    color2: Srgba::from_components((47, 141, 255, 0)),
                    color3: Srgba::from_components((255, 0, 0, 255)),
                    color4: Srgba::from_components((0x2b, 0x2b, 0x2b, 255)),
                },
            },
        }
    }

    pub fn set_color(&mut self, srgb: Srgba<u8>, variant: ColorVariant) {
        match variant {
            ColorVariant::Color1 => self.color1 = srgb,
            ColorVariant::Color2 => self.color2 = srgb,
            ColorVariant::Color3 => self.color3 = srgb,
            ColorVariant::Color4 => self.color4 = srgb,
        }
    }

    pub fn get_color(self, variant: ColorVariant) -> Srgba<u8> {
        match variant {
            ColorVariant::Color1 => self.color1,
            ColorVariant::Color2 => self.color2,
            ColorVariant::Color3 => self.color3,
            ColorVariant::Color4 => self.color4,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, CosmicConfigEntry, PartialEq, Eq)]
#[version = 1]
pub struct CpuConfig {
    pub chart: bool,
    pub label: bool,
    pub kind: GraphKind,
    pub colors: GraphColors,
}

impl Default for CpuConfig {
    fn default() -> Self {
        Self {
            chart: true,
            label: false,
            kind: GraphKind::Ring,
            colors: GraphColors::new(DeviceKind::Cpu(GraphKind::Ring)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, CosmicConfigEntry, PartialEq)]
#[version = 1]
pub struct MemoryConfig {
    pub chart: bool,
    pub label: bool,
    pub kind: GraphKind,
    pub colors: GraphColors,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            chart: true,
            label: false,
            kind: GraphKind::Ring,
            colors: GraphColors::new(DeviceKind::Memory(GraphKind::Line)),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum NetworkVariant {
    Download,
    Upload,
    Combined,
}

#[derive(Debug, Clone, Serialize, Deserialize, CosmicConfigEntry, PartialEq)]
#[version = 1]
pub struct NetworkConfig {
    pub chart: bool,
    pub label: bool,
    pub adaptive: bool,
    pub bandwidth: u64,
    pub unit: Option<usize>,
    pub colors: GraphColors,
    pub variant: NetworkVariant,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            chart: true,
            label: false,
            adaptive: true,
            bandwidth: 62_500_000, // 500Mbit/s
            unit: Some(0),
            colors: GraphColors::new(DeviceKind::Network(NetworkVariant::Upload)),
            variant: NetworkVariant::Download,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, CosmicConfigEntry, PartialEq)]
#[version = 1]
pub struct DisksConfig {
    pub chart: bool,
    pub label: bool,
    pub colors_combined: GraphColors,
    pub colors_write: GraphColors,
    pub colors_read: GraphColors,
}

impl Default for DisksConfig {
    fn default() -> Self {
        Self {
            chart: false,
            label: false,
            colors_combined: GraphColors::new(DeviceKind::Disks(DisksVariant::Combined)),
            colors_write: GraphColors::new(DeviceKind::Disks(DisksVariant::Write)),
            colors_read: GraphColors::new(DeviceKind::Disks(DisksVariant::Read)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, CosmicConfigEntry, PartialEq)]
#[version = 1]
pub struct MinimonConfig {
    pub refresh_rate: u64,
    pub label_size_default: u16,
    pub monospace_labels: bool,

    pub cpu: CpuConfig,
    pub memory: MemoryConfig,

    pub network1: NetworkConfig,
    pub network2: NetworkConfig,

    pub disks: DisksConfig,
}

impl Default for MinimonConfig {
    fn default() -> Self {
        Self {
            refresh_rate: 1000,
            label_size_default: 11,
            monospace_labels: false,
            cpu: CpuConfig::default(),
            memory: MemoryConfig::default(),
            network1: NetworkConfig::default(),
            network2: NetworkConfig::default(),
            disks: DisksConfig::default(),
        }
    }
}
