use gpui::*;
use gpui::prelude::*;
use gpui_ui_components::{Button, button::{ButtonVariant, ButtonSize}};

// Import Application trait
use gpui::Application;

/// A simple todo item
#[derive(Clone, Debug)]
struct TodoItem {
    id: usize,
    text: String,
    completed: bool,
}

/// The main Todo App state
struct TodoApp {
    todos: Vec<TodoItem>,
    next_id: usize,
    input_text: SharedString,
}

impl TodoApp {
    fn new() -> Self {
        Self {
            todos: vec![
                TodoItem {
                    id: 0,
                    text: "Learn GPUI".to_string(),
                    completed: false,
                },
                TodoItem {
                    id: 1,
                    text: "Build a component library".to_string(),
                    completed: true,
                },
                TodoItem {
                    id: 2,
                    text: "Create awesome apps".to_string(),
                    completed: false,
                },
            ],
            next_id: 3,
            input_text: "".into(),
        }
    }

    fn toggle_todo(&mut self, id: usize) {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.completed = !todo.completed;
        }
    }

    fn add_todo(&mut self, text: String) {
        if !text.trim().is_empty() {
            self.todos.push(TodoItem {
                id: self.next_id,
                text,
                completed: false,
            });
            self.next_id += 1;
        }
    }

    fn remove_todo(&mut self, id: usize) {
        self.todos.retain(|t| t.id != id);
    }

    fn clear_completed(&mut self) {
        self.todos.retain(|t| !t.completed);
    }
}

impl Render for TodoApp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let active_count = self.todos.iter().filter(|t| !t.completed).count();
        let completed_count = self.todos.len() - active_count;

        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .size_full()
            .bg(rgb(0xf8fafc))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w(px(600.0))
                    .gap_4()
                    .p_6()
                    .child(
                        // Header
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(
                                div()
                                    .text_2xl()
                                    .text_color(rgb(0x1e293b))
                                    .child("Todo App")
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(0x64748b))
                                    .child(format!(
                                        "{} active, {} completed",
                                        active_count, completed_count
                                    ))
                            )
                    )
                    .child(
                        // Input area with Add button
                        div()
                            .flex()
                            .gap_2()
                            .child(
                                div()
                                    .flex_1()
                                    .px_3()
                                    .py_2()
                                    .bg(rgb(0xffffff))
                                    .border_1()
                                    .border_color(rgb(0xe2e8f0))
                                    .rounded(px(4.0))
                                    .text_color(rgb(0x1e293b))
                                    .child("Add a new todo...")
                            )
                            .child(
                                Button::new("Add Todo")
                                    .variant(ButtonVariant::Default)
                                    .size(ButtonSize::Medium)
                            )
                    )
                    .child(
                        // Todo list
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .children(self.todos.iter().map(|todo| {
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_3()
                                    .px_4()
                                    .py_3()
                                    .bg(rgb(0xffffff))
                                    .border_1()
                                    .border_color(rgb(0xe2e8f0))
                                    .rounded(px(4.0))
                                    .hover(|style| style.border_color(rgb(0x94a3b8)))
                                    .child(
                                        // Checkbox placeholder
                                        div()
                                            .size(px(20.0))
                                            .rounded(px(4.0))
                                            .border_1()
                                            .border_color(if todo.completed {
                                                rgb(0x3b82f6)
                                            } else {
                                                rgb(0xe2e8f0)
                                            })
                                            .bg(if todo.completed {
                                                rgb(0x3b82f6)
                                            } else {
                                                rgb(0xffffff)
                                            })
                                            .when(todo.completed, |style| {
                                                style.child(
                                                    div()
                                                        .absolute()
                                                        .top(px(3.0))
                                                        .left(px(6.0))
                                                        .w(px(6.0))
                                                        .h(px(10.0))
                                                        .text_color(rgb(0xffffff))
                                                        .child("✓")
                                                )
                                            })
                                    )
                                    .child(
                                        div()
                                            .flex_1()
                                            .text_color(if todo.completed {
                                                rgb(0x94a3b8)
                                            } else {
                                                rgb(0x1e293b)
                                            })
                                            .when(todo.completed, |style| {
                                                style.line_through()
                                            })
                                            .child(todo.text.clone())
                                    )
                                    .child(
                                        Button::new("Delete")
                                            .variant(ButtonVariant::Destructive)
                                            .size(ButtonSize::Small)
                                    )
                            }))
                    )
                    .child(
                        // Footer with action buttons
                        div()
                            .flex()
                            .gap_2()
                            .justify_between()
                            .child(
                                Button::new(format!("{} items left", active_count))
                                    .variant(ButtonVariant::Ghost)
                                    .size(ButtonSize::Small)
                            )
                            .when(completed_count > 0, |el| {
                                el.child(
                                    Button::new("Clear Completed")
                                        .variant(ButtonVariant::Outline)
                                        .size(ButtonSize::Small)
                                )
                            })
                    )
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(800.0), px(600.0)), cx);
        
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("Todo App - GPUI UI Example".into()),
                    appears_transparent: false,
                    traffic_light_position: None,
                }),
                ..Default::default()
            },
            |_window, cx| cx.new(|_cx| TodoApp::new()),
        )
        .unwrap();
        
        cx.activate(true);
    });
}

