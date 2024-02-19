use chrono::Datelike;
use icondata as i;
use leptos::*;
use leptos_icons::Icon;

use crate::app::{filter_calendar_by_year_month, App, Day};

#[component]
pub fn MiniCalendar() -> impl IntoView {
  let app = use_context::<App>().expect("there to be a `count` signal provided");
  let show_reset = create_memo(move |_| {
    let today = chrono::Local::now().naive_local();
    today.year() != app.year.get() || today.month() != app.month.get()
  });
  let value = create_memo(move |_| {
    filter_calendar_by_year_month(app.days.get(), app.year.get(), app.month.get())
  });

  view! {
    <div class="font-sans">
      <div class="h-[28px] flex items-center justify-end gap-1">

        <Show when=move || { show_reset.get() }>
          <button
            class="w-[20px] h-[20px] text-gray-400 flex items-center justify-center rounded hover:text-gray-600 hover:bg-object-highlight-bg-emphasized active:bg-gray-150 hover:transition text-sm"
            on:click=move |_| app.go_to_today()
          >
            <Icon icon=i::RiArrowGoBackArrowsLine/>
          </button>
        </Show>
        <button
          class="w-[20px] h-[20px] text-gray-400 flex items-center justify-center rounded hover:text-gray-600 hover:bg-object-highlight-bg-emphasized active:bg-gray-150 hover:transition text-sm"
          on:click=move |_| app.prev_mount()
        >
          <Icon icon=i::BsChevronUp/>
        </button>
        <button
          class="w-[20px] h-[20px] text-gray-400 flex items-center justify-center rounded hover:text-gray-600 hover:bg-object-highlight-bg-emphasized active:bg-gray-150 hover:transition text-sm"
          on:click=move |_| app.next_mount()
        >
          <Icon icon=i::BsChevronDown/>
        </button>
      </div>
      <div class="w-full grid grid-cols-7 text-sm gap-1">

        {vec!["日", "一", "二", "三", "四", "五", "六"]
            .into_iter()
            .map(|day| {
                view! {
                  <div class="text-xs font-medium pt-1 text-center text-body w-[23px] h-[23px]">
                    {day}
                  </div>
                }
            })
            .collect::<Vec<_>>()} <For each=move || value.get() key=|day| day.timestamp let:child>
          <DayView day=child/>
        </For>
      </div>
    </div>
  }
}

#[component]
fn DayView(day: Day) -> impl IntoView {
  let app = use_context::<App>().expect("there to be a `count` signal provided");
  let day_style = create_memo(move |_| {
    get_day_style(
      &day,
      app.month.get() == day.month && app.year.get() == day.year,
    )
  });

  view! {
    <div
      class="text-center text-body w-[23px] h-[23px] flex items-center justify-center rounded hover:bg-object-highlight-bg-emphasized hover:transition"
      style=day_style
      on:click=move |_| app.set_selected_day(day.clone())
    >

      {day.day}
    </div>
  }
}

fn get_day_style(day: &Day, day_in_month: bool) -> String {
  let today = chrono::Local::now().naive_local();
  if day.year == today.year() && day.month == today.month() && day.day == today.day() {
    return "background-color: #F04842;color:#FFF".to_string();
  } else if day_in_month {
    return "".to_string();
  } else {
    return "opacity: 0.25;".to_string();
  }
}
