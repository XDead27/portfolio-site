use leptos::prelude::*;

use crate::components::Chip;

#[component]
pub fn DroneProject() -> impl IntoView {
    view! {
        <div class="flex flex-row h-full">
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
    }
}
