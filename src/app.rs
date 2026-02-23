// =============================================================================
// src/app.rs
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use js_sys::Array;
use crate::components::portfolio::*;

#[component]
pub fn App() -> impl IntoView {
    // Set up IntersectionObserver for scroll-reveal animations
    Effect::new(move |_| {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                // Small delay to let DOM render first
                let doc_clone = document.clone();
                let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
                    setup_scroll_reveal(&doc_clone);
                }) as Box<dyn Fn()>);

                let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    closure.as_ref().unchecked_ref(),
                    300,
                );
                closure.forget();
            }
        }
    });

    view! {
        <div class="portfolio-app">
            <Nav />
            <main>
                <Hero />
                <About />
                <Services />
                <Projects />
                <Contact />
            </main>
        </div>
    }
}

fn setup_scroll_reveal(document: &web_sys::Document) {
    // Build IntersectionObserver options via js_sys
    let options = js_sys::Object::new();
    let _ = js_sys::Reflect::set(
        &options,
        &wasm_bindgen::JsValue::from_str("threshold"),
        &wasm_bindgen::JsValue::from_f64(0.12),
    );
    let _ = js_sys::Reflect::set(
        &options,
        &wasm_bindgen::JsValue::from_str("rootMargin"),
        &wasm_bindgen::JsValue::from_str("0px 0px -60px 0px"),
    );

    let callback = wasm_bindgen::closure::Closure::wrap(Box::new(
        move |entries: Array, _observer: web_sys::IntersectionObserver| {
            for entry_val in entries.iter() {
                let entry: web_sys::IntersectionObserverEntry =
                    entry_val.unchecked_into();
                if entry.is_intersecting() {
                    let el = entry.target();
                    let el_html: web_sys::HtmlElement = el.unchecked_into();
                    let _ = el_html.class_list().add_1("visible");
                }
            }
        },
    ) as Box<dyn Fn(Array, web_sys::IntersectionObserver)>);

    let options_init: web_sys::IntersectionObserverInit = options.unchecked_into();

    if let Ok(observer) = web_sys::IntersectionObserver::new_with_options(
        callback.as_ref().unchecked_ref(),
        &options_init,
    ) {
        let selectors = [
            ".reveal", ".reveal-left", ".reveal-right", ".reveal-up",
        ];
        for sel in &selectors {
            if let Ok(nodes) = document.query_selector_all(sel) {
                let len = nodes.length();
                for i in 0..len {
                    if let Some(node) = nodes.get(i) {
                        let el: web_sys::Element = node.unchecked_into();
                        observer.observe(&el);
                    }
                }
            }
        }
        std::mem::forget(observer);
    }
    callback.forget();
        }
