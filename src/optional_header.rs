use pe_parser::optional::{OptionalHeader32, OptionalHeader64};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct OptionalHeader {
    pub major_linker_version: u8,
    pub minor_linker_version: u8,
    pub major_operating_system_version: u16,
    pub minor_operating_system_version: u16,
    pub major_subsystem_version: u16,
    pub minor_subsystem_version: u16,
}

impl OptionalHeader {
    pub fn from_header32(header: &OptionalHeader32) -> Self {
        Self {
            major_linker_version: header.major_linker_version,
            minor_linker_version: header.minor_linker_version,
            major_operating_system_version: header.major_operating_system_version,
            minor_operating_system_version: header.minor_operating_system_version,
            major_subsystem_version: header.major_subsystem_version,
            minor_subsystem_version: header.minor_subsystem_version,
        }
    }

    pub fn from_header64(header: &OptionalHeader64) -> Self {
        Self {
            major_linker_version: header.major_linker_version,
            minor_linker_version: header.minor_linker_version,
            major_operating_system_version: header.major_operating_system_version,
            minor_operating_system_version: header.minor_operating_system_version,
            major_subsystem_version: header.major_subsystem_version,
            minor_subsystem_version: header.minor_subsystem_version,
        }
    }
}
