use gpui::*;
use gpui::prelude::*;
use gpui_ui_components::*;

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
    input_text: String,
    show_completed: bool,
}

impl TodoApp {
    fn new() -> Self {
        Self {
            todos: vec![
                TodoItem {
                    id: 0,
                    text: "Learn GPUI basics".to_string(),
                    completed: true,
                },
                TodoItem {
                    id: 1,
                    text: "Build gpui-ui component library".to_string(),
                    completed: true,
                },
                TodoItem {
                    id: 2,
                    text: "Create an awesome todo app".to_string(),
                    completed: false,
                },
                TodoItem {
                    id: 3,
                    text: "Implement interactive features".to_string(),
                    completed: false,
                },
            ],
            next_id: 4,
            input_text: String::new(),
            show_completed: true,
        }
    }

    fn add_todo(&mut self, cx: &mut Context<Self>) {
        if !self.input_text.trim().is_empty() {
            self.todos.push(TodoItem {
                id: self.next_id,
                text: self.input_text.clone(),
                completed: false,
            });
            self.next_id += 1;
            self.input_text.clear();
            cx.notify();
        }
    }

    fn toggle_todo(&mut self, id: usize, cx: &mut Context<Self>) {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.completed = !todo.completed;
            cx.notify();
        }
    }

    fn delete_todo(&mut self, id: usize, cx: &mut Context<Self>) {
        self.todos.retain(|t| t.id != id);
        cx.notify();
    }

    fn clear_completed(&mut self, cx: &mut Context<Self>) {
        self.todos.retain(|t| !t.completed);
        cx.notify();
    }

    fn toggle_show_completed(&mut self, cx: &mut Context<Self>) {
        self.show_completed = !self.show_completed;
        cx.notify();
    }

    fn active_count(&self) -> usize {
        self.todos.iter().filter(|t| !t.completed).count()
    }

    fn completed_count(&self) -> usize {
        self.todos.iter().filter(|t| t.completed).count()
    }

    fn visible_todos(&self) -> Vec<&TodoItem> {
        if self.show_completed {
            self.todos.iter().collect()
        } else {
            self.todos.iter().filter(|t| !t.completed).collect()
        }
    }

    fn render_todo_item(&self, todo: &TodoItem, cx: &mut Context<Self>) -> impl IntoElement {
        let todo_id = todo.id;
        let is_completed = todo.completed;
        
        Card::new()
            .variant(CardVariant::Outlined)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .p_3()
                    .child(
                        // Checkbox with click handler
                        div()
                            .id(("checkbox", todo_id))
                            .cursor_pointer()
                            .on_click(cx.listener(move |this, _event, window, cx| {
                                this.toggle_todo(todo_id, cx);
                                window.refresh();
                            }))
                            .child(
                                Checkbox::new()
                                    .checked(is_completed)
                                    .size(CheckboxSize::Medium)
                            )
                    )
                    .child(
                        div()
                            .flex_1()
                            .text_color(if todo.completed {
                                rgb(0x94a3b8)
                            } else {
                                rgb(0x0f172a)
                            })
                            .when(todo.completed, |d| {
                                d.line_through()
                            })
                            .child(todo.text.clone())
                    )
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .child(
                                Badge::new(if todo.completed { "Done" } else { "Todo" })
                                    .variant(if todo.completed {
                                        BadgeVariant::Success
                                    } else {
                                        BadgeVariant::Warning
                                    })
                                    .size(BadgeSize::Small)
                            )
                            .child(
                                // Delete button with click handler
                                div()
                                    .id(("delete", todo_id))
                                    .cursor_pointer()
                                    .on_click(cx.listener(move |this, _event, window, cx| {
                                        this.delete_todo(todo_id, cx);
                                        window.refresh();
                                    }))
                                    .child(
                                        Button::new("Delete")
                                            .variant(ButtonVariant::Destructive)
                                            .size(ButtonSize::Small)
                                    )
                            )
                    )
            )
    }
}

impl Render for TodoApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let active_count = self.active_count();
        let completed_count = self.completed_count();

        div()
            .flex()
            .flex_col()
            .items_center()
            .size_full()
            .bg(rgb(0xf8fafc))
            .p_8()
            .child(
                Card::new()
                    .variant(CardVariant::Elevated)
                    .child(
                        CardHeader::new()
                            .title("📝 Todo App")
                            .description("A functional todo list built with GPUI-UI components")
                    )
                    .child(
                        CardContent::new()
                            .child(
                                div()
                                    .w(px(600.0))
                                    .flex()
                                    .flex_col()
                                    .gap_4()
                                    .child(
                                        // Stats badges
                                        div()
                                            .flex()
                                            .gap_2()
                                            .items_center()
                                            .child(
                                                Badge::new(format!("{} active", active_count))
                                                    .variant(BadgeVariant::Primary)
                                            )
                                            .child(
                                                Badge::new(format!("{} completed", completed_count))
                                                    .variant(BadgeVariant::Success)
                                            )
                                            .child(
                                                Badge::new(format!("{} total", self.todos.len()))
                                                    .variant(BadgeVariant::Secondary)
                                            )
                                    )
                                    .child(
                                        // Note about limitations
                                        div()
                                            .p_3()
                                            .bg(rgb(0xfef3c7))
                                            .border_1()
                                            .border_color(rgb(0xfcd34d))
                                            .rounded(px(4.0))
                                            .text_sm()
                                            .text_color(rgb(0x78350f))
                                            .child("Note: Text input is visual only. Click checkboxes and buttons to interact!")
                                    )
                                    .child(
                                        // Input section (visual only for now)
                                        Input::new()
                                            .placeholder("What needs to be done?")
                                            .value(&self.input_text)
                                            .label("New Todo (visual only - use Add Todo button)")
                                    )
                                    .child(
                                        // Action buttons with click handlers
                                        div()
                                            .flex()
                                            .gap_2()
                                            .child(
                                                div()
                                                    .id("add-todo-btn")
                                                    .cursor_pointer()
                                                    .on_click(cx.listener(|this, _event, window, cx| {
                                                        // Add a random task for demo
                                                        let tasks = vec![
                                                            "Review code",
                                                            "Write documentation",
                                                            "Fix bugs",
                                                            "Add tests",
                                                            "Refactor components",
                                                            "Update README",
                                                        ];
                                                        let task = tasks[this.next_id % tasks.len()];
                                                        this.input_text = task.to_string();
                                                        this.add_todo(cx);
                                                        window.refresh();
                                                    }))
                                                    .child(
                                                        Button::new("Add Random Todo")
                                                            .variant(ButtonVariant::Default)
                                                            .size(ButtonSize::Medium)
                                                    )
                                            )
                                            .child(
                                                div()
                                                    .id("clear-input-btn")
                                                    .cursor_pointer()
                                                    .on_click(cx.listener(|this, _event, window, cx| {
                                                        this.input_text.clear();
                                                        cx.notify();
                                                        window.refresh();
                                                    }))
                                                    .child(
                                                        Button::new("Clear Input")
                                                            .variant(ButtonVariant::Ghost)
                                                            .size(ButtonSize::Medium)
                                                    )
                                            )
                                    )
                                    .child(
                                        // Filter checkbox with click handler
                                        div()
                                            .id("show-completed-checkbox")
                                            .cursor_pointer()
                                            .on_click(cx.listener(|this, _event, window, cx| {
                                                this.toggle_show_completed(cx);
                                                window.refresh();
                                            }))
                                            .child(
                                                Checkbox::new()
                                                    .checked(self.show_completed)
                                                    .label("Show completed tasks")
                                            )
                                    )
                                    .child(
                                        // Todo list with interactive items
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_2()
                                            .children(
                                                self.visible_todos().into_iter().map(|todo| {
                                                    self.render_todo_item(todo, cx)
                                                })
                                            )
                                    )
                            )
                    )
                    .child(
                        CardFooter::new()
                            .child(
                                Button::new(format!("{} items remaining", active_count))
                                    .variant(ButtonVariant::Ghost)
                                    .size(ButtonSize::Small)
                            )
                            .when(completed_count > 0, |d| {
                                d.child(
                                    div()
                                        .id("clear-completed-btn")
                                        .cursor_pointer()
                                        .on_click(cx.listener(|this, _event, window, cx| {
                                            this.clear_completed(cx);
                                            window.refresh();
                                        }))
                                        .child(
                                            Button::new("Clear Completed")
                                                .variant(ButtonVariant::Outline)
                                                .size(ButtonSize::Small)
                                        )
                                )
                            })
                            .child(
                                Button::new("About")
                                    .variant(ButtonVariant::Link)
                                    .size(ButtonSize::Small)
                            )
                    )
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(800.0), px(750.0)), cx);
        
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("Interactive Todo App - GPUI UI Components".into()),
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
