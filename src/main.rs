mod compiler_version;
mod conclusion;
mod dll_imports;
mod info;
mod information_source;
mod linker_version;
mod optional_header;
mod standard;
mod versions_for_operating_system;

use crate::compiler_version::CompilerVersion;
use crate::conclusion::print_conclusion;
use crate::dll_imports::versions_from_imported_dlls;
use crate::info::Info;
use crate::information_source::InformationSource;
use crate::linker_version::versions_from_linker_version;
use crate::optional_header::OptionalHeader;
use crate::versions_for_operating_system::versions_for_operating_system;
use chrono::DateTime;
use clap::Parser;
use exe::VecPE;
use std::process::ExitCode;

fn print_self_reported(s: impl AsRef<str>, quiet: bool) {
    print_info(InformationSource::SelfReported, s, quiet)
}

fn print_heuristic(s: impl AsRef<str>, quiet: bool) {
    print_info(InformationSource::Heuristic, s, quiet)
}

fn print_info(source: InformationSource, s: impl AsRef<str>, quiet: bool) {
    if !quiet {
        let s = s.as_ref();
        println!("[{source}] {s}")
    }
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Do not print out intermediate results
    #[arg(short, long, default_value_t = false)]
    quiet: bool,
    /// Return error code for likely invalid information
    #[arg(short, long, default_value_t = false)]
    strict: bool,
    /// Files to analyze.
    files: Vec<String>,
}

fn main() -> ExitCode {
    let args = Args::parse();

    let mut success = true;

    let length = args.files.len();
    for (i, name) in args.files.iter().enumerate() {
        if !args.quiet {
            println!("Reading '{name}'");
        }

        let file = std::fs::read(name).unwrap();
        let pe = VecPE::from_disk_file(name).unwrap();
        let parser = pe_parser::pe::parse_portable_executable(&file).unwrap();

        let timestamp_inconsistent = parser.coff.time_date_stamp == 0;
        let date = DateTime::from_timestamp(parser.coff.time_date_stamp.into(), 0).unwrap();
        let newest_possible = CompilerVersion::newest_possible(date);
        let newest_possible_date = newest_possible
            .release_date()
            .signed_duration_since(date)
            .num_days()
            .abs();
        print_self_reported(
            format!(
                "Timestamp is {date}, which means the newest possible version is {newest_possible} ({newest_possible_date} days after release)"
            ),
            args.quiet,
        );
        let mut info = Info::new(date);

        if timestamp_inconsistent {
            info.set_likely_incorrect();
        }

        versions_from_imported_dlls(pe.as_ptr_pe(), &mut info, &args).unwrap();

        let values = if let Some(header) = &parser.optional_header_32 {
            Some(OptionalHeader::from_header32(header))
        } else if let Some(header) = &parser.optional_header_64 {
            Some(OptionalHeader::from_header64(header))
        } else {
            None
        };

        if let Some(header) = values {
            versions_from_linker_version(&header, &mut info, &args);

            versions_for_operating_system(&header, &mut info, &args);
        }

        success = print_conclusion(info, &args);

        if i + 1 != length {
            println!();
        }
    }

    if success {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
