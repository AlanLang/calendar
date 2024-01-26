mod app;
mod body;
mod components;
mod event;
mod holiday;
mod icons;
mod menu;

use anyhow::{anyhow, Result};
use gloo_net::http::Request;
use holiday::Holiday;
use leptos::*;
use log::info;
use wasm_bindgen_console_logger::DEFAULT_LOGGER;

use crate::{app::App, body::Content, menu::Menu};

fn main() {
  info!("init");
  log::set_logger(&DEFAULT_LOGGER).unwrap();
  log::set_max_level(log::LevelFilter::Info);
  console_error_panic_hook::set_once();
  leptos::mount_to_body(|| view! { <App/> })
}

async fn fetch_holidays_resource(year: String) -> Vec<Holiday> {
  let holidays = fetch_holidays(year).await;
  match holidays {
    Ok(holidays) => holidays,
    Err(e) => {
      info!("{:?}", e);
      vec![]
    }
  }
}

async fn fetch_holidays(year: String) -> Result<Vec<Holiday>> {
  let url = format!(
    "https://raw.githubusercontent.com/NateScarlet/holiday-cn/master/{}.json",
    year
  );
  let resp = Request::get(&url)
    .send()
    .await
    .map_err(|e| anyhow!("请求日期数据出错: {:?}", e))?;
  let text = resp
    .text()
    .await
    .map_err(|e| anyhow::anyhow!("获取日期数据出错:{:?}", e))?;
  let holidays = serde_json::from_str::<holiday::HolidayResult>(&text)
    .map_err(|e| anyhow!("解析日期数据出错: {:?}", e))?;
  Ok(holidays.days)
}

#[component]
fn App() -> impl IntoView {
  let holidays = create_resource(move || "2024".to_string(), fetch_holidays_resource);

  let app = App::default();
  provide_context(app);

  let style = move || {
    if app.show_left_menu.get() {
      return "grid-template-columns: 208px 1fr 300px;";
    } else {
      return "grid-template-columns: 0px 1fr 300px;";
    }
  };

  view! {
    <div class="w-full h-full grid relative transition-all select-none" style=style>
      <Menu/>
      <Content/>
      <div>
        {move || match holidays.get() {
            None => view! { <p>"Loading..."</p> }.into_view(),
            Some(data) => view! { <div>{data.len()}</div> }.into_view(),
        }}

      </div>
    </div>
  }
}
