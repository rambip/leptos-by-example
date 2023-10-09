use leptos::*;
use leptos::html::Select;

static FONTS: [&str; 4] = [
    "verdana",
    "times",
    "sans-serif",
    "monospace",
];

#[component]
fn FontSelector(set_font_index: WriteSignal<usize>) -> impl IntoView {

    let select_ref = create_node_ref::<Select>();

    let font_options = FONTS
    .into_iter()
    .map(|x| view!{ <option value=x.clone()>{x}</option> })
    .collect_view();


    view!{
        <label for="fonts">Choose a font:</label>
        <select name="fonts" id="fonts"
            ref=select_ref
            on:change=move |_| set_font_index(
                select_ref.get().unwrap().selected_index() as usize
                )
        >
            {font_options}
        </select>

    }
}

pub fn showcase() -> impl IntoView {
    let (font_index, set_font_index) = create_signal(0usize);
    view!{
        <FontSelector set_font_index=set_font_index/>
        <p
        style:font-family=move || FONTS[font_index()]>
            Here is how your font looks like
        </p>
    }
}
