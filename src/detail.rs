use leptos::*;

use crate::App;

#[component]
pub fn DetailPanel() -> impl IntoView {
  let app = use_context::<App>().expect("there to be a `count` signal provided");
  let day = move || app.selected_day.get();
  let day_text = move || day().date.format("%Y 年 %m 月 %d 日").to_string();
  let lunisolar_date = move || day().lunar;
  let lunar_day_text = move || {
    format!(
      "{:#}{:#}",
      lunisolar_date().to_lunar_month(),
      lunisolar_date().to_lunar_day()
    )
  };

  let distance_today_text = move || {
    let today = chrono::Local::now().naive_local();
    let distance = app
      .selected_day
      .get()
      .date
      .signed_duration_since(today.into())
      .num_days();
    match distance {
      0 => "今天".to_string(),
      1 => "明天".to_string(),
      -1 => "昨天".to_string(),
      d if d > 0 => format!("{} 天后", d),
      d => format!("{} 天前", -d),
    }
  };

  view! {
    <div class="py-8 text-sm">
      <div class="px-2">
        <div class="text-base font-semibold">{day_text}</div>
        <div class="">{lunar_day_text}</div>
      </div>
      <div class="relative h-8">
        <div class="absolute inset-0 flex items-center" aria-hidden="true">
          <div class="w-full border-t border-line-divider"></div>
        </div>
      </div>
      <div class="px-2">
        <div class="">{move || distance_today_text()}</div>
      </div>
    </div>
  }
}
