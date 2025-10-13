use leptos::prelude::*;

use crate::components::Chip;

#[component]
pub fn DroneProject() -> impl IntoView {
    let title = "Drone Flight Control System";
    view! {
        <div class="relative w-full h-full group">
            <div class="absolute inset-0 flex items-center justify-center group-hover:opacity-0 transition-opacity duration-300 z-10 group-hover:-z-10">
                <span class="text-lg">{title}</span>
            </div>
            <div class="flex flex-row h-full blur-lg group-hover:blur-none transition-all duration-300">
                <div class="basis-1/3">
                    <img
                        src="/images/drone.gif"
                        alt="Drone showcase"
                        class="h-full w-full object-cover object-top-left"
                    />
                </div>
                <div class="basis-2/3 flex justify-center h-full py-8 px-6 max-w-4xl mx-auto overflow-y-auto overflow-x-hidden">
                    <div>
                        <h1 class="text-2xl font-bold mb-8 text-center text-gray-100">
                            "Drone Flight Control System"
                        </h1>
                        <p class="text-gray-300 mb-4">
                            "Created the flight software for a quad-engine drone. The code was written in Rust, and it allowed the drone to fly stable, while allowing an operator to make controlled movements. The microchip used was a nRF51822, with a GY-86 sensor module."
                        </p>
                        <div class="flex justify-center mb-4">
                            <Chip name="Rust".to_string() />
                            <Chip name="nRF51822".to_string() />
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
