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
        }
        LocalStorage::set(LOCAL_STORAGE_TODO_LIST_KEY, &self.todo_list).expect("failed to set");
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <section>
                <div>
                    <header>
                        <h1>{ "todos" }</h1>
                        <input onkeypress={
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
                    <section>
                        <ul>
                            { for self.todo_list.iter().map(|x| html! {
                                <li class={ if x.finished { "completed" } else { "" } }>
                                    <input type="checkbox"/>
                                    <label>{ x.content.as_str() }</label>
                                    <button></button>
                                </li>
                            })}
                        </ul>
                    </section>
                    <footer>
                        <span class="todo-count">
                            <strong>{ self.todo_list.iter().filter(|x| !x.finished ).count() }</strong>
                            <span>{" items left"}</span>
                        </span>
                        <ul><li><a>{ "All" }</a></li></ul>
                        <ul><li><a href="#/active">{ "Active" }</a></li></ul>
                        <ul><li><a href="#/completed">{ "Completed" }</a></li></ul>
                    </footer>
                </div>
            </section>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
