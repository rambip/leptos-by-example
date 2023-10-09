use fuzzy_matcher::skim::SkimMatcherV2;

use leptos::*;

use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

#[component]
fn ExampleMatch(
    name: String,
    description: StoredValue<String>,
    highlighted: bool,
    matches: Vec<usize>,
    ) -> impl IntoView {
    // TODO: highlight `matches` in description
    view!{
            <div style=if highlighted {"background-color: gray"} else {""}>
                <b>{name}</b><p>{description}</p>
            </div>
        }
}

#[component]
pub fn FuzzyFinder (
    /// the (name, snippet) pairs to research into
    snippets: Vec<(String, StoredValue<String>)>,
    /// the setter for the index of the item chosen by the user
    choice: WriteSignal<usize>,
    ) -> impl IntoView 
{
    // word written by the user
    let (request, set_request) = create_signal(String::new());
    // wether the search bar is focused
    let (focused, set_focus) = create_signal(false);
    // the index of the currently selected word
    let (highlighted, highlight) = create_signal(0);

    let len = snippets.len();

    let matcher = SkimMatcherV2::default();

    // `scores()[i]` contains the result of the matcher 
    // when comparing `request` with `snippets[i].0`
    let scores = create_memo({let snippets=snippets.clone(); move |_| snippets.clone()
        .iter()
        .map(|(_, description)| request.with(|r| description.with_value(|d| 
                            matcher.fuzzy(d, r, true))
        ))
        .collect::<Vec<_>>()
    });

    let unwrapped_score = move |i: &usize| match scores()[*i] {
        Some((score, _)) => score,
        None => panic!(),
    };

    // the indices of the snippets, but sorted
    // according to the match
    let ordered_matches = create_memo(move |_| {
        let mut result : Vec<usize> = (0..len).filter(|i| scores()[*i].is_some()).collect();
        result.sort_by_key(unwrapped_score);
        result
    });

    let confirm = move || choice(ordered_matches()[highlighted()]);

    // exits the search bar
    let exit = move || {
        set_request(String::new());
        set_focus(false);
        let _ = document()
            .active_element()
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap()
            .blur();
    };

    // view of the matchs
    let match_list = move || {
        let snippets=snippets.clone();
        ordered_matches()
        .into_iter()
        .enumerate()
        .map(|(i, snippet_id)| view!{<ExampleMatch 
            name=snippets[snippet_id].0.clone() 
            description=snippets[snippet_id].1 
            highlighted={highlighted()==i}
            matches=scores()[snippet_id].clone().unwrap().1
            on:mouseover=move |_| highlight(i)
            on:mousedown=move |_| {confirm(); exit()}
            />})
        .collect_view()
    };


    view!{
        <div>
            <input type="text"
                placeholder="search example here"
                on:input=move |ev| {
                set_request(event_target_value(&ev));
                set_focus(true);
            }
                on:focusout=move |_| set_focus(false)

                on:keydown = move |ev| {
                    if focused(){
                        if ev.key() == "Enter" {
                            confirm();
                            exit()
                        }
                        if ev.key() == "Escape" {
                            exit()
                        }
                        if ev.key() == "ArrowDown" {
                            let i = highlighted();
                            if i<ordered_matches().len()-1 { highlight(i+1)}
                        }
                        if ev.key() == "ArrowUp" {
                            let i = highlighted();
                            if i >= 1 { highlight(i-1)}
                        }
                    }
                }

                prop:value=request
            />
            // results are hidden if the search bar is not focused
            <div style="position:relative; opacity:1">
            {move || focused().then(|| match_list())}
            </div>
        </div>
    }
}
