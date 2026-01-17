use std::env;

use crate::bin_day::CouncilBinCalendar;
use crate::councils::moray_council::MorayCouncilBinCalendar;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let (_, brgs) = args.split_at(1);
    let council = get_council_calendar(&brgs);
    let next_day = council.and_then(|c| c.get_next_bin_day());

    match next_day {
        Some(day) => println!(
            "Next bin day is {} for {}",
            day.date.to_string(),
            day.types.join(", "),
        ),
        None => println!("Could not find next bin day"),
    }
}

fn get_council_calendar(args: &[String]) -> Option<impl CouncilBinCalendar> {
    let (council_name, rest) = args.split_first()?;

    match council_name.as_str() {
        "moray" => MorayCouncilBinCalendar::new(rest),
        _ => None,
    }
}

mod bin_day {
    use chrono::{DateTime, Utc};

    #[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
    pub struct BinDay {
        pub date: DateTime<Utc>,
        pub types: Vec<String>,
    }

    pub trait CouncilBinCalendar {
        fn new(args: &[String]) -> Option<Self>
        where
            Self: Sized;
        fn get_next_bin_day(&self) -> Option<BinDay>;
    }
}

pub mod councils;
