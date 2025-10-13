use leptos::prelude::*;

#[component]
pub fn BachelorThesis() -> impl IntoView {
    view! {
        <div class="max-w-xl mx-auto p-6">
            <h1 class="text-2xl font-bold mb-2 text-gray-100">
                "Accelerating t-SNE Using the Lorenz Hyperboloid"
            </h1>
            <div class="flex items-center text-sm text-gray-500 mb-6">
                <span>"2024"</span>
                <span class="mx-2">"•"</span>
                <span>Daniel Peter, Martin Skrodzki, Elmar Eisemann</span>
            </div>
            <p class="text-gray-200 mb-4">
                "This paper investigates a method for accelerating hyperbolic t-SNE — a popular high-dimensional data visualization technique. In particular, it focuses on building a hyperbolic t-SNE variant that uses a different model of hyperbolic space (called the Lorentz Hyperboloid model) for representing the low-dimensional embeddings. An acceleration algorithm based on a tree data-structure is then used to achieve a better asymptotic runtime complexity compared to the original version. The paper then compares this implementation with other alternatives — including accelerated variants — and shows that it computes embeddings of better quality at a similar rate."
            </p>
            <a
                href="https://repository.tudelft.nl/record/uuid:977ffee3-b1ab-44bc-9361-0eb607d9df0c"
                target="_blank"
                rel="noopener noreferrer"
                class="inline-block px-4 py-2 bg-green text-beige rounded-sm hover:bg-green-100 transition-colors"
            >
                Read Publication
            </a>
        </div>
    }
}
