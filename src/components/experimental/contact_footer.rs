// =============================================================================
// src/components/contact_footer.rs
use leptos::prelude::*;
use wasm_bindgen::JsCast;

#[component]
pub fn ContactFooter() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (email, set_email) = signal(String::new());
    let (message, set_message) = signal(String::new());
    let (is_submitting, set_is_submitting) = signal(false);

    let handle_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        set_is_submitting.set(true);
        
        // Simulate form submission
        let window = web_sys::window().unwrap();
        let closure = wasm_bindgen::closure::Closure::wrap(Box::new({
            let set_is_submitting = set_is_submitting.clone();
            move || {
                set_is_submitting.set(false);
                web_sys::console::log_1(&"Form submitted successfully!".into());
            }
        }) as Box<dyn Fn()>);
        
        let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            2000,
        );
        closure.forget();
    };

    view! {
        <div class="contact-footer">
            <div class="container">
                <div class="contact-content">
                    <div class="contact-info">
                        <h2>"Get In Touch"</h2>
                        <p>"Ready to bridge the gap between your vision and reality?"</p>
                        <div class="contact-links">
                            <a href="mailto:contact@midmanstudio.com" class="contact-link">
                                <span class="icon">"📧"</span>
                                "contact@midmanstudio.com"
                            </a>
                            <a href="https://github.com/mid-d-man" target="_blank" class="contact-link">
                                <span class="icon">"⚡"</span>
                                "GitHub"
                            </a>
                        </div>
                    </div>
                    <div class="contact-form">
                        <form on:submit=handle_submit>
                            <input
                                type="text"
                                placeholder="Your Name"
                                prop:value=move || name.get()
                                on:input=move |ev| set_name.set(event_target_value(&ev))
                                required
                            />
                            <input
                                type="email"
                                placeholder="Your Email"
                                prop:value=move || email.get()
                                on:input=move |ev| set_email.set(event_target_value(&ev))
                                required
                            />
                            <textarea
                                placeholder="Your Message"
                                prop:value=move || message.get()
                                on:input=move |ev| set_message.set(event_target_value(&ev))
                                rows="4"
                                required
                            ></textarea>
                            <button
                                type="submit"
                                class="submit-btn"
                                prop:disabled=move || is_submitting.get()
                            >
                                {move || if is_submitting.get() { "Sending..." } else { "Send Message" }}
                            </button>
                        </form>
                    </div>
                </div>
                <div class="footer-bottom">
                    <p>"© 2025 MidManStudio. Bridging Vision and Reality."</p>
                </div>
            </div>
        </div>
    }
}

