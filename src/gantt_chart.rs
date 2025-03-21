use yew::prelude::*;
use wasm_bindgen::JsCast;
use stylist::style;

#[derive(Clone, PartialEq)]
struct Task {
    name: &'static str,
    start: usize, // 開始日 (0 = 1日目)
    duration: usize, // 期間 (日数)
    color: &'static str, // 背景色
}

#[function_component(GanttChart)]
pub fn gantt_chart() -> Html {
    let tasks = vec![
        Task { name: "", start: 0, duration: 5, color: "#333" },
        Task { name: "", start: 6, duration: 3, color: "#333" },
        Task { name: "", start: 10, duration: 4, color: "#333" },
    ];

    let grid_style = style!(
        "
        display: grid;
        grid-template-columns: repeat(30, 30px);
        grid-template-rows: repeat(5, 40px);
        gap: 2px;
        background: #eee;
        padding: 10px;
        border-radius: 5px;
        "
    ).unwrap();

    html! {
        <div>
            <h2>{"GanttChart"}</h2>
            <div class={grid_style}>
                { for (0..5).map(|_row| html! {
                    { for (0..30).map(|_col| html! {
                        <div style="width: 30px; height: 40px; background: white; border: 1px solid #ddd;"></div>
                    }) }
                }) }
                
                { for tasks.iter().enumerate().map(|(i, task)| html! {
                    <div 
                        style={format!(
                            "grid-column-start: {}; grid-column-end: {}; grid-row-start: {}; background: {}; color: white; text-align: center; padding: 5px;",
                            task.start + 1,
                            task.start + task.duration + 1,
                            i + 1,
                            task.color
                        )}
                    >
                        { task.name }
                    </div>
                }) }
            </div>
        </div>
    }
}