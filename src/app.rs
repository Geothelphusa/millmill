use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use stylist::yew::styled_component;
use yew_router::prelude::*;
use web_sys::MouseEvent;


use crate::styles::*;
use crate::components::*;
use crate::route::*;
use yew_router::BrowserRouter;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[styled_component(App)]
pub fn app() -> Html {
    let stylesheet = responsive_styles();

    let is_menu_opened = use_state(|| false);

    let onclick = {
        let is_menu_opened_clone = is_menu_opened.clone();
        Callback::from(move |_| is_menu_opened_clone.set(!*is_menu_opened_clone))
    };

    let onclick_clone = onclick.clone();

    let greet_input_ref = use_node_ref();

    let name = use_state(|| String::new());

    let greet_msg = use_state(|| String::new());
    {
        let greet_msg = greet_msg.clone();
        let name = name.clone();
        let name2 = name.clone();
        use_effect_with(
            name2,
            move |_| {
                spawn_local(async move {
                    if name.is_empty() {
                        return;
                    }

                    let args = serde_wasm_bindgen::to_value(&GreetArgs { name: &*name }).unwrap();
                    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
                    let new_msg = invoke("greet", args).await.as_string().unwrap();
                    greet_msg.set(new_msg);
                });

                || {}
            },
        );
    }

    let greet = {
        let name = name.clone();
        let greet_input_ref = greet_input_ref.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            name.set(
                greet_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
        })
    };

    html! {
        <main class={classes!(app_styles(), responsive_styles())}>
            <div class="container">
            <h1>{"Welcome to Tauri + Yew"}</h1>

            <div class="row">
                <a href="https://tauri.app" target="_blank">
                    <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
                </a>
                <a href="https://yew.rs" target="_blank">
                    <img src="public/yew.png" class="logo yew" alt="Yew logo"/>
                </a>
            </div>
            <p>{"Click on the Tauri and Yew logos to learn more."}</p>

            <form class="row" onsubmit={greet}>
                <input id="greet-input" ref={greet_input_ref} placeholder="Enter a name..." />
                <button type="submit">{"Greet"}</button>
            </form>
            <p>{ &*greet_msg }</p>
            <BrowserRouter>
                    <nav class={classes!(nav_styles())}>
                        <MenuButton onclick={onclick_clone} is_opened={*is_menu_opened} />
                    </nav>
                    <ul class={css!("display: flex; flex-direction: column; @media (min-width: 768px) {flex-direction: row;}")}>
                        { if *is_menu_opened {
                            html! {
                                <div class={classes!(overlay_style(), "is-opened")} onclick={onclick.clone()}>
                                    <div class={classes!(menu_style())} onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                                        <ul class={classes!(menu_list_style())}>
                                            { for vec![
                                                (Route::Home, "HOME"),
                                                (Route::About, "ABOUT"),
                                                (Route::Service, "SERVICE"),
                                                (Route::News, "NEWS"),
                                                (Route::Blog, "BLOG"),
                                            ].into_iter().map(|(route, label)| html! {
                                                <li><Link<Route> to={route} classes={classes!(menu_items())}>{ label }</Link<Route>></li>
                                            }) }
                                        </ul>
                                    </div>
                                </div>
                            }
                        } else {
                            html! {}
                        }}
                    </ul>
                    <Switch<Route> render={switch} />
                </BrowserRouter>
            </div>
        </main>
    }
}
