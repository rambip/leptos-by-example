use leptos::*;
use leptos_router::*;

mod examples;
use examples::{examples, N_EXAMPLES};

use std::rc::Rc;

mod fuzzy;
use fuzzy::{FuzzyFinder, FuzzyAble, SkimMatcherV2};

use getrandom::getrandom;

use stylist::Style;

static PUBLIC_DIR: &str = "leptos-by-example";

#[derive(Clone)]
struct Example {
    pub name: &'static str,
    pub source: &'static str,
    pub highlighted_source: &'static str,
    pub code: Signal<View>,
    pub css: Style,
    pub description: &'static str,
    pub motivation: &'static str,
    pub related: Option<&'static str>,
}

impl FuzzyAble for Rc<Example> {
    fn name(&self) -> String {
        self.name.to_string()
    }

    fn description(&self) -> String {
        self.description.to_string()
    }

    fn score(&self, matcher: &SkimMatcherV2, request: &str) -> Option<i64> {
        if let Some(s) = matcher.fuzzy(self.name, request, true){
            return Some(s.0);
        }
        if let Some(s) = matcher.fuzzy(self.description, request, true){
            return Some(s.0);
        }
        if let Some(s) = matcher.fuzzy(self.source, request, true){
            return Some(s.0);
        }
        None
    }
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
fn Suggestions(
    #[prop(into)]
    choice: Callback<&'static str>,
    examples: Vec<Rc<Example>>,
    n: usize
    ) -> impl IntoView {
    view!{
        <button on:click=move |_| choice(examples[random_small_int(n)].name)>
            random example
        </button>
        <button class="suggestion" on:click=move |_| choice("hello_world")>
            hello world
        </button>
    }
}

#[component]
fn ExampleView<F,I, 'a> (
    example: Option<&'a Example>,
    name: String,
    fallback: F
    ) -> impl IntoView 
    where F: Fn(String) -> I + 'static,
          I: IntoView + 'a

{
    match example {
        Some(e) => view!{
            // the code
            <div class="code-snippet" inner_html=e.highlighted_source></div>
            // the in-browser demo
            <div class="demo">
                <div class=e.css.get_class_name().to_string()>{e.code}</div>
            </div>
            <Documentation example=e/>
        }.into_view(),
        None => fallback(name).into_view()
    }
}

#[component]
fn App(examples: examples::Examples,
       default: &'static str
    ) -> impl IntoView {

    let location = use_location();
    let current_name = move ||
        match &location.hash.get().chars().collect::<Vec<_>>()[..] {
            [] => None,
            ['#'] => Some(default.to_string()),
            ['#', rest @ ..] => Some(rest.into_iter().collect()),
            _ => unreachable!()
    };

    let searchbar_focus = create_rw_signal(false);

    let navigate = leptos_router::use_navigate();
    let set_current_name = Callback::new(
        move |dest| navigate(&format!("{PUBLIC_DIR}/#{dest}"), Default::default())
    );


    let key_handle = window_event_listener(ev::keypress, move |ev| {
        if ev.key() == "s" {
            searchbar_focus.set(true);
        }
    });
    on_cleanup(move || key_handle.remove());

    let examples_list: Vec<Rc<_>> = examples.values().cloned().collect();

    let names : Vec<_> = examples.keys().cloned().collect();

    let set_name = Callback::new(move |i| {
        set_current_name(names[i])
    });

    view!{
        <h1 class="title">Leptos by example</h1>
        <div class="container" clone:names>
            <FuzzyFinder 
                placeholder="type `s` or click here to search example"
                items=examples_list.clone()
                focus=searchbar_focus
                choice=set_name
            />
            {move || match current_name(){
                Some(name) => view!{
                    <b class="example-title">{name.clone()}</b>
                    <ExampleView 
                        example=examples.get(name.as_str()).map(|x| x.as_ref())
                        name=name
                        fallback=move |x| view!{<div>example {x} does not exist</div>}
                    />
                }.into_view(),
                None => view!{<Suggestions 
                    choice=set_current_name 
                    examples=examples_list.clone()
                    n=N_EXAMPLES/>
                }.into_view()
            }}
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
