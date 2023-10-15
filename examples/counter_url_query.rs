use leptos::*;
use leptos_router::*;

#[component]
pub fn SimpleQueryCounter() -> impl IntoView {
    let (count, set_count) = create_query_signal::<i32>("count");
    let clear = move |_| set_count.set(None);
    let decrement =
        move |_| set_count.set(Some(count.get().unwrap_or(0) - 1));
    let increment =
        move |_| set_count.set(Some(count.get().unwrap_or(0) + 1));

    view! {
        <div>
            <button on:click=clear>"Clear"</button>
            <button on:click=decrement>"-1"</button>
            <span>"Value: " {move || count.get().unwrap_or(0)} "!"</span>
            <button on:click=increment>"+1"</button>
        </div>
    }
}

pub fn showcase() -> impl IntoView {
    view!{
        <Router>
            <SimpleQueryCounter/>
        </Router>
    }
}
