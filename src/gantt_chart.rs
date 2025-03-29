use std::rc::Rc;
use chrono::{Duration, NaiveDateTime};
use stylist::yew::styled_component;
use wasm_bindgen::JsCast;
use web_sys::{WheelEvent, MouseEvent};
use implicit_clone::ImplicitClone;
use serde::{Serialize, Deserialize};
use serde_json;

use crate::styles::*;
use yew::prelude::*;
use tauri::invoke;
use yew::platform::spawn_local;

#[derive(Clone, PartialEq, Debug, ImplicitClone, Serialize, Deserialize)]
struct Task {
    id: usize,
    name: String,
    start_date: NaiveDateTime,
    end_date: NaiveDateTime,
    color: String,
    is_dragging: bool,
    drag_offset: i64,
    drag_start_x: f64,
}

#[derive(Clone, PartialEq, ImplicitClone)]
struct TaskFormData {
    name: String,
    start_date: String,
    end_date: String,
}

fn initial_tasks() -> Vec<Task> {
    let base_date = NaiveDateTime::parse_from_str("2025-03-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap_or_else(|e| {
        log::error!("Failed to parse base date: {}", e);
        NaiveDateTime::default()
    });
    vec![
        Task { 
            id: 1, 
            name: "Task 1".to_string(), 
            start_date: base_date, 
            end_date: base_date + Duration::days(5), 
            color: "#4CAF50".to_string(),
            is_dragging: false,
            drag_offset: 0,
            drag_start_x: 0.0
        },
        Task { 
            id: 2, 
            name: "Task 2".to_string(), 
            start_date: base_date + Duration::days(6), 
            end_date: base_date + Duration::days(9), 
            color: "#FF9800".to_string(),
            is_dragging: false,
            drag_offset: 0,
            drag_start_x: 0.0
        },
        Task { 
            id: 3, 
            name: "Task 3".to_string(), 
            start_date: base_date + Duration::days(10), 
            end_date: base_date + Duration::days(14), 
            color: "#673AB7".to_string(),
            is_dragging: false,
            drag_offset: 0,
            drag_start_x: 0.0
        },
    ]
}

#[styled_component(GanttChart)]
pub fn gantt_chart() -> Html {
    let tasks = use_state(|| {
        initial_tasks()
    });

    // タスクの変更を保存する
    let save_tasks = {
        let tasks = tasks.clone();
        Callback::from(move |_| {
            let tasks = (*tasks).clone();
            spawn_local(async move {
                let _ = invoke("save_tasks", tasks).await;
            });
        })
    };

    // タスクの更新時に保存を実行
    use_effect_with(
        (*tasks).clone(),
        move |tasks| {
            save_tasks.emit(());
            || ()
        },
    );

    let zoom_level = use_state(|| 50);
    let scroll_offset = use_state(|| 0);
    let selected_task = use_state(|| None::<Task>);
    let show_task_form = use_state(|| false);
    let show_floating_window = use_state(|| false);
    let floating_window_position = use_state(|| (0.0, 0.0));
    let editing_task = use_state(|| None::<Task>);
    let dragging_task = use_state(|| None::<usize>);
    let task_form_data = use_state(|| TaskFormData {
        name: String::new(),
        start_date: String::new(),
        end_date: String::new(),
    });

    let add_task = {
        let show_task_form = show_task_form.clone();
        Callback::from(move |_| {
            show_task_form.set(true);
        })
    };

    let submit_task = {
        let tasks = tasks.clone();
        let show_task_form = show_task_form.clone();
        let task_form_data = task_form_data.clone();
        Callback::from(move |_| {
            let base_date = NaiveDateTime::parse_from_str("2025-03-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap_or_else(|e| {
                log::error!("Failed to parse base date: {}", e);
                NaiveDateTime::default()
            });
            let start_date = NaiveDateTime::parse_from_str(&task_form_data.start_date, "%Y-%m-%dT%H:%M")
                .unwrap_or_else(|e| {
                    log::error!("Failed to parse start date: {}", e);
                    base_date
                });
            let end_date = NaiveDateTime::parse_from_str(&task_form_data.end_date, "%Y-%m-%dT%H:%M")
                .unwrap_or_else(|e| {
                    log::error!("Failed to parse end date: {}", e);
                    base_date + Duration::days(1)
                });

            let mut new_tasks = (*tasks).clone();
            let id = new_tasks.len() + 1;
            new_tasks.push(Task {
                id,
                name: task_form_data.name.clone(),
                start_date,
                end_date,
                color: "#009688".to_string(),
                is_dragging: false,
                drag_offset: 0,
                drag_start_x: 0.0
            });
            tasks.set(new_tasks);
            show_task_form.set(false);
            task_form_data.set(TaskFormData {
                name: String::new(),
                start_date: String::new(),
                end_date: String::new(),
            });
        })
    };

    let remove_task = {
        let tasks = tasks.clone();
        Callback::from(move |id: usize| {
            let new_tasks = (*tasks).clone().into_iter().filter(|task| task.id != id).collect();
            tasks.set(new_tasks);
        })
    };

    let edit_task = {
        let tasks = tasks.clone();
        let selected_task = selected_task.clone();
        Callback::from(move |(id, name, start, end): (usize, String, NaiveDateTime, NaiveDateTime)| {
            let tasks = tasks.clone();
            let name = name.clone();
            let mut new_tasks = (*tasks).clone();
            for task in &mut new_tasks {
                if task.id == id {
                    task.name = name.clone();
                    task.start_date = start;
                    task.end_date = end;
                    break;
                }
            }
            tasks.set(new_tasks);
            selected_task.set(None);
        })
    };

    let on_wheel = {
        let zoom_level = zoom_level.clone();
        let scroll_offset = scroll_offset.clone();
        Callback::from(move |e: WheelEvent| {
            if e.ctrl_key() {
                // Ctrl + ホイールでズーム
                let delta = e.delta_y();
                if delta < 0.0 {
                    zoom_level.set((*zoom_level - 5).max(20));
                } else {
                    zoom_level.set((*zoom_level + 5).min(100));
                }
            } else {
                // 通常のホイールでスクロール
                let delta = e.delta_x();
                scroll_offset.set((*scroll_offset + delta as i32).max(0));
            }
        })
    };

    let on_input_name = {
        let edit_task = edit_task.clone();
        Callback::from(move |(id, name, start, end): (usize, String, NaiveDateTime, NaiveDateTime)| {
            edit_task.emit((id, name, start, end));
        })
    };

    let on_mouse_down = {
        let tasks = tasks.clone();
        let dragging_task = dragging_task.clone();
        Callback::from(move |e: MouseEvent| {
            if let Ok(Some(element)) = e.target()
                .unwrap()
                .unchecked_into::<web_sys::HtmlElement>()
                .closest("[data-task-id]")
            {
                if let Some(task_id_str) = element.get_attribute("data-task-id") {
                    if let Ok(task_id) = task_id_str.parse::<usize>() {
                        dragging_task.set(Some(task_id));
                        let mut new_tasks = (*tasks).clone();
                        if let Some(task) = new_tasks.iter_mut().find(|t| t.id == task_id) {
                            task.is_dragging = true;
                            task.drag_offset = 0;
                            task.drag_start_x = e.client_x() as f64;
                        }
                        tasks.set(new_tasks);
                    }
                }
            }
        })
    };

    let on_mouse_up = {
        let tasks = tasks.clone();
        let dragging_task = dragging_task.clone();
        Callback::from(move |_| {
            if let Some(task_id) = *dragging_task {
                let mut new_tasks = (*tasks).clone();
                if let Some(task) = new_tasks.iter_mut().find(|t| t.id == task_id) {
                    let base_date = NaiveDateTime::parse_from_str("2025-03-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
                    let new_start = base_date + Duration::days(task.drag_offset);
                    let duration = task.end_date - task.start_date;
                    task.start_date = new_start;
                    task.end_date = new_start + duration;
                    task.is_dragging = false;
                    task.drag_offset = 0;
                    task.drag_start_x = 0.0;
                }
                tasks.set(new_tasks);
            }
            dragging_task.set(None);
        })
    };

    let on_mouse_move = {
        let tasks = tasks.clone();
        let dragging_task = dragging_task.clone();
        Callback::from(move |e: MouseEvent| {
            if let Some(task_id) = *dragging_task {
                let mut new_tasks = (*tasks).clone();
                if let Some(task) = new_tasks.iter_mut().find(|t| t.id == task_id) {
                    let delta_x = e.client_x() as f64 - task.drag_start_x;
                    let days_delta = (delta_x / 100.0).round() as i64;
                    task.drag_offset = days_delta;
                }
                tasks.set(new_tasks);
            }
        })
    };

    let on_task_click = {
        let tasks = tasks.clone();
        let show_floating_window = show_floating_window.clone();
        let floating_window_position = floating_window_position.clone();
        let editing_task = editing_task.clone();
        Callback::from(move |e: MouseEvent| {
            if !e.ctrl_key() {
                let window_width = 320.0; // フローティングウィンドウの推定幅
                let window_height = 300.0; // フローティングウィンドウの推定高さ
                let viewport_width = web_sys::window().unwrap().inner_width().unwrap().as_f64().unwrap();
                let viewport_height = web_sys::window().unwrap().inner_height().unwrap().as_f64().unwrap();
                
                let mut x = e.client_x() as f64;
                let mut y = e.client_y() as f64;
                
                // 右端に収まるように調整
                if x + window_width > viewport_width {
                    x = viewport_width - window_width;
                }
                
                // 下端に収まるように調整
                if y + window_height > viewport_height {
                    y = viewport_height - window_height;
                }
                
                // 左端と上端に収まるように調整
                x = x.max(0.0);
                y = y.max(0.0);
                
                show_floating_window.set(true);
                floating_window_position.set((x, y));
                if let Ok(Some(element)) = e.target()
                    .unwrap()
                    .unchecked_into::<web_sys::HtmlElement>()
                    .closest("[data-task-id]")
                {
                    if let Some(task_id_str) = element.get_attribute("data-task-id") {
                        if let Ok(task_id) = task_id_str.parse::<usize>() {
                            if let Some(task) = (*tasks).iter().find(|t| t.id == task_id) {
                                editing_task.set(Some(task.implicit_clone()));
                            }
                        }
                    }
                }
            }
        })
    };

    let update_task = {
        let tasks = tasks.clone();
        let editing_task = editing_task.clone();
        let show_floating_window = show_floating_window.clone();
        Callback::from(move |(name, start_date, end_date): (String, String, String)| {
            if let Some(task) = (*editing_task).clone() {
                let mut new_tasks = (*tasks).clone();
                if let Some(task_to_update) = new_tasks.iter_mut().find(|t| t.id == task.id) {
                    task_to_update.name = name.clone();
                    task_to_update.start_date = NaiveDateTime::parse_from_str(&start_date, "%Y-%m-%dT%H:%M")
                        .unwrap_or(task_to_update.start_date);
                    task_to_update.end_date = NaiveDateTime::parse_from_str(&end_date, "%Y-%m-%dT%H:%M")
                        .unwrap_or(task_to_update.end_date);
                }
                tasks.set(new_tasks);
            }
            show_floating_window.set(false);
            editing_task.set(None);
        })
    };

    let _zoom_level_clone = zoom_level.clone();
    let task_form_data_name = task_form_data.clone();
    let task_form_data_start = task_form_data.clone();
    let task_form_data_end = task_form_data.clone();

    html! {
        <>
            <div style="display: flex; justify-content: space-between; align-items: center;">
                <div>
                    <button onclick={add_task}>{ "Add Task" }</button>
                </div>
                <div>
                </div>
            </div>
            if *show_task_form {
                <div class={classes!("task-form-overlay")}>
                    <div class={classes!("task-form")}>
                        <h3>{ "Add New Task" }</h3>
                        <div>
                            <label>{ "Task Name:" }</label>
                            <input
                                type="text"
                                value={task_form_data_name.name.clone()}
                                oninput={Callback::from(move |e: InputEvent| {
                                    let input = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                    task_form_data_name.set(TaskFormData {
                                        name: input.value(),
                                        start_date: task_form_data_name.start_date.clone(),
                                        end_date: task_form_data_name.end_date.clone(),
                                    });
                                })}
                            />
                        </div>
                        <div>
                            <label>{ "Start Date:" }</label>
                            <input
                                type="datetime-local"
                                value={task_form_data_start.start_date.clone()}
                                oninput={Callback::from(move |e: InputEvent| {
                                    let input = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                    task_form_data_start.set(TaskFormData {
                                        name: task_form_data_start.name.clone(),
                                        start_date: input.value(),
                                        end_date: task_form_data_start.end_date.clone(),
                                    });
                                })}
                            />
                        </div>
                        <div>
                            <label>{ "End Date:" }</label>
                            <input
                                type="datetime-local"
                                value={task_form_data_end.end_date.clone()}
                                oninput={Callback::from(move |e: InputEvent| {
                                    let input = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                    task_form_data_end.set(TaskFormData {
                                        name: task_form_data_end.name.clone(),
                                        start_date: task_form_data_end.start_date.clone(),
                                        end_date: input.value(),
                                    });
                                })}
                            />
                        </div>
                        <div>
                            <button onclick={submit_task}>{ "Submit" }</button>
                            <button onclick={Callback::from(move |_| show_task_form.set(false))}>{ "Cancel" }</button>
                        </div>
                    </div>
                </div>
            }
            {if *show_floating_window {
                html! {
                    <div class={classes!("floating-window")} style={format!("left: {}px; top: {}px;", floating_window_position.0, floating_window_position.1)}>
                        <h3>{ "Edit Task" }</h3>
                        {
                            if let Some(task) = (*editing_task).clone() {
                                let editing_task = editing_task.clone();
                                let update_task = update_task.clone();
                                let show_floating_window = show_floating_window.clone();
                                let task = task.clone();
                                let editing_task_clone = editing_task.clone();
                                let task_clone = task.clone();
                                let task_clone2 = task.clone();
                                let editing_task_clone2 = editing_task_clone.clone();
                                let editing_task_clone2_for_start = editing_task_clone2.clone();
                                let task_clone2_for_start = task_clone2.clone();
                                let editing_task_clone2_for_end = editing_task_clone2.clone();
                                let task_clone2_for_end = task_clone2.clone();
                                html! {
                                    <div>
                                        <input
                                            type="text"
                                            value={task.name.clone()}
                                            oninput={Callback::from(move |e: InputEvent| {
                                                let input = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                                let mut new_task = task.clone();
                                                new_task.name = input.value();
                                                editing_task_clone2.set(Some(new_task));
                                            })}
                                        />
                                        <input
                                            type="datetime-local"
                                            value={task_clone.start_date.format("%Y-%m-%dT%H:%M").to_string()}
                                            oninput={Callback::from(move |e: InputEvent| {
                                                let input = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                                let mut new_task = task_clone2_for_start.clone();
                                                if let Ok(date) = NaiveDateTime::parse_from_str(&input.value(), "%Y-%m-%dT%H:%M") {
                                                    new_task.start_date = date;
                                                    editing_task_clone2_for_start.set(Some(new_task));
                                                } else {
                                                    log::error!("Failed to parse start date");
                                                }
                                            })}
                                        />
                                        <input
                                            type="datetime-local"
                                            value={task_clone.end_date.format("%Y-%m-%dT%H:%M").to_string()}
                                            oninput={Callback::from(move |e: InputEvent| {
                                                let input = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                                let mut new_task = task_clone2_for_end.clone();
                                                if let Ok(date) = NaiveDateTime::parse_from_str(&input.value(), "%Y-%m-%dT%H:%M") {
                                                    new_task.end_date = date;
                                                    editing_task_clone2_for_end.set(Some(new_task));
                                                } else {
                                                    log::error!("Failed to parse end date");
                                                }
                                            })}
                                        />
                                        <button onclick={move |_| {
                                            if let Some(task) = (*editing_task).clone() {
                                                update_task.emit((
                                                    task.name.clone(),
                                                    task.start_date.format("%Y-%m-%dT%H:%M").to_string(),
                                                    task.end_date.format("%Y-%m-%dT%H:%M").to_string(),
                                                ));
                                            }
                                        }}>{ "Save" }</button>
                                        <button onclick={Callback::from(move |_| {
                                            show_floating_window.set(false);
                                            editing_task_clone.set(None);
                                        })}>{ "Cancel" }</button>
                                    </div>
                                }
                            } else {
                                html! {}
                            }
                        }
                    </div>
                }
            } else {
                html! {}
            }}
            <div 
                class={classes!("gantt-container", dropdown_styles())} 
                style={format!("width: 100%; overflow-x: auto; background-color: #ffffff;")}
                onwheel={on_wheel}
                onmousemove={on_mouse_move}
                onmouseup={on_mouse_up}
            > 
                <div style={format!("display: flex; flex-direction: column; transform: translateX(-{}px);", *scroll_offset)}>
                    <div class="grid-lines" style="position: absolute; top: 0; left: 0; right: 0; bottom: 0; pointer-events: none;">
                        { for (0..30).map(|i| html! {
                            <div style={format!(
                                "position: absolute; left: {}px; top: 0; bottom: 0; width: 1px; background-color: #e0e0e0;",
                                i * 100
                            )} />
                        })}
                    </div>
                    { for (*tasks).iter().map(|task| {
                            let remove_task = remove_task.clone();
                            let on_input_name = on_input_name.clone();
                            let on_mouse_down = on_mouse_down.clone();
                            let on_click = on_task_click.clone();
                            html! {
                                <TaskView 
                                    task={task.clone()} 
                                    remove_task={remove_task} 
                                    on_input_name={on_input_name}
                                    on_mouse_down={on_mouse_down}
                                    on_click={on_click}
                                />
                            }
                        }) }
                </div>
            </div>
        </>
    }
}

#[derive(Properties, PartialEq)]
struct TaskViewProps {
    task: Task,
    remove_task: Callback<usize>,
    on_input_name: Callback<(usize, String, NaiveDateTime, NaiveDateTime)>,
    on_mouse_down: Callback<MouseEvent>,
    on_click: Callback<MouseEvent>,
}

#[function_component(TaskView)]
fn task_view(props: &TaskViewProps) -> Html {
    let task = &props.task;
    let remove_task = props.remove_task.clone();
    let on_input_name = props.on_input_name.clone();
    let on_mouse_down = props.on_mouse_down.clone();
    let on_click = props.on_click.clone();
    let task_id = task.id;
    let task_color = &task.color;
    let task_name = &task.name;
    let task_start_date = task.start_date;
    let task_end_date = task.end_date;
    let base_date = NaiveDateTime::parse_from_str("2025-03-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap_or_else(|e| {
        log::error!("Failed to parse base date: {}", e);
        NaiveDateTime::default()
    });
    let start_offset = (task_start_date - base_date).num_days() * 100 + if task.is_dragging { task.drag_offset * 100 } else { 0 };
    let duration = (task_end_date - task_start_date).num_days() * 100;
    
    html! {
        <div style={format!("position: relative; height: 30px;")}>
            <div 
                data-task-id={task_id.to_string()}
                style={format!(
                    "position: absolute; left: {}px; width: {}px; background: {}; height: 30px; 
                    border: 1px solid black; border-radius: 5px; display: flex; align-items: center; 
                    justify-content: space-between; padding: 0 10px; color: white; font-weight: bold;
                    cursor: move; {}",
                    start_offset, duration, task_color,
                    if task.is_dragging {
                        "transition: none;"
                    } else {
                        "transition: left 0.1s ease-out;"
                    }
                )}
                onmousedown={on_mouse_down}
                onclick={on_click}
            >
                <span>{task_name}</span>
                <button 
                    onclick={remove_task.reform(move |_| task_id)}
                    style="background: none; border: none; color: white; cursor: pointer; padding: 0 5px;"
                >
                    {"×"}
                </button>
            </div>
        </div>
    }
}