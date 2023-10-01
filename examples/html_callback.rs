use leptos::*;
use leptos::html::AnyElement;

#[component]
fn MyFavoriteNumbers(
    #[prop(into)]
    render_number: Callback<i32, HtmlElement<AnyElement>>
    ) -> impl IntoView {
    view!{
        // this syntax only works on nightly.
        // When you are not on nightly, use `render_number.call(...)`
        I like {render_number(73)}
        <br/>
        But I love {render_number(42)}
    }
}

pub fn showcase() -> impl IntoView {
    view!{
        <MyFavoriteNumbers 
            render_number=|x| view!{<b>{x}</b>}
        />
    }
}
