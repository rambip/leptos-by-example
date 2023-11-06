use leptos::*;
use leptos_router::*;

mod examples;
use examples::{examples, N_EXAMPLES};

mod fuzzy;
use fuzzy::FuzzyFinder;

use getrandom::getrandom;

use stylist::Style;

static PUBLIC_DIR: &str = "leptos-by-example";

#[derive(Clone)]
struct Example {
    pub highlighted_source: &'static str,
    pub code: Signal<View>,
    pub css: Style,
    pub description: &'static str,
    pub motivation: &'static str,
    pub related: Option<&'static str>,
}

// wraps a function inside a signal.
fn pack_example<F, I>(f: F)-> Signal<View> 
where F: Fn() -> I + 'static,
      I: IntoView
{
    (move || f().into_view()).into_signal()
}

#[component]
fn Documentation<'a>(example: &'a Example) -> impl IntoView {
    view!{
        <div class="description">
            <h3>What</h3>
            <pre>
                {example.description}
            </pre>
            <h3>Why</h3>
            <div inner_html=example.motivation>
            </div>
            <h3>See also</h3>
            <div inner_html=example.related>
            </div>
        </div>
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
            // the code
            <div class="code-snippet" inner_html=e.highlighted_source></div>
            // the in-browser demo
            <div class="demo">
                <div class=e.css.get_class_name().to_string()>{e.code}</div>
            </div>
            <Documentation example=e/>
        }.into_view(),
        None => fallback(name()).into_view()
    }
}

#[component]
fn App(examples: examples::Examples,
       default: &'static str
    ) -> impl IntoView {

    let location = use_location();
    let current_name = move ||
        match &location.hash.get().chars().collect::<Vec<_>>()[..] {
            [] => default.to_string(),
            ['#'] => default.to_string(),
            ['#', rest @ ..] => rest.into_iter().collect(),
            _ => unreachable!()
    };

    let searchbar_focus = create_rw_signal(false);

    create_effect(move |_| logging::log!("current name is {}", current_name()));

    let navigate = leptos_router::use_navigate();
    let set_current_name = Callback::new(
        move |dest| navigate(&format!("{PUBLIC_DIR}/#{dest}"), Default::default())
    );


    let names: StoredValue<Vec<_>> = 
        store_value(examples.keys().cloned().collect());

    let snippets: Vec<_> = examples.clone()
        .into_iter().map(|(name, x)| (name.to_string(), store_value(x.description.to_owned())))
        .collect();

    let key_handle = window_event_listener(ev::keypress, move |ev| {
        if ev.key() == "s" {
            searchbar_focus.set(true);
        }
    });
    on_cleanup(move || key_handle.remove());


    view!{
        <h1 class="title">Leptos by example</h1>
        <div class="container">
            <RandomSelector choice=move |i| set_current_name(names.with_value(|n| n[i])) n=N_EXAMPLES/>
            <FuzzyFinder 
                placeholder="type `s` or click here to search example"
                snippets=snippets 
                focus=searchbar_focus
                choice=move |i| set_current_name(names.with_value(|n| n[i]))
            />
            <b class="example-title">{current_name}</b>
            <ExampleView 
                examples=examples 
                name=current_name.into_signal()
                fallback=move |x| view!{<div>example {x} does not exist</div>}
            />
        </div>
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
