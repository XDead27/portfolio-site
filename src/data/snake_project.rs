use leptos::prelude::*;

use crate::components::Chip;

#[component]
pub fn SnakeProject() -> impl IntoView {
    view! {
        <div class="flex flex-row h-full">
            <div class="basis-1/3">
                <img
                    src="/images/snake.gif"
                    alt="Multiplayer Terminal Snake Game Showcase"
                    class="h-full w-full object-cover object-top-left"
                />
            </div>
            <div class="basis-2/3 flex justify-center h-full py-8 px-6 max-w-4xl mx-auto overflow-y-auto overflow-x-hidden">
                <div>
                    <h1 class="text-2xl font-bold mb-8 text-center text-gray-100">
                        "Multiplayer Terminal Snake Game"
                    </h1>
                    <p class="text-gray-300 mb-4">
                        "I developed a multiplayer terminal-based snake game. Players can connect to a server via their terminal and control their snake using keyboard inputs. The game supports multiple players, and different game modes."
                    </p>
                    <div class="flex justify-center mb-4">
                        <Chip name="Python".to_string() />
                        <Chip content=view! {
                            <a href="https://github.com/XDead27/ConsoleSnake">
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
