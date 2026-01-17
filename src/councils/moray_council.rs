use chrono::{Datelike, TimeZone, Utc};
use chrono_tz::Europe::London;
use scraper::{Html, Selector};

use crate::bin_day::{BinDay, CouncilBinCalendar};

pub struct MorayCouncilBinCalendar {
    property_id: String,
}

impl MorayCouncilBinCalendar {
    fn get_url(year: i32, property_id: &String) -> String {
        format!("https://bindayfinder.moray.gov.uk/cal_{year}_view.php?id={property_id}")
    }

    fn get_calendar(&self, year: i32) -> Result<Vec<BinDay>, reqwest::Error> {
        let client = reqwest::blocking::Client::builder()
            .tls_backend_native()
            .build()?;

        let url = Self::get_url(year, &self.property_id);
        let html = client.get(dbg!(url)).send()?.error_for_status()?.text()?;

        let document = Html::parse_document(&html);
        let month_selector = Selector::parse(".month-container .days-container").unwrap();
        let day_selector = Selector::parse(r#":not([class|=""]):not([class|="blank"])"#).unwrap();

        let months = document.select(&month_selector).enumerate();

        let mut stuff: Vec<BinDay> = months
            .flat_map(|(month_num, month)| {
                month.select(&day_selector).filter_map(move |day_element| {
                    let month = u32::try_from(month_num).ok()? + 1;
                    let day = u32::from_str_radix(
                        day_element.text().collect::<String>().trim().into(),
                        10,
                    )
                    .ok()?;

                    let date = London
                        .with_ymd_and_hms(year, month, day, 0, 0, 0)
                        .earliest()?
                        .to_utc();

                    let types = day_element
                        .attr("class")?
                        .chars()
                        .map(|c| c.to_string())
                        .collect();

                    Some(BinDay {
                        date: date,
                        types: types,
                    })
                })
            })
            .collect();

        stuff.sort_by(|a, b| a.date.cmp(&b.date));

        Ok(stuff)
    }
}

impl CouncilBinCalendar for MorayCouncilBinCalendar {
    fn new(args: &[String]) -> Option<Self> {
        Some(MorayCouncilBinCalendar {
            property_id: (args.first()?.to_string()),
        })
    }
    fn get_next_bin_day(&self) -> Option<BinDay> {
        let now = Utc::now();
        let calendar = self.get_calendar(now.year()).unwrap();

        calendar.into_iter().find(|day| now.lt(&day.date))
    }
}
