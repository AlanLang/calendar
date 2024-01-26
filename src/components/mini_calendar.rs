use chrono::Datelike;
use leptos::*;
use phosphor_leptos::{ArrowArcLeft, CaretDown, CaretUp};

use crate::app::Day;

#[component]
pub fn MiniCalendar(
  value: Memo<Vec<Day>>,
  show_reset: Memo<bool>,
  #[prop(into)] on_switch_page: Callback<bool>,
  #[prop(into)] on_reset: Callback<()>,
) -> impl IntoView {
  view! {
    <div class="font-sans">
      <div class="h-[28px] flex items-center justify-end gap-1">

        <Show when=move || { show_reset.get() }>
          <button
            class="w-[20px] h-[20px] flex items-center justify-center rounded hover:bg-object-highlight-bg-emphasized active:bg-gray-150 hover:transition"
            on:click=move |_| on_reset.call(())
          >
            <ArrowArcLeft color="#999999" size="11px"/>
          </button>
        </Show>
        <button
          class="w-[20px] h-[20px] flex items-center justify-center rounded hover:bg-object-highlight-bg-emphasized active:bg-gray-150 hover:transition"
          on:click=move |_| on_switch_page.call(false)
        >
          <CaretUp color="#999999" size="11px"/>
        </button>
        <button
          class="w-[20px] h-[20px] flex items-center justify-center rounded hover:bg-object-highlight-bg-emphasized active:bg-gray-150 hover:transition"
          on:click=move |_| on_switch_page.call(true)
        >
          <CaretDown color="#999999" size="11px"/>
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
            .collect::<Vec<_>>()}
        <For
          each=move || value.get()
          key=|day| format!("{}{}{}{}", day.year, day.month, day.day, day.day_in_month)
          let:child
        >
          <div
            class="text-center text-body w-[23px] h-[23px] flex items-center justify-center rounded hover:bg-object-highlight-bg-emphasized hover:transition"
            style=get_day_style(&child)
          >
            {child.day}
          </div>
        </For>
      </div>
    </div>
  }
}

fn get_day_style(day: &Day) -> String {
  let today = chrono::Local::now().naive_local();
  if day.year == today.year() && day.month == today.month() && day.day == today.day() {
    return "background-color: #F04842;color:#FFF".to_string();
  } else if day.day_in_month {
    return "".to_string();
  } else {
    return "opacity: 0.25;".to_string();
  }
}
