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
    let zoom_level = use_state(|| 50);
    let scroll_offset = use_state(|| 0);
    let selected_task = use_state(|| None::<Rc<Task>>);

    let add_task = {
        let tasks = tasks.clone();
        Callback::from(move |_| {
            let mut new_tasks = (*tasks).clone();
            let id = new_tasks.len() + 1;
            let base_date = NaiveDateTime::parse_from_str("2025-03-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
            new_tasks.push(Task {
                id,
                name: format!("Task {}", id),
                start_date: base_date + Duration::days(id as i64 * 3),
                end_date: base_date + Duration::days(id as i64 * 5),
                color: "#009688".to_string(),
            });
            tasks.set(new_tasks);
        })
    };

    let remove_task = {
        let tasks = tasks.clone();
        Callback::from(move |id: usize| {
            let new_tasks: Vec<Task> = (*tasks).clone().into_iter().filter(|task| task.id != id).collect();
            tasks.set(new_tasks);
        })
    };

    let edit_task = {
        let tasks = tasks.clone();
        let selected_task = selected_task.clone();
        Callback::from(move |(id, name, start, end): (usize, String, NaiveDateTime, NaiveDateTime)| {
            let new_tasks: Vec<Task> = (*tasks).clone().into_iter().map(|mut task| {
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

    html! {
        <>
            <button onclick={add_task}>{ "Add Task" }</button>
            <button onclick={Callback::from(move |_| {
                log::info!("tasks: {:?}", *tasks);
                log::info!("zoom_level: {:?}", *zoom_level);
            })}>{ "Debug" }</button>
            <button onclick={zoom_in}>{ "Zoom In" }</button>
            <button onclick={zoom_out}>{ "Zoom Out" }</button>
            <button onclick={scroll_left}>{ "Scroll Left" }</button>
            <button onclick={scroll_right}>{ "Scroll Right" }</button>
            <div class={classes!("gantt-container")} style={format!("width: {}%;", *zoom_level_clone)}>
                <div class={classes!(grid_style())} style={format!("transform: translateX(-{}px);", *scroll_offset)}>
                { for (*tasks_clone).iter().map(|task| {
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
        <div class={classes!(task_style())} style={format!("background: {};", task.color)}>
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
