use leptos::*;

#[component]
fn Greeter(
    #[prop(into)]
    name: MaybeSignal<String>
    ) -> impl IntoView {

    view!{
        <p>
            hello {move || name()} !
        </p>
    }
}

pub fn showcase() -> impl IntoView {
    let (changing_name, set_name) = create_signal("bob".to_string());

    view!{
        <h3>This name will never change</h3>
        <Greeter name="rust"/>

        <h3>This name can change</h3>
        <div>
            <button on:click=move |_| set_name("alice".to_string())>
                alice
            </button>
            <button on:click=move |_| set_name("bob".to_string())>
                bob
            </button>
        </div>
        <Greeter name=changing_name/>
    }
}
