use chinese_lunisolar_calendar::LunisolarDate;
use chrono::{Datelike, Local, NaiveDate, NaiveDateTime};
use gloo::console::info;
use leptos::{spawn_local, RwSignal, SignalGet, SignalGetUntracked, SignalSet};

use crate::event::{
  create_holiday_events, fetch_holiday_events, EventGroup, CHINESE_HOLIDAYS_NAME,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct App {
  pub show_left_menu: RwSignal<bool>,
  pub scroll_top: RwSignal<i32>,
  pub year: RwSignal<i32>,
  pub month: RwSignal<u32>,
  pub days: RwSignal<Vec<Day>>,
  pub start_timestamp: i64,
  pub rate: i64,
  pub item_height: i64,
  pub global_height: i64,
  pub content_height: f64,
  week_timestamp: i64,
  pub events: RwSignal<Vec<RwSignal<EventGroup>>>,
  pub selected_day: RwSignal<Day>,
}

impl Default for App {
  fn default() -> Self {
    let week_total = 2660;
    let week_show_num = 6;
    let week_timestamp = 604800000;

    let today = Local::now().date_naive();
    let year = today.year();
    let month = today.month();

    let window = web_sys::window().unwrap();
    let content_height = window.inner_height().unwrap().as_f64().unwrap() - 45.0 - 24.0 - 30.0;
    let item_height = nearest_multiple_of_six(content_height - 1.0) / week_show_num;
    let rate = week_timestamp / item_height;
    let global_height = item_height * week_total;

    let today = chrono::Local::now().date_naive();
    let start_date = get_start_date(today);
    let start_date_timestamp = start_date.and_hms_opt(0, 0, 0).unwrap().timestamp_millis();
    let start_timestamp =
      start_date_timestamp - (week_timestamp * (week_total - week_show_num) / 2);

    let app = App {
      year: RwSignal::new(year),
      month: RwSignal::new(month),
      show_left_menu: RwSignal::new(true),
      days: RwSignal::new(generate_days(today)),
      start_timestamp,
      rate,
      global_height,
      content_height,
      item_height,
      week_timestamp,
      scroll_top: RwSignal::new(0),
      events: RwSignal::new(vec![RwSignal::new(create_holiday_events())]),
      selected_day: RwSignal::new(Day::new(today)),
    };
    app.update_scroll_top();
    let events = app.events.get_untracked();
    let holiday = events
      .into_iter()
      .find(|e| e.get_untracked().name == CHINESE_HOLIDAYS_NAME)
      .unwrap();
    let holiday_events = holiday.get_untracked().events;
    // 如果没有当前年份的事件
    if !holiday_events
      .into_iter()
      .any(|e| e.day.starts_with(&year.to_string()))
    {
      spawn_local(async move {
        let es = fetch_holiday_events(year.to_string()).await;
        let mut mut_holiday = holiday.get_untracked();
        mut_holiday.extend_events(es);
        holiday.set(mut_holiday);
      });
    }
    app
  }
}

impl App {
  pub fn generate_days(&self) {
    let day =
      NaiveDate::from_ymd_opt(self.year.get_untracked(), self.month.get_untracked(), 1).unwrap();
    let days = generate_days(day);
    self.days.set(days);
  }

  pub fn get_scroll_to_day(&self, top: i32) -> Option<NaiveDateTime> {
    let timestamp = (top as i64 + self.item_height * 2) * self.rate + self.start_timestamp;
    chrono::NaiveDateTime::from_timestamp_opt(timestamp / 1000, 0)
  }

  fn update_scroll_top(&self) {
    let binding = self.days.get_untracked();
    let start_day = get_start_date_of_week(
      NaiveDate::from_ymd_opt(self.year.get_untracked(), self.month.get_untracked(), 1).unwrap(),
    );
    let show_day = binding
      .iter()
      .find(|day| {
        day.year == start_day.year() && day.month == start_day.month() && day.day == start_day.day()
      })
      .unwrap();
    let timestamp = show_day.timestamp;
    let scroll_top = (timestamp - self.start_timestamp) / self.rate;
    self.scroll_top.set(scroll_top as i32);
  }

  pub fn get_scroll_position(&self, top: i32, down: bool) -> i32 {
    let item_height = self.item_height as i32;
    let a = top % item_height;
    if a < item_height / 3 {
      return top - a;
    }
    if a > item_height - item_height / 3 {
      return top + item_height - a;
    }

    if down {
      top + item_height - a
    } else {
      top - a
    }
  }
}

impl App {
  pub fn go_to_today(&self) {
    let today = chrono::Local::now().date_naive();
    self.set_year_month_without_scroll(today.year(), today.month());
    self.update_scroll_top();
  }

  pub fn next_mount(self) {
    if self.month.get() == 12 {
      self.month.set(1);
      self.year.set(self.year.get() + 1);
    } else {
      self.month.set(self.month.get() + 1);
    }
    self.generate_days();
    self.update_scroll_top();
  }

  pub fn prev_mount(self) {
    if self.month.get() == 1 {
      self.month.set(12);
      self.year.set(self.year.get() - 1);
    } else {
      self.month.set(self.month.get() - 1);
    }
    self.generate_days();
    self.update_scroll_top();
  }

  pub fn set_year_month_without_scroll(&self, year: i32, month: u32) {
    if year == self.year.get_untracked() && month == self.month.get_untracked() {
      return;
    }
    self.year.set(year);
    self.month.set(month);
    self.generate_days();
  }

  pub fn set_selected_day(&self, day: Day) {
    self.selected_day.set(day);
  }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Day {
  pub year: i32,
  pub month: u32,
  pub day: u32,
  pub date: NaiveDate,
  pub timestamp: i64,
  pub is_weekend: bool,
  pub lunar: LunisolarDate,
}

impl Day {
  pub fn new(day: NaiveDate) -> Self {
    let timestamp = day.and_hms_opt(0, 0, 0).unwrap().timestamp_millis();
    let lunisolar_date = LunisolarDate::from_date(day).unwrap();
    Day {
      year: day.year(),
      month: day.month(),
      day: day.day(),
      date: day,
      timestamp,
      is_weekend: day.weekday() == chrono::Weekday::Sat || day.weekday() == chrono::Weekday::Sun,
      lunar: lunisolar_date,
    }
  }
}

pub fn get_start_date(date: NaiveDate) -> NaiveDate {
  let first_day_of_month = date.with_day(1).unwrap();
  let starting_weekday = first_day_of_month.weekday();
  let days_to_go_back = starting_weekday.num_days_from_sunday();
  first_day_of_month - chrono::Duration::days(days_to_go_back as i64)
}

pub fn get_start_date_of_week(date: NaiveDate) -> NaiveDate {
  let starting_weekday = date.weekday();
  let days_to_go_back = starting_weekday.num_days_from_sunday();
  date - chrono::Duration::days(days_to_go_back as i64)
}

pub fn generate_days(show_date: NaiveDate) -> Vec<Day> {
  info!("generate_days {}:?", show_date.to_string());
  let start_date = get_start_date_of_week(show_date);
  let start_date = start_date - chrono::Duration::days(35);
  let mut days: Vec<Day> = Vec::new();
  for i in 0..112 {
    let day = start_date + chrono::Duration::days(i);
    days.push(Day::new(day));
  }
  days
}

pub fn filter_calendar_by_year_month(days: Vec<Day>, year: i32, month: u32) -> Vec<Day> {
  let date = get_start_date_of_week(NaiveDate::from_ymd_opt(year, month, 1).unwrap());
  let index = days
    .iter()
    .position(|day| day.date == date)
    .unwrap_or_default();
  days[index..index + 42].to_vec()
}

fn nearest_multiple_of_six(number: f64) -> i64 {
  (number / 6.0).round() as i64 * 6
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn start_date() {
    let date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    assert_eq!(
      get_start_date(date),
      NaiveDate::from_ymd_opt(2023, 12, 31).unwrap()
    );

    let date = NaiveDate::from_ymd_opt(2024, 1, 26).unwrap();
    assert_eq!(
      get_start_date(date),
      NaiveDate::from_ymd_opt(2023, 12, 31).unwrap()
    );
  }

  #[test]
  fn timestamp() {
    let date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    let timestamp = date.and_hms_opt(0, 0, 0).unwrap().timestamp_millis();
    assert_eq!(timestamp, 1704067200000)
  }
}
