use leptos::*;

#[component]
pub fn Banner() -> impl IntoView {
    view! {
        <div class="flex flex-col items-start justify-center">
            <div class="flex justify-center items-center w-28 h-28 bg-neutral-900 rounded-2xl shadow-lg pl-1 shrink-0">
                <img src="/images/podcast-logo.png" alt="podcast-logo" class="w-20"/>
            </div>
            <div class="text-white text-3xl font-title font-bold mt-8">
                "NEVER STOP LEARNING."
            </div>
            <div class="text-white opacity-30 text-md mt-2">
                "Podcast by Jeong Jinwoo"
            </div>
        </div>
    }
}
