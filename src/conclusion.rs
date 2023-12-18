use crate::info::Info;
use crate::Args;

pub fn print_conclusion(info: Info, args: &Args) -> bool {
    if !info.likely_correct() {
        println!(
            "THIS IS LIKELY INCORRECT EITHER BECAUSE IT WAS PARSED INCORRECTLY OR DELIBERATELY MODIFIED"
        );
    }

    println!("Compiled: {}", info.timestamp());

    match info.compiler_version().as_slice() {
        [] => {
            println!("Compiler: ?");
        }
        [single] => {
            println!("Compiler: {single}");
        }
        all => {
            println!("Compiler (one of):");
            for cv in all {
                println!("         {cv}")
            }
        }
    }

    if let Some(cv) = info.linker_version() {
        println!("Linker: {cv}");
    }

    if let Some(standard) = info.standard() {
        println!("C++ Standard used: At least {standard}")
    }

    if let Some(operating_system) = info.minimum_operating_system() {
        println!("Minimum Operating System: {operating_system}");
    }

    !args.strict && info.likely_correct()
}
