use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use std::fmt::Formatter;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum CompilerVersion {
    VisualCPP1_0,
    VisualCPP1_5,
    VisualCPP2_0,
    VisualCPP4_0,
    VisualCPP4_1,
    VisualCPP4_2,
    VisualCPP5_0,
    VisualCPP6_0,
    VisualStudioDotNet2002,
    VisualStudioDotNet2003,
    VisualStudio2005,
    VisualStudio2008,
    VisualStudio2010,
    VisualStudio2012,
    VisualStudio2013,
    VisualStudio2015,
    VisualStudio2017,
    VisualStudio2019,
    VisualStudio2022,
}

impl std::fmt::Display for CompilerVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CompilerVersion::VisualCPP1_0 => f.write_str("Visual C++ 1.0"),
            CompilerVersion::VisualCPP1_5 => f.write_str("Visual C++ 1.5"),
            CompilerVersion::VisualCPP2_0 => f.write_str("Visual C++ 2.0"),
            CompilerVersion::VisualCPP4_0 => f.write_str("Visual C++ 4.0"),
            CompilerVersion::VisualCPP4_1 => f.write_str("Visual C++ 4.1"),
            CompilerVersion::VisualCPP4_2 => f.write_str("Visual C++ 4.2"),
            CompilerVersion::VisualCPP5_0 => f.write_str("Visual C++ 5.0 (Visual Studio 97)"),
            CompilerVersion::VisualCPP6_0 => f.write_str("Visual C++ 6.0 (Visual Studio 6.0)"),
            CompilerVersion::VisualStudioDotNet2002 => f.write_str("Visual Studio .NET 2002"),
            CompilerVersion::VisualStudioDotNet2003 => f.write_str("Visual Studio .NET 2003"),
            CompilerVersion::VisualStudio2005 => f.write_str("Visual Studio 2005"),
            CompilerVersion::VisualStudio2008 => f.write_str("Visual Studio 2008"),
            CompilerVersion::VisualStudio2010 => f.write_str("Visual Studio 2010"),
            CompilerVersion::VisualStudio2012 => f.write_str("Visual Studio 2012"),
            CompilerVersion::VisualStudio2013 => f.write_str("Visual Studio 2013"),
            CompilerVersion::VisualStudio2015 => f.write_str("Visual Studio 2015"),
            CompilerVersion::VisualStudio2017 => f.write_str("Visual Studio 2017"),
            CompilerVersion::VisualStudio2019 => f.write_str("Visual Studio 2019"),
            CompilerVersion::VisualStudio2022 => f.write_str("Visual Studio 2022"),
        }
    }
}

fn date_to_datetime(year: i32, month: u32, day: u32) -> DateTime<Utc> {
    DateTime::from_naive_utc_and_offset(
        NaiveDateTime::new(
            match NaiveDate::from_ymd_opt(year, month, day) {
                None => panic!("invalid date"),
                Some(e) => e,
            },
            NaiveTime::MIN,
        ),
        Utc,
    )
}

impl CompilerVersion {
    pub fn release_date(&self) -> DateTime<Utc> {
        match self {
            CompilerVersion::VisualCPP1_0 => date_to_datetime(1993, 2, 22),
            CompilerVersion::VisualCPP1_5 => date_to_datetime(1993, 12, 1),
            CompilerVersion::VisualCPP2_0 => date_to_datetime(1994, 9, 16),
            CompilerVersion::VisualCPP4_0 => date_to_datetime(1995, 12, 11),
            CompilerVersion::VisualCPP4_1 => date_to_datetime(1996, 2, 27),
            CompilerVersion::VisualCPP4_2 => date_to_datetime(1996, 7, 22),
            CompilerVersion::VisualCPP5_0 => date_to_datetime(1997, 3, 19),
            CompilerVersion::VisualCPP6_0 => date_to_datetime(1998, 9, 2),
            CompilerVersion::VisualStudioDotNet2002 => date_to_datetime(2002, 2, 13),
            CompilerVersion::VisualStudioDotNet2003 => date_to_datetime(2003, 4, 24),
            CompilerVersion::VisualStudio2005 => date_to_datetime(2005, 11, 7),
            CompilerVersion::VisualStudio2008 => date_to_datetime(2007, 11, 19),
            CompilerVersion::VisualStudio2010 => date_to_datetime(2010, 4, 12),
            CompilerVersion::VisualStudio2012 => date_to_datetime(2012, 9, 12),
            CompilerVersion::VisualStudio2013 => date_to_datetime(2013, 10, 17),
            CompilerVersion::VisualStudio2015 => date_to_datetime(2015, 6, 20),
            CompilerVersion::VisualStudio2017 => date_to_datetime(2017, 3, 7),
            CompilerVersion::VisualStudio2019 => date_to_datetime(2019, 4, 2),
            CompilerVersion::VisualStudio2022 => date_to_datetime(2021, 11, 8),
        }
    }

    pub fn newest_possible(datetime: DateTime<Utc>) -> Self {
        let mut newest = Self::VisualCPP1_0;

        for v in Self::values() {
            if v.release_date() > datetime {
                return newest;
            } else {
                newest = *v;
            }
        }

        newest
    }

    fn values() -> &'static [Self] {
        &[
            Self::VisualCPP1_0,
            Self::VisualCPP1_5,
            Self::VisualCPP2_0,
            Self::VisualCPP4_0,
            Self::VisualCPP4_1,
            Self::VisualCPP4_2,
            Self::VisualCPP5_0,
            Self::VisualCPP6_0,
            Self::VisualStudioDotNet2002,
            Self::VisualStudioDotNet2003,
            Self::VisualStudio2005,
            Self::VisualStudio2008,
            Self::VisualStudio2010,
            Self::VisualStudio2012,
            Self::VisualStudio2013,
            Self::VisualStudio2015,
            Self::VisualStudio2017,
            Self::VisualStudio2019,
            Self::VisualStudio2022,
        ]
    }
}
