use log::info;
use serde::{Deserialize, Serialize};

use crate::holiday::fetch_holidays;

pub const CHINESE_HOLIDAYS_NAME: &str = "中国节假日";

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
pub struct EventGroup {
  pub name: String,
  pub color: String,
  pub events: Vec<Event>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
pub struct Event {
  pub day: String,
  pub title: String,
  pub is_lunar: bool,
}

impl EventGroup {
  pub fn extend_events(&mut self, events: Vec<Event>) {
    self.events.extend(events);
  }
}

pub fn create_holiday_events() -> EventGroup {
  EventGroup {
    name: CHINESE_HOLIDAYS_NAME.to_string(),
    color: "#00d48a".to_string(),
    events: Vec::new(),
  }
}

pub async fn fetch_holiday_events(year: String) -> Vec<Event> {
  let mut events: Vec<Event> = Vec::new();
  let days = fetch_holidays(year).await;
  if let Ok(days) = days {
    for day in days {
      let mut name = day.name.clone();
      if day.is_off_day == false {
        name.push_str(" (补班)");
      }
      let event = Event {
        day: day.date,
        title: name,
        is_lunar: false,
      };
      events.push(event);
    }
  }
  info!("events: {:?}", events);
  events
}
