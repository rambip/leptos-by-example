use leptos::*;

mod examples;
use examples::{examples, Examples};

mod fuzzy;
use fuzzy::FuzzyFinder;


struct Example {
    pub highlighted_source: &'static str,
    name: &'static str,
    pub code: Signal<View>,
    pub css: Option<&'static str>,
    pub description: &'static str
}

// wraps a function inside a signal.
fn pack_example<F, I>(f: F)-> Signal<View> 
where F: Fn() -> I + 'static,
      I: IntoView
{
    (move || f().into_view()).into_signal()
}

#[component]
fn Description(examples: StoredValue<Examples>, current: ReadSignal<usize>) -> impl IntoView {
    let description = move || {
        examples.with_value(|ex| ex[current()].description)
    };

    view!{
        <pre>
            {description}
        </pre>
    }
}


/// the in-browser demo of the example
#[component]
fn Showcase(examples: StoredValue<Examples>, current: ReadSignal<usize>) -> impl IntoView {
    let current_showcase = 
        move || examples.with_value(|ex| ex[current()].code.get());

    let current_css = 
        move || examples.with_value(|ex| ex[current()].css);

    view!{
        <div style="border: 2px solid black; height: 50%; overflow-y: scroll"
            css=current_css>
            {current_showcase}
        </div>
    }
}

#[component]
fn App(examples: StoredValue<examples::Examples>,
       initial: usize
    ) -> impl IntoView {
    let (current_example, set_current_example) = create_signal(initial);

    let current_source = 
        move || examples.with_value(|ex| ex[current_example()].highlighted_source);

    let descriptions: Vec<_> = examples.with_value(
        |e| e.into_iter().map(|x| (x.name.to_owned(), store_value(x.description.to_owned())))
        .collect()
    );

    view!{
        <FuzzyFinder snippets=descriptions choice=set_current_example/>
        <Description examples=examples current=current_example/>
        // the code
        <div style="height: 50%; overflow-y: scroll"
            inner_html=current_source
        >
        </div>
        <Showcase examples=examples current=current_example/>
    }
}

fn main(){
    let examples = examples();

    let hello_world_id = examples.iter().position(|x| x.name=="hello_world").unwrap();

    let examples = store_value(examples);
    console_error_panic_hook::set_once();


    leptos::mount_to_body(move || view!{<App examples=examples initial=hello_world_id/>});
}
