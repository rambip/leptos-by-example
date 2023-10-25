use leptos::*;
use leptos_router::*;

mod examples;
use examples::{examples, N_EXAMPLES};

mod fuzzy;
use fuzzy::FuzzyFinder;

use getrandom::getrandom;

use stylist::Style;


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
        <div style="max-height:30%; overflow:scroll; border: 1px solid black">
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
            <Documentation example=e/>
            // the code
            <div style="display:flex; height:50%">
                <div style="width: 50%; height: 100%; overflow-y: scroll"
                    inner_html=e.highlighted_source
                >
                </div>
                // the in-browser demo
                <div style="border: 2px solid black; margin: 10px; width: 50%; height: 100%; overflow-y: scroll">
                    <div class=e.css.get_class_name().to_string()>{e.code}</div>
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

    let location = use_location();
    let current_name = move ||
        match &location.hash.get().chars().collect::<Vec<_>>()[..] {
            [] => default.to_string(),
            ['#'] => default.to_string(),
            ['#', rest @ ..] => rest.into_iter().collect(),
            _ => unreachable!()
    };

    create_effect(move |_| logging::log!("current name is {}", current_name()));

    let navigate = leptos_router::use_navigate();
    let set_current_name = Callback::new(
        move |dest| navigate(&format!("#{dest}"), Default::default())
    );


    let names: Vec<_> = examples.keys().cloned().collect();

    let snippets: Vec<_> = examples.clone()
        .into_iter().map(|(name, x)| (name.to_string(), store_value(x.description.to_owned())))
        .collect();

    let set_current_example_by_index = move |i: usize| set_current_name(names[i]);

    view!{
        <div style:display="flex">
            <b style="padding-right: 30px; font-size: 25px">{current_name}</b>
            <RandomSelector choice=set_current_example_by_index.clone() n=N_EXAMPLES/>
            <FuzzyFinder snippets=snippets choice=set_current_example_by_index/>
        </div>
        <ExampleView 
            examples=examples 
            name=current_name.into_signal()
            fallback=move |x| view!{<div>example {x} does not exist</div>}
        />
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
