use leptos::prelude::*;

#[component]
pub fn SkillsWindow() -> impl IntoView {
    view! {
        <section class="min-h-screen py-8 px-6">
          <div class="max-w-5xl mx-auto">
            <h1 class="text-2xl font-bold mb-8 text-center text-gray-100">
              Skills
            </h1>

            <div class="mb-10">
              <h2 class="text-xl font-semibold text-green mb-4">
                Programming Languages
              </h2>
              <div class="flex flex-wrap gap-3">
                <span class="px-4 py-2 bg-green text-black rounded-full text-sm font-medium">C</span>
                <span class="px-4 py-2 bg-green text-black rounded-full text-sm font-medium">C++</span>
                <span class="px-4 py-2 bg-green text-black rounded-full text-sm font-medium">Rust</span>
                <span class="px-4 py-2 bg-green text-black rounded-full text-sm font-medium">Python</span>
                <span class="px-4 py-2 bg-green text-black rounded-full text-sm font-medium">Java</span>
                <span class="px-4 py-2 bg-green text-black rounded-full text-sm font-medium">TypeScript</span>
                <span class="px-4 py-2 bg-green text-black rounded-full text-sm font-medium">Matlab</span>
                <span class="px-4 py-2 bg-green text-black rounded-full text-sm font-medium">x86-64 Assembly</span>
              </div>
            </div>

            <div class="mb-10">
              <h2 class="text-xl font-semibold text-green mb-4">
                Frameworks & Tools
              </h2>
              <div class="flex flex-wrap gap-3">
                <span class="px-4 py-2 bg-beige-200 text-black rounded-full text-sm font-medium">Spring</span>
                <span class="px-4 py-2 bg-beige-200 text-black rounded-full text-sm font-medium">Node.js</span>
                <span class="px-4 py-2 bg-beige-200 text-black rounded-full text-sm font-medium">OpenGL</span>
                <span class="px-4 py-2 bg-beige-200 text-black rounded-full text-sm font-medium">OpenCL</span>
                <span class="px-4 py-2 bg-beige-200 text-black rounded-full text-sm font-medium">Git</span>
                <span class="px-4 py-2 bg-beige-200 text-black rounded-full text-sm font-medium">SCRUM Agile</span>
              </div>
            </div>

            <div class="mb-10">
              <h2 class="text-xl font-semibold text-green mb-4">
                Domains
              </h2>
              <div class="flex flex-wrap gap-3">
                <span class="px-4 py-2 bg-green text-black rounded-full text-sm font-medium">Embedded Systems</span>
                <span class="px-4 py-2 bg-green text-black rounded-full text-sm font-medium">GPU Programming</span>
                <span class="px-4 py-2 bg-green text-black rounded-full text-sm font-medium">Cybersecurity & Ethical Hacking</span>
                <span class="px-4 py-2 bg-green text-black rounded-full text-sm font-medium">Machine Learning</span>
                <span class="px-4 py-2 bg-green text-black rounded-full text-sm font-medium">Reinforcement Learning</span>
              </div>
            </div>

            <div>
              <h2 class="text-xl font-semibold text-green mb-4">
                Soft Skills & Languages
              </h2>
              <div class="flex flex-wrap gap-3">
                <span class="px-4 py-2 bg-beige-200 text-black rounded-full text-sm font-medium">Leadership</span>
                <span class="px-4 py-2 bg-beige-200 text-black rounded-full text-sm font-medium">Teaching</span>
                <span class="px-4 py-2 bg-beige-200 text-black rounded-full text-sm font-medium">Project Management</span>
                <span class="px-4 py-2 bg-beige-200 text-black rounded-full text-sm font-medium">Romanian (native)</span>
                <span class="px-4 py-2 bg-beige-200 text-black rounded-full text-sm font-medium">English (C2)</span>
                <span class="px-4 py-2 bg-beige-200 text-black rounded-full text-sm font-medium">Japanese (N4)</span>
                <span class="px-4 py-2 bg-beige-200 text-black rounded-full text-sm font-medium">French (A2)</span>
              </div>
            </div>
          </div>
        </section>
    }
}
