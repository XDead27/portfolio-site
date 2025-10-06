use leptos::prelude::*;

#[component]
pub fn SkillsWindow() -> impl IntoView {
    let class = "px-4 py-2 text-black rounded-sm text-sm font-medium outline outline-beige-200 outline-offset-3";
    let green_pill_class = format!("{} bg-green", class);
    let beige_pill_class = format!("{} bg-gray-400", class);
    view! {
        <section class="py-8 px-6">
            <div class="max-w-5xl mx-auto">
                <h1 class="text-2xl font-bold mb-8 text-center text-gray-100">Skills</h1>

                <div class="mb-10">
                    <h2 class="text-xl font-semibold mb-4">Programming Languages</h2>
                    <div class="flex flex-wrap gap-3">
                        <span class=green_pill_class.clone()>C</span>
                        <span class=green_pill_class.clone()>C++</span>
                        <span class=green_pill_class.clone()>Rust</span>
                        <span class=green_pill_class.clone()>Python</span>
                        <span class=green_pill_class.clone()>Java</span>
                        <span class=green_pill_class.clone()>TypeScript</span>
                        <span class=green_pill_class.clone()>Matlab</span>
                        <span class=green_pill_class.clone()>x86-64 Assembly</span>
                    </div>
                </div>

                <div class="mb-10">
                    <h2 class="text-xl font-semibold mb-4">Frameworks & Tools</h2>
                    <div class="flex flex-wrap gap-3">
                        <span class=beige_pill_class.clone()>Spring</span>
                        <span class=beige_pill_class.clone()>Node.js</span>
                        <span class=beige_pill_class.clone()>OpenGL</span>
                        <span class=beige_pill_class.clone()>OpenCL</span>
                        <span class=beige_pill_class.clone()>Git</span>
                        <span class=beige_pill_class.clone()>"SCRUM Agile"</span>
                    </div>
                </div>

                <div class="mb-10">
                    <h2 class="text-xl font-semibold mb-4">Domains</h2>
                    <div class="flex flex-wrap gap-3">
                        <span class=green_pill_class.clone()>Embedded Systems</span>
                        <span class=green_pill_class.clone()>GPU Programming</span>
                        <span class=green_pill_class.clone()>Cybersecurity & Ethical Hacking</span>
                        <span class=green_pill_class.clone()>Machine Learning</span>
                        <span class=green_pill_class.clone()>Reinforcement Learning</span>
                    </div>
                </div>

                <div>
                    <h2 class="text-xl font-semibold mb-4">Soft Skills & Languages</h2>
                    <div class="flex flex-wrap gap-3">
                        <span class=beige_pill_class.clone()>Leadership</span>
                        <span class=beige_pill_class.clone()>Teaching</span>
                        <span class=beige_pill_class.clone()>Project Management</span>
                        <span class=beige_pill_class.clone()>Romanian (native)</span>
                        <span class=beige_pill_class.clone()>English (C2)</span>
                        <span class=beige_pill_class.clone()>Japanese (N4)</span>
                        <span class=beige_pill_class.clone()>French (A2)</span>
                    </div>
                </div>
            </div>
        </section>
    }
}
