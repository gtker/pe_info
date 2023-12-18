use crate::compiler_version::CompilerVersion;
use crate::info::Info;
use crate::optional_header::OptionalHeader;
use crate::{print_self_reported, Args};

pub fn versions_from_linker_version(
    optional_header: &OptionalHeader,
    info: &mut Info,
    args: &Args,
) {
    let minor = optional_header.minor_linker_version;
    let major = optional_header.major_linker_version;

    let one = |cv: CompilerVersion, info: &mut Info, major: u8, minor: u8| {
        let validity = if cv.release_date() > info.timestamp() {
            "which is not possible with the timestamp"
        } else {
            "which is possible with the timestamp"
        };

        print_self_reported(
            format!("Linker version is {major}.{minor}, which suggests linker is {cv}, {validity}"),
            args.quiet,
        );
        info.set_linker_version(cv);
    };

    match (major, minor) {
        (0, value) => print_self_reported(
            format!("Linker version is known invalid: 0.{value}"),
            args.quiet,
        ),
        (1, _) => one(CompilerVersion::VisualCPP1_0, info, major, minor),
        (2, _) => one(CompilerVersion::VisualCPP2_0, info, major, minor),
        (4, 0) => one(CompilerVersion::VisualCPP4_0, info, major, minor),
        (4, 1) => one(CompilerVersion::VisualCPP4_1, info, major, minor),
        (4, 2) => one(CompilerVersion::VisualCPP4_2, info, major, minor),
        (5, _) => one(CompilerVersion::VisualCPP5_0, info, major, minor),
        (6, _) => one(CompilerVersion::VisualCPP6_0, info, major, minor),
        (7, 0) => one(CompilerVersion::VisualStudioDotNet2002, info, major, minor),
        (7, 1) => one(CompilerVersion::VisualStudioDotNet2003, info, major, minor),
        (8, _) => one(CompilerVersion::VisualStudio2005, info, major, minor),
        (9, _) => one(CompilerVersion::VisualStudio2008, info, major, minor),
        (10, _) => one(CompilerVersion::VisualStudio2010, info, major, minor),
        (11, _) => one(CompilerVersion::VisualStudio2012, info, major, minor),
        (12, _) => one(CompilerVersion::VisualStudio2013, info, major, minor),
        (14, 0..=9) => one(CompilerVersion::VisualStudio2015, info, major, minor),
        (14, 10..=16) => one(CompilerVersion::VisualStudio2017, info, major, minor),
        (14, 20) => one(CompilerVersion::VisualStudio2019, info, major, minor),
        (14, 21..) => one(CompilerVersion::VisualStudio2022, info, major, minor),
        _ => {
            print_self_reported(
                format!("Linker version is unknown, possibly invalid: {major}.{minor}"),
                args.quiet,
            );
        }
    }
}
