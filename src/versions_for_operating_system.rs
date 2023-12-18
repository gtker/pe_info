use crate::info::Info;
use crate::optional_header::OptionalHeader;
use crate::{print_self_reported, Args};
use std::fmt::Formatter;

#[derive(Debug, Ord, Copy, Clone, PartialOrd, Eq, PartialEq, Hash)]
pub enum OperatingSystem {
    Windows95,
    Windows98,
    Windows2000,
    WindowsMe,
    WindowsXP,
    WindowsVista,
    Windows7,
    Windows8,
    Windows8_1,
    Windows10,
}

impl std::fmt::Display for OperatingSystem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            OperatingSystem::Windows95 => "Windows 95",
            OperatingSystem::Windows98 => "Windows 98",
            OperatingSystem::Windows2000 => "Windows 2000",
            OperatingSystem::WindowsMe => "Windows Me",
            OperatingSystem::WindowsXP => "Windows XP",
            OperatingSystem::WindowsVista => "Windows Vista",
            OperatingSystem::Windows7 => "Windows 7",
            OperatingSystem::Windows8 => "Windows 8",
            OperatingSystem::Windows8_1 => "Windows 8.1",
            OperatingSystem::Windows10 => "Windows 10",
        })
    }
}

fn versions_to_os(major: u16, minor: u16) -> Option<OperatingSystem> {
    Some(match (major, minor) {
        (4, 0) => OperatingSystem::Windows95,
        (4, 10) => OperatingSystem::Windows98,
        (4, 90) => OperatingSystem::WindowsMe,
        (5, 0) => OperatingSystem::Windows2000,
        (5, 1) => OperatingSystem::WindowsXP,
        (6, 0) => OperatingSystem::WindowsVista,
        (6, 1) => OperatingSystem::Windows7,
        (6, 2) => OperatingSystem::Windows8,
        (6, 3) => OperatingSystem::Windows8_1,
        (10, 0) => OperatingSystem::Windows10,
        _ => {
            return None;
        }
    })
}

pub fn versions_for_operating_system(pe: &OptionalHeader, info: &mut Info, args: &Args) {
    let major = pe.major_operating_system_version;
    let minor = pe.minor_operating_system_version;
    let os = versions_to_os(major, minor);
    match os {
        None => {
            print_self_reported(
                format!("Minimum invalid operating system version {major}.{minor}."),
                args.quiet,
            );
        }
        Some(os) => {
            info.set_os(os);
            print_self_reported(
                format!("Minimum operating system version is {major}.{minor} which suggests {os}"),
                args.quiet,
            );
        }
    }

    let major = pe.major_subsystem_version;
    let minor = pe.minor_subsystem_version;
    let os = versions_to_os(major, minor);
    match os {
        None => {
            print_self_reported(
                format!("Invalid minimum subsystem version {major}.{minor}."),
                args.quiet,
            );
        }
        Some(os) => {
            info.set_subsystem(os);
            if let Some(info_os) = info.os() {
                if os == info_os {
                    print_self_reported(format!("Minimum subsystem version {major}.{minor} agrees with operating system version of {os}"), args.quiet)
                } else {
                    info.set_likely_incorrect();
                    print_self_reported(format!("Minimum subsystem version {major}.{minor} ({os}) suggests different version than operating system version ({info_os})"), args.quiet)
                }
            } else {
                print_self_reported(
                    format!("Minimum subsystem version is {major}.{minor} which suggests {os}"),
                    args.quiet,
                );
            }
        }
    }
}
