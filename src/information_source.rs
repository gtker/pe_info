use std::fmt::Formatter;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum InformationSource {
    SelfReported,
    Heuristic,
}

impl std::fmt::Display for InformationSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            InformationSource::SelfReported => " Self Reported ",
            InformationSource::Heuristic => " Heuristic     ",
        })
    }
}
