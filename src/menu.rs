use chrono::Datelike;
use leptos::*;

use crate::{
  app::{filter_calendar_by_year_month, App},
  components::mini_calendar::MiniCalendar,
  event::EventGroup,
  icons::menu::MenuIcon,
};

#[component]
pub fn Menu() -> impl IntoView {
  let app = use_context::<App>().expect("there to be a `count` signal provided");
  let days = create_memo(move |_| {
    filter_calendar_by_year_month(app.days.get(), app.year.get(), app.month.get())
  });
  let show_reset = create_memo(move |_| {
    let today = chrono::Local::now().naive_local();
    today.year() != app.year.get() || today.month() != app.month.get()
  });

  view! {
    <div class="bg-material-base flex flex-col overflow-hidden">
      <div class="h-[44px]">
        <div
          class="fixed left-3 top-2 z-10"
          on:click=move |_e| {
              app.show_left_menu.set(!app.show_left_menu.get_untracked());
          }
        >

          <MenuIcon/>
        </div>
      </div>
      <div class="border-b p-2">
        <MiniCalendar
          value=days
          show_reset=show_reset
          on_switch_page=move |is_next: bool| {
              if is_next {
                  app.next_mount();
              } else {
                  app.prev_mount();
              }
          }

          on_reset=move |_| {
              app.go_to_today();
          }
        />

      </div>
      <div class="">
        <div class="p-2 pt-4 text-gray-400" style="font-size: 10px">
          事件列表
        </div>
        <div class="">
          <For each=move || app.events.get() key=|e| e.get().name let:event>
            <EventItem value=event/>
          </For>
        </div>
      </div>
    </div>
  }
}

#[component]
pub fn EventItem(value: RwSignal<EventGroup>) -> impl IntoView {
  view! {
    <div class="h-[28px] p-1 px-2 text-sm text-default-subdued ">
      <div class="gap-1 p-1 flex items-center rounded hover:bg-object-highlight-bg-emphasized hover:transition">
        <div class="h-[16px] w-[16px] flex items-center justify-center">
          <div
            class="h-[10px] w-[10px]"
            style=format!("background-color: {};border-radius: 3px;", value.get().color)
          ></div>
        </div>
        <div class="">{move || value.get().name}</div>
      </div>

    </div>
  }
}
