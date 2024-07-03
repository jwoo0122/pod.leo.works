use leptos::*;

mod components;
use components::banner::*;
use components::episodes::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="w-screen h-screen bg-gradient-to-br from-red-950 to-red-600 flex flex-col justify-center items-start px-6">
            <Banner />
            <Episodes />
        </div>
    }
}
