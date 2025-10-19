use gpui::*;
use gpui::prelude::*;
use gpui_ui_components::{
    Badge, BadgeSize, BadgeVariant, Button, ButtonVariant, ButtonSize,
    Card, CardContent, CardFooter, CardHeader, CardVariant,
    Checkbox, CheckboxSize, ToggleState,
    TextInput, TextInputSize, TextInputVariant,
};
use gpui_ui_components::text_input::text_input_actions;
use gpui::Application;
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_TODO_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Debug)]
struct TodoItem {
    id: usize,
    text: SharedString,
    completed: bool,
}

impl TodoItem {
    fn new(text: impl Into<SharedString>) -> Self {
        Self {
            id: NEXT_TODO_ID.fetch_add(1, Ordering::SeqCst),
            text: text.into(),
            completed: false,
        }
    }
}

struct TodoApp {
    todos: Vec<TodoItem>,
    show_completed: bool,
    input_text: Entity<TextInput>,
}

impl TodoApp {
    fn new(cx: &mut Context<Self>) -> Self {
        let app_entity = cx.entity();
        let input_text = cx.new(|cx| {
            TextInput::new("todo-input", cx)
                .placeholder("Add a new todo... (Press Enter to add)")
                .size(TextInputSize::Medium)
                .variant(TextInputVariant::Default)
                .on_submit(move |text, _window, cx| {
                    app_entity.update(cx, |this, cx| {
                        this.add_todo(text.to_string());
                        cx.notify();
                    })
                })
        });
        
        Self {
            todos: vec![
                TodoItem::new("Learn GPUI"),
                TodoItem::new("Build a component library"),
                TodoItem::new("Make a todo app"),
            ],
            show_completed: true,
            input_text,
        }
    }

    fn toggle_todo(&mut self, id: usize) {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.completed = !todo.completed;
        }
    }

    fn add_todo(&mut self, text: String) {
        if !text.trim().is_empty() {
            self.todos.push(TodoItem::new(text));
        }
    }

    fn remove_todo(&mut self, id: usize) {
        self.todos.retain(|t| t.id != id);
    }

    fn clear_completed(&mut self) {
        self.todos.retain(|t| !t.completed);
    }

    fn toggle_show_completed(&mut self) {
        self.show_completed = !self.show_completed;
    }
}

impl Render for TodoApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let active_count = self.todos.iter().filter(|t| !t.completed).count();
        let completed_count = self.todos.iter().filter(|t| t.completed).count();

        let filtered_todos: Vec<TodoItem> = if self.show_completed {
            self.todos.clone()
        } else {
            self.todos.iter().filter(|t| !t.completed).cloned().collect()
        };

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0xf8fafc)) // slate-50
            .items_center()
            .justify_center()
            .child(
                Card::new()
                    .variant(CardVariant::Elevated)
                    .child(
                        CardHeader::new()
                            .title("Todo App")
                            .description("A simple todo application built with GPUI and gpui-ui components.")
                    )
                    .child(
                        CardContent::new()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_4()
                                    .child(
                                        div()
                                            .flex()
                                            .gap_2()
                                            .child(
                                                div()
                                                    .flex_1()
                                                    .child(self.input_text.clone())
                                            )
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_2()
                                            .children(filtered_todos.into_iter().map(|todo| {
                                                let todo_id = todo.id;
                                                let checkbox_id = ElementId::Name(format!("todo-checkbox-{}", todo_id).into());
                                                let delete_btn_id = ElementId::Name(format!("todo-delete-{}", todo_id).into());
                                                
                                                div()
                                                    .flex()
                                                    .items_center()
                                                    .justify_between()
                                                    .p_2()
                                                    .rounded(px(4.0))
                                                    .hover(|s| s.bg(rgb(0xf1f5f9))) // slate-100
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .items_center()
                                                            .gap_2()
                                                            .child(
                                                                Checkbox::new(checkbox_id, ToggleState::from(todo.completed))
                                                                    .size(CheckboxSize::Medium)
                                                                    .on_click(cx.listener(move |this, _state, _window, cx| {
                                                                        this.toggle_todo(todo_id);
                                                                        cx.notify();
                                                                    }))
                                                            )
                                                            .child(
                                                                div()
                                                                    .flex_1()
                                                                    .text_color(if todo.completed {
                                                                        rgb(0x94a3b8) // slate-400
                                                                    } else {
                                                                        rgb(0x0f172a) // slate-900
                                                                    })
                                                                    .when(todo.completed, |d| {
                                                                        d.line_through()
                                                                    })
                                                                    .child(todo.text.clone())
                                                            )
                                                    )
                                                    .child(
                                                        Button::new(delete_btn_id, "Delete")
                                                            .variant(ButtonVariant::Destructive)
                                                            .size(ButtonSize::Small)
                                                            .on_click(cx.listener(move |this, _event, _window, cx| {
                                                                this.remove_todo(todo_id);
                                                                cx.notify();
                                                            }))
                                                    )
                                            }))
                                    )
                            )
                    )
                    .child(
                        CardFooter::new()
                            .child(
                                div()
                                    .flex()
                                    .w_full()
                                    .justify_between()
                                    .items_center()
                                    .child(
                                        Badge::new(format!("{} items left", active_count))
                                            .variant(BadgeVariant::Outline)
                                            .size(BadgeSize::Small)
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .gap_2()
                                            .child(
                                                Checkbox::new("show-completed-checkbox", ToggleState::from(self.show_completed))
                                                    .label("Show completed")
                                                    .on_click(cx.listener(|this, _state, _window, cx| {
                                                        this.toggle_show_completed();
                                                        cx.notify();
                                                    }))
                                            )
                                            .when(completed_count > 0, |d| {
                                                d.child(
                                                    Button::new("clear-completed-btn", "Clear Completed")
                                                        .variant(ButtonVariant::Ghost)
                                                        .size(ButtonSize::Small)
                                                        .on_click(cx.listener(|this, _event, _window, cx| {
                                                            this.clear_completed();
                                                            cx.notify();
                                                        }))
                                                )
                                            })
                                    )
                            )
                    )
            )
    }
}

fn main() {
    Application::new().run(move |cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(800.0), px(600.0)), cx);

        // Bind keys for text input
        use text_input_actions::*;
        cx.bind_keys([
            KeyBinding::new("backspace", Backspace, None),
            KeyBinding::new("delete", Delete, None),
            KeyBinding::new("left", Left, None),
            KeyBinding::new("right", Right, None),
            KeyBinding::new("shift-left", SelectLeft, None),
            KeyBinding::new("shift-right", SelectRight, None),
            KeyBinding::new("cmd-a", SelectAll, None),
            KeyBinding::new("cmd-v", Paste, None),
            KeyBinding::new("cmd-c", Copy, None),
            KeyBinding::new("cmd-x", Cut, None),
            KeyBinding::new("home", Home, None),
            KeyBinding::new("end", End, None),
            KeyBinding::new("enter", Submit, None),
        ]);

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
            |_window, cx| cx.new(|cx| TodoApp::new(cx)),
        )
        .unwrap();

        cx.activate(true);
    });
}
