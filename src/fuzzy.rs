use fuzzy_matcher::skim::SkimMatcherV2;

use leptos::*;
use leptos::html::Input;

#[component]
fn ExampleMatch(
    name: String,
    description: StoredValue<String>,
    highlighted: bool,
    matches: Vec<usize>,
    ) -> impl IntoView {
    // TODO: highlight `matches` in description
    view!{
            <div style:background-color=highlighted.then(|| "gray")> 
                <b>{name}</b><p>{description}</p>
            </div>
        }
}

#[component]
pub fn FuzzyFinder<F: Fn(usize) + 'static> (
    /// the (name, snippet) pairs to research into
    snippets: Vec<(String, StoredValue<String>)>,
    /// the setter for the index of the item chosen by the user
    choice: F,
    focus: RwSignal<bool>,
    ) -> impl IntoView 
{
    // word written by the user
    let (request, set_request) = create_signal(String::new());

    let (match_visibility, set_match_visibility)=create_signal(false);

    let input_ref = create_node_ref::<Input>();

    create_effect(move |_| {
        if focus.get() {
            input_ref()
                .unwrap()
                .focus()
                .unwrap()
        }
        else {
            input_ref()
                .unwrap()
                .blur()
                .unwrap();
            set_request(String::new());
            set_match_visibility(false);
        }

    });

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
        Some((score, _)) => -score,
        None => panic!(),
    };

    // the indices of the snippets, but sorted
    // according to the match
    let ordered_matches = create_memo(move |_| {
        let mut result : Vec<usize> = (0..len).filter(|i| scores()[*i].is_some()).collect();
        result.sort_by_key(unwrapped_score);
        result
    });

    let confirm = Signal::derive(
        move || choice(ordered_matches()[highlighted()])
    );

    // view of the matchs
    let match_list = Signal::derive(move || {
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
            on:mousedown=move |_| {confirm(); focus.set(false)}
            />})
        .collect_view()
    });


    view!{
        <div style="position: relative">
            <input type="text"
                ref=input_ref
                style:width="100%"
                placeholder="search example here"
                on:input=move |ev| {
                set_request(event_target_value(&ev));
                focus.set(true);
            }
                on:focusout=move |_| focus.set(false)

                on:keydown = move |ev| {
                    if ev.key() == "Escape" {
                        focus.set(false)
                    }
                    if focus.get(){
                        match ev.key().as_ref() {
                            "Enter" => {
                                confirm();
                                focus.set(false)
                            }
                            "ArrowDown" => {
                                let i = highlighted();
                                if i<ordered_matches().len()-1 { highlight(i+1)}
                            }
                            "ArrowUp" => {
                                let i = highlighted();
                                if i >= 1 { highlight(i-1)}
                            }
                            _ => {
                                set_match_visibility(true);
                            }
                        }
                    }
                }

                prop:value=request
            />
            // results are hidden if the search bar is not focused
            <div style="position:absolute; background-color: white">
            {move || match_visibility().then(match_list)}
            </div>
        </div>
    }
}
