use gloo::storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement as InputElement;
use yew::prelude::*;

#[derive(Serialize, Deserialize)]
struct Todo {
    pub finished: bool,
    pub content: String,
}

type TodoList = Vec<Todo>;

struct App {
    todo_list: TodoList,
}

enum Msg {
    AddTodo(String),
    Toggle(usize),
    ClearCompleted,
}

const LOCAL_STORAGE_TODO_LIST_KEY: &'static str = "todo_list";

impl Component for App {
    type Message = Msg;

    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        App {
            todo_list: LocalStorage::get(LOCAL_STORAGE_TODO_LIST_KEY)
                .unwrap_or_else(|_| Vec::new()),
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddTodo(content) => self.todo_list.push(Todo {
                finished: false,
                content,
            }),
            Msg::Toggle(idx) => self.todo_list[idx].finished = !self.todo_list[idx].finished,
            Msg::ClearCompleted => self.todo_list = self.todo_list.drain(..).filter(|todo| !todo.finished).collect(), // https://doc.rust-lang.org/stable/std/vec/struct.Drain.html,
        };
        LocalStorage::set(LOCAL_STORAGE_TODO_LIST_KEY, &self.todo_list).expect("failed to set");
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <section class="todoapp">
                <div>
                    <header class="header">
                        <h1>{ "todos" }</h1>
                        <input class="new-todo" onkeypress={
                            ctx.link().batch_callback(|e: KeyboardEvent| match e.key().as_str() {
                                "Enter" => {
                                    let input_element: InputElement = e.target_unchecked_into();
                                    let value = input_element.value();
                                    input_element.set_value(""); // reset
                                    Some(Msg::AddTodo(value))
                                },
                                _ => None,
                            })
                        }/>
                    </header>
                    <section class="main">
                        <ul class="todo-list">
                            { for self.todo_list.iter().enumerate().map(|(idx, x)| html! {
                                <div class="view">
                                    <li class={ if x.finished { "completed" } else { "" } }>
                                        <div class="view">
                                        <input class="toggle" type="checkbox" onclick={ctx.link().callback(move |_| Msg::Toggle(idx))}/>
                                        <label>{ x.content.as_str() }</label>
                                        <button></button>
                                        </div>
                                    </li>
                                </div>
                            })}
                        </ul>
                    </section>
                    <footer class="footer">
                        <span class="todo-count">
                            <strong>{ self.todo_list.iter().filter(|x| !x.finished ).count() }</strong>
                            <span>{" items left"}</span>
                        </span>
                        <ul class="filters">
                            <li><a class="selected">{ "All" }</a></li>
                            <li><a href="#/active">{ "Active" }</a></li>
                            <li><a href="#/completed">{ "Completed" }</a></li>
                        </ul>
                        <button class="clear-completed" onclick={ctx.link().callback(|_| Msg::ClearCompleted)}>{ "Clear completed" }</button>
                    </footer>
                </div>
            </section>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
