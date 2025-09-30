use leptos::prelude::*;

#[component]
pub fn BioWindow() -> impl IntoView {
    view! {
        <div class="py-8 px-6">
            <div class="flex flex-col space-y-4 mx-auto">
                <h1 class="text-2xl font-bold mb-8 text-center text-gray-100">
                    To cut it short
                </h1>
                <div class="flex flex-row justify-center space-x-4 items-center">
                    <img src="/images/profile.png" alt="Profile Picture" class="w-32 h-32 rounded-full outline-4 outline-green outline-offset-4"/>
                    <p>
                        "I'm Daniel. I currently live in Delft (a small city between The Hague and Rotterdam, if you're not familliar), but I'm originally from Romania."
                    </p>
                </div>
                <p>
                    "I left in 2021 to study computer science here, and 'till now I've amassed a bachelor's degree, (almost) a master's degree, and 5 years of experience in bycicle riding."
                </p>
                <p>
                    "I currently enjoy these human activities:"
                    <ul class="list-disc list-inside ml-4">
                        <li>"Reading books (from Sci-fi to philosophy)"</li>
                        <li>"Butchering wood (aka woodworking)"</li>
                        <li>"Playing video games"</li>
                        <li>"Spinning (Poi) balls"</li>
                    </ul>
                </p>
                <p>
                    "I have historically also enjoyed:"
                    <ul class="list-disc list-inside ml-4">
                        <li>"Playing the piano (classical music) and guitar"</li>
                        <li>"Practicing capoiera"</li>
                    </ul>
                </p>
            </div>
        </div>
    }
}
