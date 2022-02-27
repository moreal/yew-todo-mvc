use yew::prelude::*;
use web_sys::HtmlInputElement as InputElement;

struct Todo {
    pub finished: bool,
    pub content: String,
}

type TodoList = Vec<Todo>;

struct App {
    todo_list: TodoList,
}

enum Msg {
    AddTodo(String)
}

impl Component for App {
    type Message = Msg;

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

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddTodo(content) => {
                self.todo_list.push(Todo { finished: false, content })
            }
        }
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
