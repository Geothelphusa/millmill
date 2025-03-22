use yew_router::prelude::*;
use crate::gantt_chart::*;


#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/service")]
    Service,
    #[at("/news")]
    News,
    #[at("/blog")]
    Blog,
    #[at("/GanttChart")]
    GanttChart,
}

use yew::prelude::*;

pub fn switch(route: Route) -> Html {
  match route {
      Route::Home => html! { <h1>{ "Home" }</h1> },
      Route::About => html! { <h1>{ "About Page" }</h1> },
      Route::Service => html! { <h1>{ "Service Page" }</h1> },
      Route::News => html! { <h1>{ "News Page" }</h1> },
      Route::Blog => html! { <h1>{ "Blog Page" }</h1> },
      Route::GanttChart => html! { <GanttChart /> },
  }
}
