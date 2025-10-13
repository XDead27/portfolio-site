use leptos::prelude::*;

#[component]
pub fn MasterThesis() -> impl IntoView {
    view! {
        <div class="max-w-xl mx-auto p-6">
            <h1 class="text-2xl font-bold mb-2 text-gray-100">
                "Fuzzing Hypervisors Used in Space Applications"
            </h1>
            <div class="flex items-center text-sm text-gray-500 mb-6">
                <span>"expected 2026"</span>
                <span class="mx-2">"•"</span>
                <span>Daniel Peter, Alexios Voulimeneas, Georgios Smaragdakis</span>
            </div>
            <p class="text-gray-200 mb-4">"More information TBA."</p>
        </div>
    }
}
