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
    filter: Filter,
}

enum Msg {
    AddTodo(String),
    Toggle(usize),
    ClearCompleted,
    SetFilter(Filter),
    Destroy(usize),
    ToggleAll,
}

#[derive(Copy, Clone, PartialEq)]
enum Filter {
    All,
    Active,
    Completed,
}

impl Filter {
    fn to_string(self) -> String {
        match self {
            Filter::All => "All",
            Filter::Active => "Active",
            Filter::Completed => "Completed",
        }.to_string()
    }
}

const LOCAL_STORAGE_TODO_LIST_KEY: &'static str = "todo_list";

impl Component for App {
    type Message = Msg;

    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        App {
            todo_list: LocalStorage::get(LOCAL_STORAGE_TODO_LIST_KEY)
                .unwrap_or_else(|_| Vec::new()),
            filter: Filter::All,
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
            Msg::SetFilter(filter) => self.filter = filter,
            Msg::Destroy(idx) => {
                self.todo_list.remove(idx);
                ()
            },
            Msg::ToggleAll => {
                for todo in self.todo_list.iter_mut() {
                    todo.finished = true;
                }
            }
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
                        <input id="toggle-all" class="toggle-all" type="checkbox" />
                        <label for="toggle-all" onclick={ctx.link().callback(move |_| Msg::ToggleAll)}/>
                        <ul class="todo-list">
                            { for self.todo_list.iter().enumerate().filter(|(_, x)| match self.filter {
                                Filter::All => true,
                                Filter::Active => !x.finished,
                                Filter::Completed => x.finished
                            }).map(|(idx, x)| self.view_todo_entry(ctx, idx, x))}
                        </ul>
                    </section>
                    <footer class="footer">
                        <span class="todo-count">
                            <strong>{ self.todo_list.iter().filter(|x| !x.finished ).count() }</strong>
                            <span>{" items left"}</span>
                        </span>
                        <ul class="filters">
                            {for vec![Filter::All, Filter::Active, Filter::Completed].iter().map(|filter| self.view_filter(ctx, filter.to_owned()))}
                        </ul>
                        <button class="clear-completed" onclick={ctx.link().callback(|_| Msg::ClearCompleted)}>{ "Clear completed" }</button>
                    </footer>
                </div>
            </section>
        }
    }
}

impl App {
    fn view_todo_entry(&self, ctx: &Context<Self>, idx: usize, todo: &Todo) -> Html {
        html! {
            <div class="view">
                <li class={ if todo.finished { "completed" } else { "" } }>
                    <div class="view">
                    <input class="toggle" type="checkbox" onclick={ctx.link().callback(move |_| Msg::Toggle(idx))}/>
                    <label>{ todo.content.as_str() }</label>
                    <button class="destroy" onclick={ctx.link().callback(move |_| Msg::Destroy(idx))}/>
                    </div>
                </li>
            </div>
        }
    }

    fn view_filter(&self, ctx: &Context<Self>, filter: Filter) -> Html {
        let filter_string = filter.to_string();
        html! { <li><a class={ if self.filter == filter {"selected"} else {""} } onclick={ctx.link().callback(move |_| Msg::SetFilter(filter))}>{ filter_string }</a></li> }
    }
}

fn main() {
    yew::start_app::<App>();
}
