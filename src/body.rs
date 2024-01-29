use chinese_lunisolar_calendar::LunarDay;
use chrono::Datelike;
use leptos::{html::Div, *};
use phosphor_leptos::{CaretDown, CaretUp};
use web_sys::{ScrollBehavior, ScrollToOptions};

use crate::app::{App, Day};

const LUNAR_MONTH:[&str;12]=["正月", "二月", "三月", "四月", "五月", "六月", "七月", "八月","九月", "十月", "冬月", "腊月"];
const LUNAR_DAY:[&str;30]=["初一", "初二", "初三", "初四", "初五", "初六", "初七", "初八","初九", "初十", "十一", "十二", "十三", "十四","十五", "十六", "十七", "十八","十九", "二十", "廿一", "廿二","廿三", "廿四", "廿五", "廿六","廿七", "廿八", "廿九", "三十"];

#[component]
pub fn Content() -> impl IntoView {
  let body_ref = create_node_ref::<Div>();
  let app = use_context::<App>().expect("there to be a `count` signal provided");
  let (start, set_start) = create_signal(Some(0));

  let days: Memo<Vec<Vec<Day>>> = create_memo(move |_| {
    app
      .days
      .get()
      .chunks(7)
      .map(|chunk| chunk.to_vec())
      .collect()
  });

  let handle_scroll = move |_e: leptos::ev::Event| {
    if let Some(body_ref) = body_ref.get() {
      let top = body_ref.scroll_top();
      if start.get().is_none() {
        set_start.set(Some(top));
      }
      if let Some(date) = app.get_scroll_to_day(top) {
        app.set_year_month_without_scroll(date.year(), date.month());
      }
    }
  };

  let handle_scroll_end = move |_e: leptos::ev::Event| {
    if let Some(body_ref) = body_ref.get() {
      if let Some(start) = start.get() {
        let top = body_ref.scroll_top();
        if start != top {
          let is_down = top > start;
          set_start.set(None);
          let position = app.get_scroll_position(top, is_down);
          let mut options = ScrollToOptions::new();
          options.top(position as f64);
          options.behavior(ScrollBehavior::Smooth);
          body_ref.scroll_with_scroll_to_options(&options);
        }
      }
    }
  };

  create_effect(move |_| {
    let scroll_top = app.scroll_top.get();
    if let Some(el) = body_ref.get_untracked() {
      el.set_scroll_top(scroll_top);
    }
    scroll_top
  });

  view! {
    <div class="right-line flex flex-col grid-rows-[45px_24px_1fr] h-screen relative">
      <div class="h-[45px] flex justify-end items-center gap-2 px-4">
        <button
          type="button"
          class="h-[28px] rounded bg-white px-3 py-1 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50 active:bg-gray-100"
          on:click=move |_e| {
              app.go_to_today();
          }
        >

          Today
        </button>
        <button
          class="w-[20px] h-[20px] text-gray-400 hover:text-gray-600 flex items-center justify-center rounded hover:bg-object-highlight-bg-emphasized hover:transition active:bg-gray-150"
          on:click=move |_e| { app.prev_mount() }
        >

          <CaretUp size="15px"/>
        </button>
        <button
          class="w-[20px] h-[20px] text-gray-400 hover:text-gray-600 flex items-center justify-center rounded hover:bg-object-highlight-bg-emphasized hover:transition active:bg-gray-150"
          on:click=move |_e| { app.next_mount() }
        >
          <CaretDown size="15px"/>
        </button>
      </div>
      <div class="px-6 text-2xl font-medium tracking-tight overflow-hidden">
        {move || format!("{} 年 {} 月", app.year.get(), app.month.get())}
      </div>
      <div class="flex flex-col" style="flex:1;overflow-y: auto;">
        <div class="p-1.5 text-base grid grid-cols-7 border-b">
          {vec!["周日", "周一", "周二", "周三", "周四", "周五", "周六"]
              .into_iter()
              .map(|day| {
                  view! { <div class="pt-1 text-center text-body">{day}</div> }
              })
              .collect::<Vec<_>>()}
        </div>
        <div
          class="overflow-auto"
          on:scroll=handle_scroll
          on:scrollend=handle_scroll_end
          style=format!(
              "height: {}px;scroll-snap-type: y mandatory;overscroll-behavior-y: none;",
              app.content_height,
          )

          node_ref=body_ref
        >
          <div class="relative" style=format!("height: {}px;", app.global_height)>
            <For each=move || days.get() key=|day| day[0].key let:week>
              <div
                class="bottom-line grid grid-cols-7 absolute top-0 left-0 right-0 transition-all"
                data-timestamp=week[0].timestamp
                data-date=week[0].date.to_string()
                style=format!(
                    "transform: translateY({}px); height: {}px",
                    (week[0].timestamp - app.start_timestamp) / app.rate,
                    app.item_height,
                )
              >

                <For each=move || week.clone() key=|day| day.key let:day>
                  <div class="text-base p-2 relative top-line" style=get_day_content_style(&day)>
                    <div class="flex justify-between items-center" style=get_day_style(&day)>
                    <div class="">{get_lunar_day_text(&day)}</div>
                      <div
                        class="rounded w-[25px] h-[25px] flex items-center justify-center"
                        style=get_day_num_style(&day)
                      >
                        {day.day}
                      </div>
                    </div>
                  </div>
                </For>
              </div>
            </For>
          </div>
        </div>
      </div>
    </div>
  }
}

fn get_day_style(day: &Day) -> String {
  if day.day_in_month {
    return "".to_string();
  } else {
    return "opacity: 0.25;".to_string();
  }
}

fn get_day_content_style(day: &Day) -> String {
  if day.is_weekend {
    return "background: #FCFCFC".to_string();
  } else {
    return "".to_string();
  }
}

fn get_day_num_style(day: &Day) -> String {
  let today = chrono::Local::now().naive_local();
  if day.year == today.year() && day.month == today.month() && day.day == today.day() {
    return "background-color: #F04842; color:#FFF".to_string();
  }
  "".to_string()
}

fn get_lunar_day_text(day: &Day) -> String {
  let lunar = day.lunar.to_lunar_day();
  match lunar {
    LunarDay::First => format!("{:#}{:#}", day.lunar.to_lunar_month(), lunar),
    _ => format!("{:#}", lunar),
  }

}