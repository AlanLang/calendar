mod app;
mod body;
mod components;
mod event;
mod holiday;
mod icons;
mod menu;

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

#[component]
fn App() -> impl IntoView {
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
      <div></div>
    </div>
  }
}
