use leptos::*;

pub struct Epi<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub src: &'a str,
}

#[component]
pub fn Episode(epi: Epi<'static>) -> impl IntoView {
    view! {
        <div class="max-w-7 rounded-lg w-full text-white">
            <audio controls src=format!("/episodes/{}", epi.src) ></audio>
            <div class="text-2xl mt-4 font-title">{epi.name}</div>
            <div class="opacity-50">{epi.description}</div>
        </div>
    }
}
