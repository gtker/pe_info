use crate::compiler_version::CompilerVersion;
use crate::standard::CppStandard;
use crate::versions_for_operating_system::OperatingSystem;
use chrono::{DateTime, Utc};
use std::collections::BTreeSet;

#[derive(Debug, Ord, Clone, PartialOrd, Eq, PartialEq, Hash)]
pub struct Info {
    compiler_version: BTreeSet<CompilerVersion>,
    linker_version: Option<CompilerVersion>,
    standard: Option<CppStandard>,
    timestamp: DateTime<Utc>,
    is_cpp: bool,
    operating_system: Option<OperatingSystem>,
    subsystem: Option<OperatingSystem>,
    information_likely_incorrect: bool,
}

impl Info {
    pub fn new(timestamp: DateTime<Utc>) -> Self {
        Self {
            compiler_version: Default::default(),
            linker_version: None,
            standard: None,
            timestamp,
            is_cpp: false,
            operating_system: None,
            subsystem: None,
            information_likely_incorrect: false,
        }
    }

    pub fn compiler_version(&self) -> Vec<CompilerVersion> {
        self.compiler_version.iter().cloned().collect()
    }

    pub fn linker_version(&self) -> Option<CompilerVersion> {
        self.linker_version
    }

    pub fn set_likely_incorrect(&mut self) {
        self.information_likely_incorrect = true;
    }

    pub fn likely_correct(&self) -> bool {
        !self.information_likely_incorrect
    }

    pub fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }

    pub fn is_cpp(&self) -> bool {
        self.is_cpp
    }

    pub fn set_cpp(&mut self, is_cpp: bool) {
        if is_cpp {
            self.is_cpp = true;
        }
    }

    pub fn os(&self) -> Option<OperatingSystem> {
        self.operating_system
    }

    pub fn set_os(&mut self, os: OperatingSystem) {
        self.operating_system = Some(os);
    }

    pub fn set_subsystem(&mut self, os: OperatingSystem) {
        self.subsystem = Some(os);
    }

    pub fn add_standard(&mut self, standard: CppStandard) {
        match self.standard {
            None => self.standard = Some(standard),
            Some(s) => {
                if standard > s {
                    self.standard = Some(standard);
                }
            }
        }
    }

    pub fn standard(&self) -> Option<CppStandard> {
        self.standard
    }

    pub fn set_linker_version(&mut self, version: CompilerVersion) {
        self.linker_version = Some(version);
    }

    pub fn exclude_all_compiler_versions_except(&mut self, version: CompilerVersion) {
        self.compiler_version.clear();
        self.insert_single_compiler_version(version);
    }

    pub fn insert_single_compiler_version(&mut self, version: CompilerVersion) {
        self.compiler_version.insert(version);
    }

    pub fn insert_multiple_compiler_versions(&mut self, versions: &[CompilerVersion]) {
        for v in versions {
            self.insert_single_compiler_version(*v);
        }
    }

    pub fn minimum_operating_system(&self) -> Option<OperatingSystem> {
        let Some(os) = self.os() else {
            return None;
        };
        let Some(subsystem) = self.subsystem else {
            return None;
        };

        if os == subsystem {
            return Some(os);
        }

        None
    }
}
