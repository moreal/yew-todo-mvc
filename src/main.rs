mod components;
mod domains;

use crate::domains::filter::Filter;
use crate::domains::todo::Todo;
use crate::domains::todo::TodoList;
use std::rc::Rc;

use gloo::storage::{LocalStorage, Storage};
use web_sys::HtmlInputElement as InputElement;
use yew::prelude::*;

use components::todo_entry::*;
use components::filter::*;

enum Msg {
    AddTodo(String),
    Toggle(usize),
    ClearCompleted,
    SetFilter(Filter),
    Destroy(usize),
    ToggleAll,
}

const LOCAL_STORAGE_TODO_LIST_KEY: &'static str = "todo_list";

#[derive(PartialEq)]
struct State {
    todo_list: TodoList,
    filter: Filter,
}

impl Reducible for State {
    type Action = Msg;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Msg::AddTodo(content) => {
                let mut todo_list = self.todo_list.clone();
                todo_list.push(Todo {
                    finished: false,
                    content,
                });
                State {
                    todo_list,
                    ..*self
                }
            },
            Msg::Toggle(idx) => {
                let mut todo_list = self.todo_list.clone();
                todo_list[idx].finished = !todo_list[idx].finished;
                State {
                    todo_list,
                    ..*self
                }
            },
            Msg::ClearCompleted => {
                let mut todo_list = self.todo_list.clone();
                todo_list.retain(|todo| !todo.finished);
                State {
                    todo_list,
                    ..*self
                }
            }
            Msg::SetFilter(filter) => State { filter, todo_list: self.todo_list.clone() },
            Msg::Destroy(idx) => {
                let mut todo_list = self.todo_list.clone();
                todo_list.remove(idx);
                State {
                    todo_list,
                    ..*self
                }
            },
            Msg::ToggleAll => {
                let todo_list = self.todo_list.iter().map(|todo| Todo { finished: true, content: todo.content.clone() }).collect();
                State {
                    todo_list,
                    ..*self
                }
            }
        }.into()
    }
}

#[function_component(App)]
fn app() -> Html {
    let state = use_reducer(|| State {
        todo_list: LocalStorage::get(LOCAL_STORAGE_TODO_LIST_KEY)
            .unwrap_or_else(|_| Vec::new()),
        filter: Filter::All,
    });

    use_effect_with_deps(move |state| {
            LocalStorage::set(LOCAL_STORAGE_TODO_LIST_KEY, &state.clone().todo_list).expect("failed to set");
            || ()
        }, 
    state.clone(),
    );

    let onkeypress = {
        let state = state.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let input_element: InputElement = e.target_unchecked_into();
                let value = input_element.value();
                input_element.set_value(""); // reset
                state.dispatch(Msg::AddTodo(value));
            }
        })
    };

    let toggle_all = {
        let state = state.clone();
        Callback::from(move |_| state.dispatch(Msg::ToggleAll))
    };

    let clear_completed = {
        let state = state.clone();
        Callback::from(move |_| state.dispatch(Msg::ClearCompleted))
    };

    let ondestroy = {
        let state = state.clone();
        Callback::from(move |idx: usize| state.dispatch(Msg::Destroy(idx)))
    };
    let ontoggle = {
        let state = state.clone();
        Callback::from(move |idx: usize| state.dispatch(Msg::Toggle(idx)))
    };
    let onsetfilter = {
        let state = state.clone();
        Callback::from(move |filter: Filter| state.dispatch(Msg::SetFilter(filter)))
    };

    html! {
        <section class="todoapp">
            <div>
                <header class="header">
                    <h1>{ "todos" }</h1>
                    <input class="new-todo" {onkeypress}/>
                </header>
                <section class="main">
                    <input id="toggle-all" class="toggle-all" type="checkbox" />
                    <label for="toggle-all" onclick={toggle_all}/>
                    <ul class="todo-list">
                        { for state.todo_list.iter().enumerate().filter(|(_, x)| match state.filter {
                            Filter::All => true,
                            Filter::Active => !x.finished,
                            Filter::Completed => x.finished
                        }).map(|(idx, x)| {
                            let todo: domains::todo::Todo = x.clone();
                            html! { <TodoEntry {idx} ondestroy={ondestroy.clone()} ontoggle={ontoggle.clone()} {todo} /> } 
                        })}
                    </ul>
                </section>
                <footer class="footer">
                    <span class="todo-count">
                        <strong>{ state.todo_list.iter().filter(|x| !x.finished ).count() }</strong>
                        <span>{" items left"}</span>
                    </span>
                    <ul class="filters">
                        {for vec![Filter::All, Filter::Active, Filter::Completed].into_iter().map(|filter| html! { <FilterEntry {filter} selected={filter == state.filter} onsetfilter={onsetfilter.clone()} /> })}
                    </ul>
                    <button class="clear-completed" onclick={clear_completed}>{ "Clear completed" }</button>
                </footer>
            </div>
        </section>
    }
}

fn main() {
    yew::start_app::<App>();
}
