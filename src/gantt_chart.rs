use std::rc::Rc;
use chrono::{Duration, NaiveDateTime};
use stylist::yew::styled_component;
use wasm_bindgen::JsCast;

use crate::styles::*;
use yew::prelude::*;

#[derive(Clone, PartialEq, Debug)]
struct Task {
    id: usize,
    name: String,
    start_date: NaiveDateTime,
    end_date: NaiveDateTime,
    color: String,
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
        Task { id: 1, name: "Task 1".to_string(), start_date: base_date, end_date: base_date + Duration::days(5), color: "#4CAF50".to_string() },
        Task { id: 2, name: "Task 2".to_string(), start_date: base_date + Duration::days(6), end_date: base_date + Duration::days(9), color: "#FF9800".to_string() },
        Task { id: 3, name: "Task 3".to_string(), start_date: base_date + Duration::days(10), end_date: base_date + Duration::days(14), color: "#673AB7".to_string() },
    ]
}

#[styled_component(GanttChart)]
pub fn gantt_chart() -> Html {
    let tasks = use_state(initial_tasks);
    let zoom_level = use_state(|| 50); // ズームレベルを50%に設定
    let scroll_offset = use_state(|| 0);
    let selected_task = use_state(|| None::<Rc<Task>>);
    let show_task_form = use_state(|| false);
    let task_form_data = use_state(|| TaskFormData {
        name: String::new(),
        start_date: String::new(),
        end_date: String::new(),
    });

    let add_task = {
        let tasks = tasks.clone();
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

    let zoom_in = {
        let zoom_level = zoom_level.clone();
        Callback::from(move |_| zoom_level.set((*zoom_level - 10).max(20)))
    };

    let zoom_out = {
        let zoom_level = zoom_level.clone();
        Callback::from(move |_| zoom_level.set((*zoom_level + 10).min(100)))
    };

    let scroll_left = {
        let scroll_offset = scroll_offset.clone();
        Callback::from(move |_| scroll_offset.set((*scroll_offset - 50).max(0)))
    };

    let scroll_right = {
        let scroll_offset = scroll_offset.clone();
        Callback::from(move |_| scroll_offset.set(*scroll_offset + 50))
    };

    let on_input_name = {
        let edit_task = edit_task.clone();
        Callback::from(move |(id, name, start, end): (usize, String, NaiveDateTime, NaiveDateTime)| {
            edit_task.emit((id, name, start, end));
        })
    };

    let zoom_level_clone = zoom_level.clone();
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
                    <button onclick={zoom_in}>{ "Zoom In" }</button>
                    <button onclick={zoom_out}>{ "Zoom Out" }</button>
                    <button onclick={scroll_left}>{ "Scroll Left" }</button>
                    <button onclick={scroll_right}>{ "Scroll Right" }</button>
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
            <div class={classes!("gantt-container")} style={format!("width: 100%; overflow-x: auto;")}> 
                <div class={classes!(grid_style())} style={format!("transform: translateX(-{}px); grid-template-columns: repeat(30, {}px);", *scroll_offset, *zoom_level_clone)}>
                    { for tasks_clone.iter().map(|task| {
                            let remove_task = remove_task.clone();
                            let on_input_name = on_input_name.clone();
                            html! {
                                <TaskView task={Rc::new(task.clone())} remove_task={remove_task} on_input_name={on_input_name} />
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
}

#[function_component(TaskView)]
fn task_view(props: &TaskViewProps) -> Html {
    let task = props.task.clone();
    let remove_task = props.remove_task.clone();
    let on_input_name = props.on_input_name.clone();
    let task_id = task.id;
    html! {
        <div class={classes!(task_style())} style={format!("background: {}; height: 30px; border: 1px solid black; border-radius: 5px;", task.color)}>
            <input
                value={task.name.clone()}
                oninput={Callback::from(move |e: InputEvent| {
                    let input = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                    on_input_name.emit((task.id, input.value(), task.start_date, task.end_date));
                })} />
                <button onclick={remove_task.reform(move |_| task_id)}>{ "Delete" }</button>
        </div>
    }
}