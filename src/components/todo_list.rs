use crate::TodoEntry;
use yew::prelude::*;

use crate::domains::{todo::{TodoList as Items}, filter::Filter};

#[derive(Properties, PartialEq, Clone)]
pub struct TodoListProps {
    pub todo_list: Items,
    pub filter: Filter,
    pub ondestroy: Callback<usize>,
    pub ontoggle: Callback<usize>,
}

#[function_component(TodoList)]
pub fn todo_list(props: &TodoListProps) -> Html {
    html! {
        <ul class="todo-list">
            { for props.todo_list.iter().enumerate().filter(|(_, x)| match props.filter {
                Filter::All => true,
                Filter::Active => !x.finished,
                Filter::Completed => x.finished
            }).map(|(idx, x)| {
                html! { <TodoEntry {idx} ondestroy={props.ondestroy.clone()} ontoggle={props.ontoggle.clone()} todo={x.clone()} /> } 
            })}
        </ul>
    }
}
