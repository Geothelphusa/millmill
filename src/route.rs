use yew_router::prelude::*;
use crate::{gantt_chart::*, home::Home};


#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/NOW_IN_PRODUCTION")]
    NowInProduction,
    #[at("/TOOL_SETTING")]
    ToolSetting,
    #[at("/Instructions_Creation")]
    InstructionsCreation,
    #[at("/GanttChart")]
    GanttChart,
}

use yew::prelude::*;

pub fn switch(route: Route) -> Html {
  match route {
      Route::Home => html! { <Home /> },
      Route::NowInProduction => html! { <h1>{ "Now In Production" }</h1> },
      Route::ToolSetting => html! { <h1>{ "Tool Setting" }</h1> },
      Route::InstructionsCreation => html! { <h1>{ "Instructions Creation" }</h1> },
      Route::GanttChart => html! { <GanttChart /> },
  }
}