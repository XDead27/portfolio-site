use leptos::prelude::*;

use crate::components::Chip;

#[component]
pub fn ImaginaryProject() -> impl IntoView {
    view! {
        <div class="flex flex-row h-full">
            <div class="basis-1/3">
                <img src="/images/graphlex.png" alt="Graphlex Showcase" class="h-full w-full object-cover"/>
            </div>
            <div class="basis-2/3 flex items-center justify-center h-full py-8 px-6 max-w-4xl mx-auto">
                <div>
                    <h1 class="text-2xl font-bold mb-8 text-center text-gray-100">"GraphlexJS"</h1>
                    <p class="text-gray-300 mb-4">
                        "GraphlexJS is a TypeScript library that enables GPU-accelerated complex transformation visualization. GraphlexJS provides an API for transforming an expression in mathematical notation into code that can be run on the GPU. This code can then be used to apply a transformation to image or video data."
                    </p>
                    <p class="text-gray-300 mb-4">
                        "This library was developed for the company "
                        <a href="https://imaginary.org/">
                            "IMAGINARY"
                        </a>
                        ", which creates interactive mathematical exhibits for museums and science centers. The goal of the project was to create a library that could be used to visualize complex transformations in real-time."
                    </p>
                    <div class="flex justify-center mb-4">
                        <Chip name="TypeScript".to_string() />
                        <Chip name="WebGL".to_string() />
                        <Chip name="GPU Programming".to_string() />
                    </div>
                </div>
            </div>
        </div>
    }
}
