use leptos::*;

mod episode;
use episode::{Epi, Episode};

const EPISODES: [Epi; 1] = [Epi {
    name: "Test Episode",
    description: "This is a test episode.",
    src: "test.mp3",
}];

#[component]
pub fn Episodes() -> impl IntoView {
    view! {
        <div class="w-full mt-24">
            {
                EPISODES.map(| epi | view! { <Episode epi=epi /> })
            }
        </div>
    }
}
