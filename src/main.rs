mod app;
mod styles;
mod route;
mod components;
mod gantt_chart;

use app::App;

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
