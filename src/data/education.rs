use leptos::prelude::*;

#[component]
pub fn EducationWindow() -> impl IntoView {
    view! {
      <section class="min-h-screen py-8 px-6">
        <div class="max-w-4xl mx-auto">
          <h1 class="text-2xl font-bold mb-8 text-center text-gray-100">
            Education
          </h1>

          <div class="flex items-start shadow-md rounded-sm p-6 mb-8 border-l-8 border-green space-x-4">
            <div class="flex-shrink-0">
              <img
                src="/images/tudelft-logo.png"
                alt="TU Delft Logo"
                class="w-12 h-12 rounded-full object-contain bg-gray-200"
              />
            </div>
            <div>
              <h2 class="text-xl font-semibold text-gray-200">
                MSc Computer Science
              </h2>
              <p class="text-gray-300">
                "Delft University of Technology · 2024 – Present"
              </p>
              <ul class="list-disc list-inside mt-3 text-gray-400">
                <li>Themes: Cybersecurity & Computer Systems</li>
              </ul>
            </div>
          </div>

          <div class="flex items-start shadow-md rounded-sm p-6 mb-8 border-l-8 border-green space-x-4">
            <div class="flex-shrink-0">
              <img
                src="/images/tohoku-logo.png"
                alt="Tohoku University Logo"
                class="w-12 h-12 rounded-full object-contain bg-gray-200"
              />
            </div>
            <div>
              <h2 class="text-xl font-semibold text-gray-200">
                Research Exchange Student
              </h2>
              <p class="text-gray-300">
                "Tohoku University, Japan · Oct 2023 – Feb 2024"
              </p>
              <ul class="list-disc list-inside mt-3 text-gray-400">
                <li>Paper: <em>Spatio-temporal Learning Rule</em> (published at IEICE 2024)
                </li>
                <li>Implemented STLR model in Matlab and tested performance</li>
                <li>Average grade: AA / AA</li>
              </ul>
            </div>
          </div>

          <div class="flex items-start shadow-md rounded-sm p-6 mb-8 border-l-8 border-green space-x-4">
            <div class="flex-shrink-0">
              <img
                src="/images/tudelft-logo.png"
                alt="TU Delft Logo"
                class="w-12 h-12 rounded-full object-contain bg-gray-200"
              />
            </div>
            <div>
              <h2 class="text-xl font-semibold text-gray-200">
                BSc Computer Science and Engineering
              </h2>
              <p class="text-gray-300">
                "Delft University of Technology · 2021 – 2024"
              </p>
              <ul class="list-disc list-inside mt-3 text-gray-400">
                <li>Thesis: <em>Accelerating t-SNE using the Lorentz Hyperboloid</em></li>
                <li>Average grade: 8.86 / 10.0</li>
                <li>Focus areas: Embedded Software, GPU Programming, Cybersecurity</li>
              </ul>
            </div>
          </div>
        </div>
      </section>
    }
}
