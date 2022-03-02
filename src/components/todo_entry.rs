use yew::prelude::*;
use crate::domains::todo::Todo;

#[derive(PartialEq, Properties, Clone)]
pub struct TodoEntryProps {
    pub idx: usize,
    pub todo: Todo,
    pub ontoggle: Callback<usize>,
    pub ondestroy: Callback<usize>,
}

#[function_component(TodoEntry)]
pub fn todo_entry(props: &TodoEntryProps) -> Html {
    let idx = props.idx;
    let toggle = {
        let ontoggle = props.ontoggle.clone();
        move |_| ontoggle.emit(idx)
    };
    let destroy = {
        let ondestroy = props.ondestroy.clone();
        move |_| ondestroy.emit(idx)
    };

    html! {
        <div class="view">
            <li class={ if props.todo.finished { "completed" } else { "" } }>
                <div class="view">
                <input class="toggle" type="checkbox" onclick={toggle}/>
                <label>{ props.todo.content.as_str() }</label>
                <button class="destroy" onclick={destroy}/>
                </div>
            </li>
        </div>
    }
}
