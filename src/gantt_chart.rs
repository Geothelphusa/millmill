use std::cell::RefCell;
use std::rc::Rc;

use chrono::Datelike;
use stylist::yew::styled_component;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use stylist::style;

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

fn initial_tasks() -> Vec<Task> {
    let base_date = NaiveDateTime::parse_from_str("2025-03-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    vec![
        Task { id: 1, name: "Task 1", start_date: base_date, end_date: base_date + Duration::days(5), color: "#4CAF50" },
        Task { id: 2, name: "Task 2", start_date: base_date + Duration::days(6), end_date: base_date + Duration::days(9), color: "#FF9800" },
        Task { id: 3, name: "Task 3", start_date: base_date + Duration::days(10), end_date: base_date + Duration::days(14), color: "#673AB7" },
    ]
}

#[styled_component(GanttChart)]
pub fn gantt_chart() -> Html {
    let grid_style = style!(
        r#"
        display: grid;
        grid-template-columns: repeat(30, 50px);
        grid-template-rows: repeat(5, 60px);
        gap: 4px;
        background: #f0f0f0;
        padding: 20px;
        border-radius: 10px;
        box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
        "#
    )
    .unwrap();

    let cell_style = style!(
        r#"
        width: 50px;
        height: 60px;
        background: white;
        border: 1px solid #ddd;
        display: flex;
        justify-content: center;
        align-items: center;
        "#
    )
    .unwrap();

    let task_style = style!(
        r#"
        color: white;
        text-align: center;
        padding: 8px;
        cursor: pointer;
        border-radius: 5px;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
        display: flex;
        justify-content: center;
        align-items: center;
        "#
    )
    .unwrap();

    let tasks = use_state(initial_tasks);
    let from_date_ref = use_state(|| Rc::new(RefCell::new(None)));
    let dragging_task_id = use_state(|| Rc::new(RefCell::new(None)));

    let on_mouse_down = {
        let from_date_ref = from_date_ref.clone();
        let dragging_task_id = dragging_task_id.clone();
        Callback::from(move |(e, task): (MouseEvent, Task)| {
            e.prevent_default();
            from_date_ref.set(Rc::new(RefCell::new(Some(task.start_date))));
            dragging_task_id.set(Rc::new(RefCell::new(Some(task.id))));
        })
    };

    let on_mouse_move = {
        let from_date_ref = from_date_ref.clone();
        let dragging_task_id = dragging_task_id.clone();
        let tasks = tasks.clone();
        Callback::from(move |e: MouseEvent| {
            let from_date_rc = &**from_date_ref;
            let dragging_id_rc = &**dragging_task_id;

            let from_date_option = from_date_rc.borrow();
            let dragging_id_option = dragging_id_rc.borrow();

            if let Some(_from_date) = *from_date_option {
                if let Some(dragging_id) = *dragging_id_option {
                    let diff_days = (e.movement_x() as i64) / 50;
                    let current_tasks = (*tasks).clone();
                    let new_tasks: Vec<Task> = current_tasks
                        .iter()
                        .map(|task| {
                            if task.id == dragging_id {
                                let new_start_date = task.start_date + Duration::days(diff_days);
                                let new_end_date = task.end_date + Duration::days(diff_days);
                                Task {
                                    start_date: new_start_date,
                                    end_date: new_end_date,
                                    ..task.clone()
                                }
                            } else {
                                task.clone()
                            }
                        })
                        .collect();
                    tasks.set(new_tasks);
                }
            }
        })
    };

    let on_mouse_up = {
        let from_date_ref = from_date_ref.clone();
        let dragging_task_id = dragging_task_id.clone();
        Callback::from(move |_| {
            from_date_ref.set(Rc::new(RefCell::new(None)));
            dragging_task_id.set(Rc::new(RefCell::new(None)));
        })
    };

    {
        let on_mouse_move = on_mouse_move.clone();
        let on_mouse_up = on_mouse_up.clone();
        use_effect_with((), move |_| {
            let move_listener = EventListener::new(&gloo::utils::window(), "mousemove", move |e: &web_sys::Event| {
                if let Ok(mouse_event) = e.clone().dyn_into::<web_sys::MouseEvent>() {
                    on_mouse_move.emit(mouse_event);
                }
            });
            let up_listener = EventListener::new(&gloo::utils::window(), "mouseup", move |e: &web_sys::Event| {
                if let Ok(mouse_event) = e.clone().dyn_into::<web_sys::MouseEvent>() {
                    on_mouse_up.emit(mouse_event);
                }
            });

            move || drop((move_listener, up_listener))
        });
    }

    let tasks_clone = (*tasks).clone();

    html! {
        <>
            <div>
                <h2>{ "ガントチャート" }</h2>
                <div class={grid_style}>
                    { for (0..30).map(|_row| html! {
                        { for (0..30).map(|_col| html! {
                            <div class={cell_style.clone()}></div>
                        }) }
                    }) }
                    {
                        tasks_clone.iter().enumerate().map(|(i, task)| {
                            let task_clone = task.clone();
                            let row_index = i + 1;
                            let start_day = task.start_date.day();
                            let end_day = task.end_date.day();
                            let column_start = if start_day > 0 && start_day <= 30 { start_day } else { 1 };
                            let column_end = if end_day > 0 && end_day <= 30 { end_day + 1 } else { 31 };
                            html! {
                                <div
                                    class={task_style.clone()}
                                    style={format!("grid-column-start: {}; grid-column-end: {}; grid-row-start: {}; background: {};", column_start, column_end, row_index, task.color)}
                                    onmousedown={on_mouse_down.reform(move |e: MouseEvent| (e, task_clone.clone()))}
                                    onmousemove={on_mouse_move.clone()}
                                    onmouseup={on_mouse_up.clone()}
                                >
                                    { task.name }
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
            </div>
        </>
    }
}