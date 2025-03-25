use yew::prelude::*;
use stylist::yew::styled_component;
use yew_router::prelude::*;

use crate::styles::*;
use crate::route::*;

#[styled_component(Home)]
pub fn home() -> Html {
    html! (
        <>
            // main containts
          <BrowserRouter>
            <div class={classes!(responsive_styles())}>
                <Link<Route> to={Route::NowInProduction}>
                    <img class={classes!(title_logo())} src={"millmill/public/yajirushi08_cycle.png"}/>
                </Link<Route>>
                <Link<Route> to={Route::ToolSetting}>
                    <img class={classes!(title_logo())} src={"millmill/public/machine_senban.png"}/>
                </Link<Route>>
                <Link<Route> to={Route::InstructionsCreation}>
                    <img class={classes!(title_logo())} src={"millmill/public/document_syorui_pen.png"}/>
                </Link<Route>>
            </div>
          </BrowserRouter>
        </>
    )
}
