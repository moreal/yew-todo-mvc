mod components;
mod domains;
mod state;

use crate::domains::filter::Filter;

use gloo::storage::{LocalStorage, Storage};
use web_sys::HtmlInputElement as InputElement;
use yew::prelude::*;

use components::filter::*;
use components::todo_entry::*;
use components::todo_list::TodoList as TodoListComponent;

use state::{Msg, State};

const LOCAL_STORAGE_TODO_LIST_KEY: &str = "todo_list";

#[function_component(App)]
fn app() -> Html {
    let state = use_reducer(|| State {
        todo_list: LocalStorage::get(LOCAL_STORAGE_TODO_LIST_KEY).unwrap_or_else(|_| Vec::new()),
        filter: Filter::All,
    });

    use_effect_with_deps(
        move |state| {
            LocalStorage::set(LOCAL_STORAGE_TODO_LIST_KEY, &state.clone().todo_list)
                .expect("failed to set");
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
                    <TodoListComponent todo_list={state.todo_list.clone()} filter={state.filter} {ondestroy} {ontoggle} />
                </section>
                <footer class="footer">
                    <span class="todo-count">
                        <strong>{ state.todo_list.iter().filter(|x| !x.finished ).count() }</strong>
                        <span>{" items left"}</span>
                    </span>
                    <ul class="filters">
                        {for Filter::all().into_iter().map(|filter| html! { <FilterEntry {filter} selected={filter == state.filter} onsetfilter={onsetfilter.clone()} /> })}
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
