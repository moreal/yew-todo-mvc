use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <section>
            <div>
                <header>
                    <h1>{ "todos" }</h1>
                    <input value={""}/>
                </header>
                <section>
                    <ul>
                        <li>
                            <input type="checkbox"/>
                            <label>{ "Hello" }</label>
                            <button></button>
                        </li>
                        <li>
                            <input type="checkbox"/>
                            <label>{ "AAA" }</label>
                            <button></button>
                        </li>
                    </ul>
                </section>
                <footer>
                    <span class="todo-count">
                        <strong>{ 2 }</strong>
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

fn main() {
    yew::start_app::<App>();
}
