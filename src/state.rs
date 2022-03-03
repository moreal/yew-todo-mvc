use std::rc::Rc;

use yew::prelude::*;

use crate::domains::filter::Filter;
use crate::domains::todo::Todo;
use crate::domains::todo::TodoList;

pub enum Msg {
    AddTodo(String),
    Toggle(usize),
    ClearCompleted,
    SetFilter(Filter),
    Destroy(usize),
    ToggleAll,
}

#[derive(PartialEq)]
pub struct State {
    pub todo_list: TodoList,
    pub filter: Filter,
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
                State { todo_list, ..*self }
            }
            Msg::Toggle(idx) => {
                let mut todo_list = self.todo_list.clone();
                todo_list[idx].finished = !todo_list[idx].finished;
                State { todo_list, ..*self }
            }
            Msg::ClearCompleted => {
                let mut todo_list = self.todo_list.clone();
                todo_list.retain(|todo| !todo.finished);
                State { todo_list, ..*self }
            }
            Msg::SetFilter(filter) => State {
                filter,
                todo_list: self.todo_list.clone(),
            },
            Msg::Destroy(idx) => {
                let mut todo_list = self.todo_list.clone();
                todo_list.remove(idx);
                State { todo_list, ..*self }
            }
            Msg::ToggleAll => {
                let todo_list = self
                    .todo_list
                    .iter()
                    .map(|todo| Todo {
                        finished: true,
                        content: todo.content.clone(),
                    })
                    .collect();
                State { todo_list, ..*self }
            }
        }
        .into()
    }
}
