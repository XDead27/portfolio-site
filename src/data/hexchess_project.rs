use leptos::prelude::*;

use crate::components::Chip;

#[component]
pub fn HexChessProject() -> impl IntoView {
    view! {
        <div class="flex flex-row h-full">
            <div class="basis-1/3">
                <img
                    src="/images/hexagon.png"
                    alt="Hexagon Chess Showcase"
                    class="h-full w-full object-cover object-top-left"
                />
            </div>
            <div class="basis-2/3 flex justify-center h-full py-8 px-6 max-w-4xl mx-auto overflow-y-auto overflow-x-hidden">
                <div>
                    <h1 class="text-2xl font-bold mb-8 text-center text-gray-100">
                        "Online Hexagon Chess"
                    </h1>
                    <p class="text-gray-300 mb-4">
                        "Hexagonal chess is a chess variant played on a hexagonal board. The game was invented by Władysław Gliński in 1936. The game is played on a hexagonal board comprising 91 hexagonal cells."
                    </p>
                    <p class="text-gray-300 mb-4">
                        "I developed an online version of hexagonal chess that allows two players to play against each other in real-time."
                    </p>
                    <div class="flex justify-center mb-4">
                        <Chip name="JavaScript".to_string() />
                        <Chip name="Node.js".to_string() />
                        <Chip name="Express".to_string() />
                        <Chip content=view! {
                            <a href="https://github.com/XDead27/hexagon-chess">
                                <img src="/icons/git.svg" alt="GitHub" class="w-4 h-4 inline" />
                            </a>
                        }
                            .into_any() />
                    </div>
                </div>
            </div>
        </div>
    }
}
