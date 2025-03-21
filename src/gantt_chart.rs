use std::cell::RefCell;
use std::rc::Rc;

use chrono::Datelike;
use stylist::yew::styled_component;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use stylist::style;

use crate::components::*;
use crate::styles::*;

use chrono::{NaiveDateTime, Duration};
use gloo_events::EventListener;

#[derive(Clone, PartialEq)]
struct Task {
    id: usize,
    name: &'static str,
    start_date: NaiveDateTime,
    end_date: NaiveDateTime,
    color: &'static str,
}

// タスクの初期状態
fn initial_tasks() -> Vec<Task> {
    let base_date = NaiveDateTime::parse_from_str("2025-03-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    vec![
        Task { id: 1, name: "", start_date: base_date, end_date: base_date + Duration::days(5), color: "#4CAF50" },
        Task { id: 2, name: "", start_date: base_date + Duration::days(6), end_date: base_date + Duration::days(9), color: "#FF9800" },
        Task { id: 3, name: "", start_date: base_date + Duration::days(10), end_date: base_date + Duration::days(14), color: "#673AB7" },
    ]
}

#[styled_component(GanttChart)]
pub fn gantt_chart() -> Html {
    
    let tasks = use_state(initial_tasks);
    let from_date_ref = use_state(|| Rc::new(RefCell::new(None)));
    let tasks_ref = use_state(|| tasks.clone());

    // マウスダウン時に開始位置を記録
    let on_mouse_down = {
        let from_date_ref = from_date_ref.clone();
        Callback::from(move |(e, task): (MouseEvent, Task)| {
            e.prevent_default(); // Prevent text selection
            from_date_ref.borrow_mut().replace(task.start_date);
        })
    };    

    // マウス移動時にタスクを移動
    let on_mouse_move = {
        let from_date_ref = from_date_ref.clone();
        let tasks_clone = tasks_ref.clone(); // tasks_refをclone
        Callback::from(move |e: MouseEvent| {
            if let Some(from_date) = *from_date_ref.borrow() {
                let diff_days = (e.movement_x() as i64) / 10;
                let new_tasks: Vec<Task> = tasks_clone.iter().map(|task| {
                    Task {
                        start_date: task.start_date + Duration::days(diff_days),
                        end_date: task.end_date + Duration::days(diff_days),
                        ..task.clone()
                    }
                }).collect();
                tasks.set(new_tasks);
            }
        })
    };

    // マウスアップ時にリセット
    let on_mouse_up = {
        let from_date_ref = from_date_ref.clone();
        Callback::from(move |_| {
            from_date_ref.borrow_mut().take();
        })
    };

    // グローバルイベントリスナーを登録
    {
        let on_mouse_move = on_mouse_move.clone();
        let on_mouse_up = on_mouse_up.clone();
        use_effect_with((), move |_| {
                let move_listener = EventListener::new(&gloo::utils::window(), "mousemove", move |e: &web_sys::Event| {
                    on_mouse_move.emit(e.clone().dyn_into::<web_sys::MouseEvent>().unwrap());
                });
                let up_listener = EventListener::new(&gloo::utils::window(), "mouseup", move |e: &web_sys::Event| {
                    on_mouse_up.emit(e.clone().dyn_into().unwrap());
                });
        
                move || drop((move_listener, up_listener))
            });
    }

    html! {
        <div>
            <h2>{ "ガントチャート" }</h2>
            <div style="display: grid; grid-template-columns: repeat(30, 30px); grid-template-rows: repeat(5, 40px); gap: 2px; background: #eee; padding: 10px; border-radius: 5px;">
                { for tasks.iter().enumerate().map(|(i, task)| html! {
                    <div 
                        style={format!(
                            "grid-column-start: {}; grid-column-end: {}; grid-row-start: {}; background: {}; color: white; text-align: center; padding: 5px; cursor: pointer;",
                            task.start_date.day(), 
                            task.end_date.day() + 1, 
                            i + 1,
                            task.color
                        )}
                        onmousedown={on_mouse_down.reform(move |e| (e.clone(), task.clone()))}
                    >
                        { task.name }
                    </div>
                })}
            </div>
        </div>
    }
}
