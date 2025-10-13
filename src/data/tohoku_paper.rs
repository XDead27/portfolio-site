use leptos::prelude::*;

#[component]
pub fn TohokuPaper() -> impl IntoView {
    view! {
        <div class="max-w-xl mx-auto p-6">
            <h1 class="text-2xl font-bold mb-2 text-gray-100">
                "Parameters Search for Spatiotemporal Learning Neural Networks"
            </h1>
            <div class="flex items-center text-sm text-gray-500 mb-6">
                <span>"2024"</span>
                <span class="mx-2">"•"</span>
                <span>
                    Daniel Peter, Mamoru Furuya, Takeru Tsuji, Takemori Orima, Yoshihiko Horio
                </span>
            </div>
            <p class="text-gray-200 mb-4">
                "In order to build a hardware spiking neural network (SNN) using the spatiotemporal learning rule (STLR), there have been some constraints when it comes to the input signal patterns. In previous studies, the synchronous digital pulse signals were used. In addition, an evaluation method using the output patterns based on a multistage template matching on a 2-D Hamming distane map was proposed."
            </p>
            <p class="text-gray-200 mb-4">
                "In this study, as a step towards achieving a STLR SNN, pulse width modulated (PWM) binary signals are introduced. The method multistage template matching is also employed to find good learning parameter sets for the PWM spatiotemporal inputs."
            </p>
            <a
                href="https://pub.confit.atlas.jp/ja/event/general2024/presentation/N-1-18"
                target="_blank"
                rel="noopener noreferrer"
                class="inline-block px-4 py-2 bg-green text-beige rounded-sm hover:bg-green-100 transition-colors"
            >
                Read Publication
            </a>
        </div>
    }
}
