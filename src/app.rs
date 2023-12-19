use leptos::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
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
