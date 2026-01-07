use crate::bin_day::{CouncilBinCalendar, moray_council::MorayCouncilBinCalendar};

fn main() {
    let council = get_council_calendar();
    let next_day = council.get_next_bin_day();

    match next_day {
        Some(day) => println!(
            "Next bin day is {} for {}",
            // day.date.format("%s %e %b"),
            day.date.to_string(),
            day.types.join(", "),
        ),
        None => println!("Could not find next bin day"),
    }
}

fn get_council_calendar() -> impl CouncilBinCalendar {
    MorayCouncilBinCalendar::new()
}

mod bin_day {
    use chrono::NaiveDate;

    pub struct BinDay {
        pub date: NaiveDate,
        pub types: Vec<String>,
    }

    pub trait CouncilBinCalendar {
        fn get_next_bin_day(&self) -> Option<BinDay>;
    }

    pub mod moray_council {
        use chrono::{Days, Utc};

        use crate::bin_day::{BinDay, CouncilBinCalendar};

        pub struct MorayCouncilBinCalendar {}

        impl MorayCouncilBinCalendar {
            pub fn new() -> Self {
                Self {}
            }
        }

        impl CouncilBinCalendar for MorayCouncilBinCalendar {
            // https://bindayfinder.moray.gov.uk/cal_2026_view.php?id={{id}}
            fn get_next_bin_day(&self) -> Option<BinDay> {
                let future_date = Utc::now().checked_add_days(Days::new(5)).unwrap();
                let next_day = BinDay {
                    date: future_date.date_naive(),
                    types: vec![String::from("green")],
                };
                Some(next_day)
            }
        }
    }
}
