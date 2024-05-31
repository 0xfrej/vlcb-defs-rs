use crate::{MergModuleType, RocRailModuleType, SpectrumModuleType, SprogModuleType, SysPixieModuleType};


#[derive(Debug)]
pub struct ModuleVersion {
    major: u8,
    minor: char,
    beta: u8,
}

impl ModuleVersion {
    #[inline]
    pub fn new(major: u8, minor: char, beta: u8) -> Self {
        debug_assert!(
            minor.is_ascii_alphabetic(),
            "The minor version must be a ASCII alphabetic character"
        );
        Self { major, minor, beta }
    }

    #[inline]
    pub fn major(&self) -> u8 {
        self.major
    }

    #[inline]
    pub fn minor(&self) -> char {
        self.minor
    }

    #[inline]
    pub fn beta(&self) -> u8 {
        self.beta
    }
}


pub enum ModuleType {
    Vlcb,
    Merg(MergModuleType),
    Sprog(SprogModuleType),
    RocRail(RocRailModuleType),
    Spectrum(SpectrumModuleType),
    SysPixie(SysPixieModuleType),
    Generic(u8),
}

impl From<ModuleType> for u8 {
    fn from(val: ModuleType) -> Self {
        match val {
            ModuleType::Vlcb => MergModuleType::VLCB.into(),
            ModuleType::Merg(v) => v.into(),
            ModuleType::Sprog(v) => v.into(),
            ModuleType::RocRail(v) => v.into(),
            ModuleType::Spectrum(v) => v.into(),
            ModuleType::SysPixie(v) => v.into(),
            ModuleType::Generic(v) => v,
        }
    }
}
