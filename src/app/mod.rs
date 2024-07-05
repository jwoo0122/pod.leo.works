use leptos::*;

mod components;
use components::banner::*;
use components::episodes::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="w-screen h-screen bg-gradient-to-br from-cyan-950 to-cyan-600 flex flex-col justify-center items-start px-6">
            <Banner />
            <Episodes />
        </div>
    }
}
