use crate::compiler_version::CompilerVersion;
use crate::info::Info;
use crate::standard::get_cpp_standard_for_function;
use crate::{print_heuristic, Args};
use exe::{CCharString, ImageImportDescriptor, ImportData, ImportDirectory, PtrPE};

pub fn versions_from_imported_dlls(pe: PtrPE, info: &mut Info, args: &Args) -> Result<(), String> {
    let Ok(import_directory) = ImportDirectory::parse(&pe) else {
        info.set_likely_incorrect();
        return Err("Unable to parse import directory".to_string());
    };

    specific_imports(&import_directory, &pe, info, args);
    specific_symbols(&import_directory, &pe, info, args);

    Ok(())
}

fn get_name_and_imports<'a>(
    descriptor: &'a ImageImportDescriptor,
    pe: &'a PtrPE,
) -> Option<(&'a str, Vec<ImportData<'a>>)> {
    let lib = if let Ok(lib) = descriptor.get_name(pe) {
        if let Ok(lib) = lib.as_str() {
            lib
        } else {
            ""
        }
    } else {
        ""
    };

    let Ok(imports) = descriptor.get_imports(pe) else {
        return None;
    };

    Some((lib, imports))
}

fn specific_symbols(import_directory: &ImportDirectory, pe: &PtrPE, info: &mut Info, args: &Args) {
    for descriptor in import_directory.descriptors {
        let Some((lib, imports)) = get_name_and_imports(descriptor, pe) else {
            continue;
        };

        let mut cpp_symbol = imports.iter().filter_map(|a| match a {
            ImportData::ImportByName(name) => {
                if name.starts_with("?") {
                    Some(name)
                } else {
                    None
                }
            }
            _ => None,
        });

        while let Some(symbol) = cpp_symbol.next() {
            info.set_cpp(true);
            print_heuristic(
                format!("Imported symbol '{symbol}' from '{lib}' suggests C++ was used."),
                args.quiet,
            );

            break;
        }
    }

    for descriptor in import_directory.descriptors {
        let Some((lib, imports)) = get_name_and_imports(descriptor, pe) else {
            continue;
        };

        for import in imports {
            match import {
                ImportData::Ordinal(_) => {}
                ImportData::ImportByName(name) => {
                    if let Some(standard) = get_cpp_standard_for_function(name) {
                        if info.is_cpp() {
                            print_heuristic( format!("Import of symbol '{name}' from '{lib}' suggests at least standard {standard}"), args.quiet);
                            info.add_standard(standard);
                        }
                    }
                }
            }
        }
    }
}

fn specific_imports(import_directory: &ImportDirectory, pe: &PtrPE, info: &mut Info, args: &Args) {
    for descriptor in import_directory.descriptors {
        let Ok(module_name) = descriptor.get_name(pe) else {
            continue;
        };

        let Ok(module_name) = module_name.as_str() else {
            continue;
        };

        let one = |cv: CompilerVersion, info: &mut Info| {
            let time_validity = if info.timestamp() > cv.release_date() {
                "is possible"
            } else {
                info.set_likely_incorrect();
                "isn't possible"
            };

            print_heuristic(format!(
                "Links against {module_name} which suggests compiler is {}, which {time_validity} with the release date of the compiler",
                cv,
            ), args.quiet);
            info.exclude_all_compiler_versions_except(cv)
        };

        let multi = |cvs: &[CompilerVersion], info: &mut Info| {
            let mut s = format!("Links against {module_name} which suggests compiler is either ",);

            for (i, cv) in cvs.iter().enumerate() {
                if i != 0 {
                    s += ", "
                }

                let time_validity = if info.timestamp() > cv.release_date() {
                    "valid for timestamp"
                } else {
                    info.set_likely_incorrect();
                    "not valid for timestamp"
                };
                s += &format!("{cv} ({time_validity})");
            }

            print_heuristic(s, args.quiet);
            info.insert_multiple_compiler_versions(cvs)
        };

        match module_name.to_lowercase().as_str() {
            "msvcrt.dll" => multi(
                &[
                    CompilerVersion::VisualCPP4_2,
                    CompilerVersion::VisualCPP5_0,
                    CompilerVersion::VisualCPP6_0,
                ],
                info,
            ),
            "msvcp50.dll" => {
                one(CompilerVersion::VisualCPP5_0, info);
            }
            "msvcp60.dll" => one(CompilerVersion::VisualCPP6_0, info),
            "msvcr70.dll" | "msvcp70.dll" => one(CompilerVersion::VisualStudioDotNet2002, info),
            "msvcr71.dll" | "msvcp71.dll" => one(CompilerVersion::VisualStudioDotNet2003, info),
            "msvcr80.dll" | "msvcp80.dll" => one(CompilerVersion::VisualStudio2005, info),
            "msvcr90.dll" | "msvcp90.dll" => one(CompilerVersion::VisualStudio2008, info),
            "msvcr100.dll" | "msvcp100.dll" => one(CompilerVersion::VisualStudio2010, info),
            "msvcr110.dll" | "msvcp110.dll" => one(CompilerVersion::VisualStudio2012, info),
            "msvcr120.dll" | "msvcp120.dll" => one(CompilerVersion::VisualStudio2013, info),
            "ucrtbase.dll" | "vcruntime140.dll" | "msvcp140.dll" => multi(
                &[
                    CompilerVersion::VisualStudio2015,
                    CompilerVersion::VisualStudio2017,
                    CompilerVersion::VisualStudio2019,
                    CompilerVersion::VisualStudio2022,
                ],
                info,
            ),
            module => {
                if module.contains("api-ms-win-crt") {
                    multi(
                        &[
                            CompilerVersion::VisualStudio2015,
                            CompilerVersion::VisualStudio2017,
                            CompilerVersion::VisualStudio2019,
                            CompilerVersion::VisualStudio2022,
                        ],
                        info,
                    );
                }
            }
        }
    }
}
