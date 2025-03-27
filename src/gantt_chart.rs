use std::rc::Rc;
use chrono::{Duration, NaiveDateTime};
use stylist::yew::styled_component;
use wasm_bindgen::JsCast;
use web_sys::{WheelEvent, MouseEvent};

use crate::styles::*;
use yew::prelude::*;

#[derive(Clone, PartialEq, Debug)]
struct Task {
    id: usize,
    name: String,
    start_date: NaiveDateTime,
    end_date: NaiveDateTime,
    color: String,
    is_dragging: bool,
    drag_offset: i64,
}

#[derive(Clone, PartialEq)]
struct TaskFormData {
    name: String,
    start_date: String,
    end_date: String,
}

fn initial_tasks() -> Vec<Task> {
    let base_date = NaiveDateTime::parse_from_str("2025-03-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    vec![
        Task { 
            id: 1, 
            name: "Task 1".to_string(), 
            start_date: base_date, 
            end_date: base_date + Duration::days(5), 
            color: "#4CAF50".to_string(),
            is_dragging: false,
            drag_offset: 0
        },
        Task { 
            id: 2, 
            name: "Task 2".to_string(), 
            start_date: base_date + Duration::days(6), 
            end_date: base_date + Duration::days(9), 
            color: "#FF9800".to_string(),
            is_dragging: false,
            drag_offset: 0
        },
        Task { 
            id: 3, 
            name: "Task 3".to_string(), 
            start_date: base_date + Duration::days(10), 
            end_date: base_date + Duration::days(14), 
            color: "#673AB7".to_string(),
            is_dragging: false,
            drag_offset: 0
        },
    ]
}

#[styled_component(GanttChart)]
pub fn gantt_chart() -> Html {
    let tasks = use_state(initial_tasks);
    let zoom_level = use_state(|| 50);
    let scroll_offset = use_state(|| 0);
    let selected_task = use_state(|| None::<Rc<Task>>);
    let show_task_form = use_state(|| false);
    let dragging_task = use_state(|| None::<usize>);
    let task_form_data = use_state(|| TaskFormData {
        name: String::new(),
        start_date: String::new(),
        end_date: String::new(),
    });

    let add_task = {
        let _tasks = tasks.clone();
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
            if let (Ok(start_date), Ok(end_date)) = (
                NaiveDateTime::parse_from_str(&task_form_data.start_date, "%Y-%m-%d %H:%M:%S"),
                NaiveDateTime::parse_from_str(&task_form_data.end_date, "%Y-%m-%d %H:%M:%S")
            ) {
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
                });
                tasks.set(new_tasks);
                show_task_form.set(false);
                task_form_data.set(TaskFormData {
                    name: String::new(),
                    start_date: String::new(),
                    end_date: String::new(),
                });
            }
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
            let new_tasks = (*tasks).clone().into_iter().map(|mut task| {
                if task.id == id {
                    task.name = name.clone();
                    task.start_date = start;
                    task.end_date = end;
                }
                task
            }).collect();
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
        Callback::from(move |task_id: usize| {
            dragging_task.set(Some(task_id));
            let mut new_tasks = (*tasks).clone();
            if let Some(task) = new_tasks.iter_mut().find(|t| t.id == task_id) {
                task.is_dragging = true;
                task.drag_offset = 0;
            }
            tasks.set(new_tasks);
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
                let rect = e.target().unwrap().unchecked_into::<web_sys::HtmlElement>().get_bounding_client_rect();
                let x = e.client_x() as f64 - rect.left();
                let days_offset = (x / 100.0).round() as i64;
                
                let mut new_tasks = (*tasks).clone();
                if let Some(task) = new_tasks.iter_mut().find(|t| t.id == task_id) {
                    task.drag_offset = days_offset;
                }
                tasks.set(new_tasks);
            }
        })
    };

    let _zoom_level_clone = zoom_level.clone();
    let tasks_clone = tasks.clone();
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
                    { for tasks_clone.iter().map(|task| {
                            let remove_task = remove_task.clone();
                            let on_input_name = on_input_name.clone();
                            let on_mouse_down = on_mouse_down.clone();
                            html! {
                                <TaskView 
                                    task={Rc::new(task.clone())} 
                                    remove_task={remove_task} 
                                    on_input_name={on_input_name}
                                    on_mouse_down={on_mouse_down}
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
    task: Rc<Task>,
    remove_task: Callback<usize>,
    on_input_name: Callback<(usize, String, NaiveDateTime, NaiveDateTime)>,
    on_mouse_down: Callback<usize>,
}

#[function_component(TaskView)]
fn task_view(props: &TaskViewProps) -> Html {
    let task = props.task.clone();
    let remove_task = props.remove_task.clone();
    let on_input_name = props.on_input_name.clone();
    let on_mouse_down = props.on_mouse_down.clone();
    let task_id = task.id;
    let task_color = task.color.clone();
    let task_name = task.name.clone();
    let task_start_date = task.start_date;
    let task_end_date = task.end_date;
    let base_date = NaiveDateTime::parse_from_str("2025-03-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let start_offset = (task_start_date - base_date).num_days() * 100 + if task.is_dragging { task.drag_offset * 100 } else { 0 };
    let duration = (task_end_date - task_start_date).num_days() * 100;
    
    html! {
        <div style={format!("display: flex; align-items: center; margin: 5px 0;")}>
            <div style={format!("width: 200px;")}>
                <input
                    value={task_name.clone()}
                    oninput={Callback::from(move |e: InputEvent| {
                        let input = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                        on_input_name.emit((task_id, input.value(), task_start_date, task_end_date));
                    })} />
            </div>
            <div style={format!("position: relative; flex-grow: 1; height: 30px;")}>
                <div 
                    style={format!(
                        "position: absolute; left: {}px; width: {}px; background: {}; height: 30px; 
                        border: 1px solid black; border-radius: 5px; display: flex; align-items: center; 
                        justify-content: space-between; padding: 0 10px; color: white; font-weight: bold;
                        cursor: move; transition: left 0.1s ease-out;",
                        start_offset, duration, task_color
                    )}
                    onmousedown={on_mouse_down.reform(move |_| task_id)}
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
        </div>
    }
}