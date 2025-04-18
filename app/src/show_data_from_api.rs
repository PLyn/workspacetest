use leptos::prelude::*;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    let value = RwSignal::new("".to_string());
    let counter = RwSignal::new(0);

    let on_click = move |_| {
        leptos::task::spawn_local(async move {
            let api_said = say_hi(counter.get()).await.unwrap();
            value.set(api_said);
            counter.update(|v| *v += 1);
        });
    };

    view! {
        <div>
            <button on:click=on_click>"What does the API say?"</button>
            <p>{value}</p>
        </div>
    }
}

#[server(SayHi)]
pub async fn say_hi(num: i32) -> Result<String, ServerFnError> {
    Ok(format!("Hello from the API!!! I got {num}"))
}