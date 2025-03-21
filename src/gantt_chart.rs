use yew::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use wasm_bindgen::JsCast;

#[derive(Clone)]
struct Task {
    name: &'static str,
    start: usize,  // 開始日 (例: 0 = 1日目)
    duration: usize, // 期間 (日数)
}

#[function_component(GanttChart)]
pub fn gantt_chart() -> Html {
    let canvas_ref = use_node_ref();

    let tasks = vec![
        Task { name: "Task A", start: 0, duration: 5 },
        Task { name: "Task B", start: 3, duration: 7 },
        Task { name: "Task C", start: 8, duration: 4 },
    ];

    {
        let canvas_ref = canvas_ref.clone();
        let tasks = tasks.clone();
        
        use_effect(move || {
            if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {                
                if let Ok(Some(context)) = canvas.get_context("2d") {
                    if let Ok(context) = context.dyn_into::<CanvasRenderingContext2d>() {
                        context.clear_rect(0.0, 0.0, 400.0, 200.0);

                        for (i, task) in tasks.iter().enumerate() {
                            let y = 30.0 + i as f64 * 40.0;
                            let x = task.start as f64 * 20.0;
                            let width = task.duration as f64 * 20.0;

                            context.set_fill_style(&"#3498db".into());
                            context.fill_rect(x, y, width, 30.0);

                            context.set_fill_style(&"black".into());
                            context.set_font("14px Arial");
                            context.fill_text(task.name, x + 5.0, y + 20.0).ok();
                        }
                    }
                }
            }
            || ()
        });
    }

    html! {
        <div>
            <canvas ref={canvas_ref} width="400" height="200"></canvas>
        </div>
    }
}
