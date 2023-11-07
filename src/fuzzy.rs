pub use fuzzy_matcher::skim::SkimMatcherV2;

use leptos::*;
use leptos::html::Input;

#[component]
fn ExampleMatch(
    name: String,
    description: String,
    highlighted: bool,
    ) -> impl IntoView {
    // TODO: highlight `matches` in description
    view!{
            <div style:background-color=highlighted.then(|| "gray")> 
                <b>{name}</b><p>{description}</p>
            </div>
        }
}

pub trait FuzzyAble {
    fn description(&self) -> String;
    fn name(&self) -> String;
    fn score(&self, matcher: &SkimMatcherV2, request: &str) -> Option<i64>;
}

#[component]
pub fn FuzzyFinder<I: FuzzyAble + Clone + 'static, F> (
    /// the items to research into
    items: Vec<I>,
    /// the setter for the index of the item chosen by the user
    choice: F,
    focus: RwSignal<bool>,
    placeholder: &'static str,
    ) 
    -> impl IntoView 
where F: Fn(usize) + 'static
{
    // word written by the user
    let (request, set_request) = create_signal(String::new());

    let input_ref = create_node_ref::<Input>();

    let snippets = store_value(items);

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
        }

    });

    // the index of the currently selected word
    let (highlighted, highlight) = create_signal(0);

    let len = snippets.with_value(|s| s.len());



    let scores: Memo<Vec<_>> = create_memo(move |_| {
        let matcher = SkimMatcherV2::default();
        with!(|snippets| 
             snippets.into_iter()
            .map(|item| with!(|request| item.score(&matcher, request)))
            .collect()
            )
        }
    );


    // the indices of the snippets, but sorted
    // according to the match
    let ordered_matches : Memo<Vec<_>> = create_memo(move |_| {
        let mut result : Vec<(usize, i64)> = (0..len)
            .filter_map(|i| match scores()[i]{
                Some(s) => Some((i, s)),
                _ => None
            })
            .collect();

        result.sort_by_key(|(_, score)| -score);
        result.into_iter()
            .map(|(i, _)| i)
            .collect()
    });

    let confirm = Signal::derive(
        move || choice(ordered_matches()[highlighted()])
    );

    // view of the matchs
    let match_list = Signal::derive(move || {
        let snippets = snippets();
        ordered_matches()
        .into_iter()
        .enumerate()
        .map(|(i, snippet_id)| view!{<ExampleMatch 
            name=snippets[snippet_id].name()
            description=snippets[snippet_id].description()
            highlighted={highlighted()==i}
            on:mouseover=move |_| highlight(i)
            on:mousedown=move |_| {confirm(); focus.set(false)}
            />})
        .collect_view()
    });


    view!{
        <div class="searchbar" style="position: relative">
            <input type="text"
                ref=input_ref
                placeholder=placeholder
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
                            _ => ()
                        }
                    }
                }

                prop:value=request
            />
            // results are hidden if the search bar is not focused,
            // or if no text is written
            <div style="position:absolute; background-color: white">
            {move || (!request().is_empty()).then(match_list)}
            </div>
        </div>
    }
}
