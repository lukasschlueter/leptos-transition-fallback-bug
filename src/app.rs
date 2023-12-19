use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos-transition-fallback-bug.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="load" view=BuggyTransition/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <h1>"Welcome to Leptos!"</h1>
        <A href="/load">"Click here to load a buggy transition"</A>
    }
}

#[component]
fn BuggyTransition() -> impl IntoView {
    let resource = create_resource(|| (), |_| slow_server_function());

    view! {
        <Transition fallback=|| view!{"Loading...!"}>
            {
                move || resource.get().map(|value| match value {
                    Ok(_) => view!{"Success!"},
                    Err(_) => view!{"Error!"},
                })
            }
        </Transition>
    }
}

#[server]
async fn slow_server_function() -> Result<(), ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    Ok(())
}
