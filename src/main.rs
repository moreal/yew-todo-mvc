use yew::prelude::*;

struct Todo {
    pub finished: bool,
    pub content: String,
}

type TodoList = Vec<Todo>;

struct App {
    todo_list: TodoList,
}

impl Component for App {
    type Message = ();

    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        App {
            todo_list: vec![
                Todo {
                    finished: true,
                    content: "Todo".to_string(),
                },
                Todo {
                    finished: false,
                    content: "AAA".to_string(),
                }
            ]
        }
    }

    fn view(&self, _: &Context<Self>) -> Html {
        let todo_list = self.todo_list.iter().map(|x| html! {
            <li class={ if x.finished { "completed" } else { "" } }>
                <input type="checkbox"/>
                <label>{ x.content.as_str() }</label>
                <button></button>
            </li>
        }).collect::<Html>();
        html! {
            <section>
                <div>
                    <header>
                        <h1>{ "todos" }</h1>
                        <input value={""}/>
                    </header>
                    <section>
                        <ul>
                            {todo_list}
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
