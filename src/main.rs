use leptos::*;
use leptos_router::*;

mod examples;
use examples::{examples, Examples, N_EXAMPLES};

mod fuzzy;
use fuzzy::FuzzyFinder;

use getrandom::getrandom;


#[derive(Clone)]
struct Example {
    pub highlighted_source: &'static str,
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
fn Description<'a>(example: &'a Example) -> impl IntoView {
    view!{
        <pre>
            {example.description}
        </pre>
    }
}


fn random_small_int(n: usize) -> usize {
    let buf: &mut [u8] = &mut [0,0];
    getrandom(buf).unwrap();
    let (a,b) = (buf[0] as usize, buf[1] as usize);
    (a*256 + b) % n
}

#[component]
fn RandomSelector<F: Fn(usize) + 'static>(choice: F, n: usize) -> impl IntoView {
    view!{
        <button on:click=move |_| choice(random_small_int(n) as usize)>
            random example
        </button>
    }
}

#[component]
fn ExampleView<F,I> (
    examples: examples::Examples,
    name: Signal<String>,
    fallback: F
    ) -> impl IntoView 
    where F: Fn(String) -> I + 'static,
          I: IntoView
{
    move || match examples.get(&name() as &str) {
        Some(e) => view!{
            <Description example=e/>
            // the code
            <div style="display:flex; height: 100%">
                <div style="width: 50%; height: 100%; overflow-y: scroll"
                    inner_html=e.highlighted_source
                >
                </div>
                // the in-browser demo
                <div style="border: 2px solid black; margin: 10px; width: 50%; height: 100%; overflow-y: scroll"
                    css=e.css>
                    {e.code}
                </div>
            </div>
        }.into_view(),
        None => fallback(name()).into_view()
    }
}

#[component]
fn App(examples: examples::Examples,
       default: &'static str
    ) -> impl IntoView {
    let (current_name, set_current_name) = create_query_signal("example");
    let current_name = Signal::derive(
        move || current_name().unwrap_or(default.to_string())
    );
    let set_current_name = move |x| set_current_name(Some(x));

    let names: Vec<_> = examples.keys().cloned().collect();

    let snippets: Vec<_> = examples.clone()
        .into_iter().map(|(name, x)| (name.to_string(), store_value(x.description.to_owned())))
        .collect();

    let set_current_example_by_index = move |i: usize|
        set_current_name(names[i].to_string());

    view!{
        <Router>
            <div style:display="flex">
                <FuzzyFinder snippets=snippets choice=set_current_example_by_index.clone()/>
                <RandomSelector choice=set_current_example_by_index n=N_EXAMPLES/>
            </div>
            <h2>{current_name}</h2>
            <ExampleView 
                examples=examples 
                name=current_name 
                fallback=move |x| view!{<div>example {x} does not exist</div>}
        />
        </Router>
    }
}

fn main(){
    console_error_panic_hook::set_once();

    let entrypoint = move ||
        view!{
            <Router>
                <App examples=examples() default="hello_world"/>
            </Router>
        };


    leptos::mount_to_body(entrypoint)
}
