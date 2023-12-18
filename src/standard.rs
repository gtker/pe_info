use std::fmt::Formatter;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum CppStandard {
    Cpp11,
    Cpp20,
}

impl std::fmt::Display for CppStandard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            CppStandard::Cpp11 => "C++11",
            CppStandard::Cpp20 => "C++20",
        })
    }
}

pub fn get_cpp_standard_for_function(name: &str) -> Option<CppStandard> {
    Some(match name {
        "atoll" => CppStandard::Cpp11,
        "strtoll" => CppStandard::Cpp11,
        "strtoull" => CppStandard::Cpp11,
        "isblank" => CppStandard::Cpp11,
        "strtoimax" => CppStandard::Cpp11,
        "strtoumax" => CppStandard::Cpp11,
        "mbrtoc16" => CppStandard::Cpp11,
        "c16rtomb" => CppStandard::Cpp11,
        "mbrtoc32" => CppStandard::Cpp11,
        "c32rtomb" => CppStandard::Cpp11,
        "iswblank" => CppStandard::Cpp11,
        "wcstoimax" => CppStandard::Cpp11,
        "wcstoumax" => CppStandard::Cpp11,

        "mbrtoc8" => CppStandard::Cpp20,
        "c8rtomb" => CppStandard::Cpp20,

        _ => {
            return None;
        }
    })
}
