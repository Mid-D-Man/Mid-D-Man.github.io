// =============================================================================
// src/components/portfolio/wizard.rs
// The hidden viewer for the "Patron saint of freelancers" picture. Nothing on the page
// links to it: it opens when the nav logo is clicked (see nav.rs), and closes on a click
// outside the picture, on the close button, or on Esc.
use leptos::prelude::*;
use wasm_bindgen::JsCast;

#[component]
pub fn WizardViewer(open: ReadSignal<bool>, set_open: WriteSignal<bool>) -> impl IntoView {
    // Esc closes it (same listener pattern as the scroll handler in nav.rs)
    Effect::new(move |_| {
        if let Some(window) = web_sys::window() {
            let closure = wasm_bindgen::closure::Closure::wrap(Box::new(
                move |ev: web_sys::KeyboardEvent| {
                    if ev.key() == "Escape" {
                        set_open.set(false);
                    }
                },
            ) as Box<dyn Fn(web_sys::KeyboardEvent)>);
            let _ = window.add_event_listener_with_callback(
                "keydown",
                closure.as_ref().unchecked_ref(),
            );
            closure.forget();
        }
    });

    view! {
        <Show when=move || open.get()>
            <div
                class="wizard-viewer"
                role="dialog"
                aria-modal="true"
                aria-label="Patron saint of freelancers"
            >
                // Click anywhere outside the picture to close
                <div class="wizard-backdrop" on:click=move |_| set_open.set(false)></div>

                <figure class="about-figure wizard-figure">
                    <div class="about-figure-frame">
                        <img
                            src="images/wizard.jpg"
                            alt="Black-and-white image of a long-bearded wizard holding a fan of dollar bills in one hand and a sword in the other"
                            width="473"
                            height="422"
                        />
                    </div>
                    <figcaption class="about-figure-caption">"Patron saint of freelancers"</figcaption>
                </figure>

                <button
                    class="wizard-close"
                    aria-label="Close"
                    on:click=move |_| set_open.set(false)
                >
                    "✕"
                </button>
            </div>
        </Show>
    }
}
