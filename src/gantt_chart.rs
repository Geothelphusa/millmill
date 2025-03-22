use yew::prelude::*;
use yew_hooks::prelude::*;
use web_sys::{Element, MouseEvent};
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use chrono::{NaiveDate, Duration, Datelike, prelude::*};
use gloo_timers::callback::Timeout;
use gloo_utils::window;

// 仮のデータ構造と関数（実際のアプリケーションに合わせて実装してください）
#[derive(Clone, PartialEq)]
struct Task {
    task_id: String,
    start_date: NaiveDate,
    end_date: NaiveDate,
    // その他のタスクのプロパティ
}

#[derive(Clone, PartialEq)]
struct RowHeader {
    id: String,
    // その他の行ヘッダのプロパティ
}

fn use_get_row_headers() -> UseStateHandle<Vec<RowHeader>> {
    // 実際の行ヘッダデータを取得するロジックを実装
    use_state(|| vec![])
}

fn use_get_db_tasks() -> Vec<Task> {
    // 実際のDBからタスクデータを取得するロジックを実装
    vec![]
}

fn compute_map(
    tasks: Vec<Task>,
    fiscal_year_from: NaiveDate,
    fiscal_year_to: NaiveDate,
    row_headers: Vec<RowHeader>,
) -> (HashMap<String, (i32, i32)>, HashMap<String, i32>) {
    // タスクと行ヘッダの位置を計算するロジックを実装
    (HashMap::new(), HashMap::new())
}

fn get_tasks_by_row_header(tasks: Vec<Task>, row_headers: Vec<RowHeader>) -> HashMap<String, Vec<Task>> {
    // 行ヘッダごとにタスクを分割するロジックを実装
    HashMap::new()
}

fn update_task_temporary(task: Task) {
    // タスクの一時的な更新ロジックを実装
}

fn update_task(task: Vec<Task>) {
    // タスクの永続的な更新ロジックを実装
}

const ONE_DAY_WIDTH: i32 = 20;
const ROW_HEADER_HEIGHT: i32 = 30;
const ROW_HEADER_WIDTH: i32 = 100; // 仮の値

#[function_component(GanttChart)]
fn gantt_chart() -> Html {
    let fiscal_year = 2024;
    let fiscal_year_from = NaiveDate::from_ymd_opt(fiscal_year, 3, 1).unwrap();
    let fiscal_year_to = NaiveDate::from_ymd_opt(fiscal_year + 1, 2, 31).unwrap();

    let row_headers = use_get_row_headers();
    let db_tasks = use_get_db_tasks();

    let local_tasks = use_state(|| db_tasks);
    let drawing_tasks = use_state(|| (*local_tasks).clone());

    {
        let local_tasks = local_tasks.clone();
        let drawing_tasks = drawing_tasks.clone();
        use_effect_with_deps(move |_| {
            drawing_tasks.set((*local_tasks).clone());
            || ()
        }, local_tasks);
    }

    let (tasks_position_map, row_header_position_map) = {
        let local_tasks = (*local_tasks).clone();
        let row_headers = (*row_headers).clone();
        use_memo(move |(local_tasks, row_headers)| {
            compute_map(local_tasks, fiscal_year_from, fiscal_year_to, row_headers)
        }, (local_tasks, row_headers))
    };

    let tasks_by_row_header = {
        let drawing_tasks = (*drawing_tasks).clone();
        let row_headers = (*row_headers).clone();
        use_memo(move |(drawing_tasks, row_headers)| {
            get_tasks_by_row_header(drawing_tasks, row_headers)
        }, (drawing_tasks, row_headers))
    };

    let watch_element_ref: NodeRef = use_node_ref();
    let scroll_left = use_state(|| 0);

    let horizontal_scroll_to = {
        let watch_element_ref = watch_element_ref.clone();
        use_callback(move |left: i32, _| {
            if let Some(element) = watch_element_ref.cast::<Element>() {
                element.scroll_to_with_scroll_to_options(web_sys::ScrollToOptions::new().top(element.scroll_top() as f64).left(left as f64).behavior(web_sys::ScrollBehavior::Smooth));
                true
            } else {
                false
            }
        }, ())
    };

    let on_scroll = {
        let scroll_left = scroll_left.clone();
        let watch_element_ref = watch_element_ref.clone();
        use_throttle(move || {
            if let Some(element) = watch_element_ref.cast::<Element>() {
                scroll_left.set(element.scroll_left());
            }
        }, 20)
    };

    {
        let watch_element_ref = watch_element_ref.clone();
        let on_scroll = on_scroll.clone();
        use_effect_with_deps(move |_| {
            if let Some(element) = watch_element_ref.cast::<Element>() {
                let on_scroll_clone = on_scroll.clone();
                let closure = Closure::wrap(Box::new(move || on_scroll_clone()));
                element.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref()).unwrap();
                let observer = web_sys::ResizeObserver::new(closure.as_ref().unchecked_ref()).unwrap();
                observer.observe(&element);
                || {
                    observer.disconnect();
                    element.remove_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref()).unwrap();
                }
            } else {
                || ()
            }
        }, watch_element_ref);
    }

    let visible_range = {
        let scroll_left = *scroll_left;
        let watch_element_ref = watch_element_ref.clone();
        use_memo(move |scroll_left| {
            let relative_days_from = scroll_left / ONE_DAY_WIDTH;
            let relative_days_to = (scroll_left + watch_element_ref.cast::<Element>().map(|e| e.client_width()).unwrap_or(0)) / ONE_DAY_WIDTH;
            (fiscal_year_from + Duration::days(relative_days_from as i64), fiscal_year_from + Duration::days(relative_days_to as i64))
        }, scroll_left)
    };

    let from_date_ref = use_mut_ref(|| None::<NaiveDate>);

    let on_mouse_move_start = {
        let from_date_ref = from_date_ref.clone();
        use_callback(move |now_date: NaiveDate, _| {
            *from_date_ref.borrow_mut() = Some(now_date);
        }, ())
    };

    let on_mouse_move = {
        let drawing_tasks = drawing_tasks.clone();
        let from_date_ref = from_date_ref.clone();
        use_callback(move |(task, now_date): (Task, NaiveDate), _| {
            if let Some(from_date) = *from_date_ref.borrow() {
                let diff_days = (now_date - from_date).num_days();
                let mut new_task = task.clone();
                new_task.start_date = new_task.start_date + Duration::days(diff_days);
                new_task.end_date = new_task.end_date + Duration::days(diff_days);
                update_task_temporary(new_task);
            }
        }, ())
    };

    let on_mouse_move_end = {
      let drawing_tasks = drawing_tasks.clone();
      use_callback(move |task_id: String, _| {
          let new_task = (*drawing_tasks).iter().find(|task| task.task_id == task_id).cloned();
          if let Some(new_task) = new_task {
              update_task(vec![new_task]);
          }
      }, ())
  };

  let events_ref = use_mut_ref(|| Rc::new(RefCell::new(Events {
      on_mouse_move_start,
      on_mouse_move,
      on_mouse_move_end,
  })));

  let on_mouse_down_to_slide_task = {
      let events_ref = events_ref.clone();
      let watch_element_ref = watch_element_ref.clone();
      use_callback(move |(e, task): (MouseEvent, Task), _| {
          if let Some(target) = watch_element_ref.cast::<Element>() {
              let get_now_date = |e: MouseEvent| {
                  let rect = target.get_bounding_client_rect();
                  let left = rect.left() as i32;
                  let scroll_left = target.scroll_left();
                  let row_header_width = ROW_HEADER_WIDTH;
                  let client_x = e.client_x();
                  let days = (client_x - left + scroll_left - row_header_width) / ONE_DAY_WIDTH;
                  fiscal_year_from + Duration::days(days as i64)
              };

              let on_mouse_move = {
                  let events_ref = events_ref.clone();
                  use_throttle(move |e: MouseEvent| {
                      let events = events_ref.borrow();
                      (events.on_mouse_move)((task.clone(), get_now_date(e)));
                  }, 50)
              };

              let on_mouse_up = {
                  let events_ref = events_ref.clone();
                  use_callback(move |e: MouseEvent, _| {
                      let events = events_ref.borrow();
                      (events.on_mouse_move_end)(task.task_id.clone());
                      // イベントリスナーの削除
                      // ...
                  }, ())
              };

              // イベントリスナーの追加
              // ...
              (events_ref.borrow().on_mouse_move_start)(get_now_date(e));
          }
      }, ())
  };

  let visible_tasks = {
    let drawing_tasks = (*drawing_tasks).clone();
    let tasks_position_map = (*tasks_position_map).clone();
    use_memo(move |(drawing_tasks, tasks_position_map)| {
        drawing_tasks
            .iter()
            .filter(|task| {
                task.start_date < visible_range.1 && task.end_date > visible_range.0
            })
            .map(|task| {
                let task_position = tasks_position_map.get(&task.task_id).unwrap();
                let rel_days = (task.start_date - fiscal_year_from).num_days() as i32;
                let task_width = (task.end_date - task.start_date).num_days() as i32 * ONE_DAY_WIDTH;

                let style = yew::virtual_dom::AttrValue::V1(format!(
                    "position: absolute; top: {}px; left: {}px; width: {}px;",
                    task_position.1 * ROW_HEADER_HEIGHT,
                    rel_days * ONE_DAY_WIDTH,
                    task_width
                ));

                (style, task.clone())
            })
            .collect::<Vec<_>>()
    }, (drawing_tasks, tasks_position_map))
};

html! {
    <div ref={watch_element_ref}>
        {
            for (*tasks_by_row_header).iter().map(|(row_header_id, tasks)| {
                html! {
                    <div>
                        {row_header_id} // 行ヘッダの表示
                        {
                            for tasks.iter().map(|task| {
                                let style = visible_tasks.iter().find(|(_, t)| t.task_id == task.task_id).map(|(s, _)| s.clone());
                                if let Some(style) = style {
                                    html! {
                                        <div style={style} onmousedown={on_mouse_down_to_slide_task.reform(move |e: MouseEvent| (e, task.clone()))}>
                                            {task.task_id.clone()} // タスクの表示
                                        </div>
                                    }
                                } else {
                                    html! {}
                                }
                            })
                        }
                    </div>
                }
            })
        }
    </div>
}
}