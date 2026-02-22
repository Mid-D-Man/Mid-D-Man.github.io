// =============================================================================
// src/components/portfolio/contact.rs
use leptos::prelude::*;
use wasm_bindgen::JsCast;

#[component]
pub fn Contact() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (email, set_email) = signal(String::new());
    let (message, set_message) = signal(String::new());
    let (status, set_status) = signal(FormStatus::Idle);

    #[derive(Clone, PartialEq)]
    enum FormStatus {
        Idle,
        Sending,
        Success,
    }

    let handle_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        if status.get() != FormStatus::Idle { return; }
        set_status.set(FormStatus::Sending);

        let window = web_sys::window().unwrap();
        let set_status_clone = set_status.clone();
        let set_name_clone = set_name.clone();
        let set_email_clone = set_email.clone();
        let set_message_clone = set_message.clone();

        let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
            set_status_clone.set(FormStatus::Success);
            set_name_clone.set(String::new());
            set_email_clone.set(String::new());
            set_message_clone.set(String::new());

            let set_s = set_status_clone.clone();
            let reset = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
                set_s.set(FormStatus::Idle);
            }) as Box<dyn Fn()>);
            let _ = web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(
                reset.as_ref().unchecked_ref(),
                4000,
            );
            reset.forget();
        }) as Box<dyn Fn()>);

        let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            1800,
        );
        closure.forget();
    };

    view! {
        <section id="contact" class="contact-section section">
            <div class="section-container">
                <div class="section-label">
                    <span class="label-line"></span>
                    <span>"04 — Contact"</span>
                </div>

                <h2 class="section-title reveal">"Let's Build"<br/><em>"Something Real"</em></h2>

                <div class="contact-grid">
                    // Left: Info + Socials
                    <div class="contact-info reveal-left">
                        <p class="contact-tagline">
                            "Have a game idea, a project that needs a developer, "
                            "or just want to say hi? I'm always open to interesting conversations."
                        </p>

                        <div class="contact-direct">
                            <a href="mailto:contact@midmanstudio.com" class="contact-email">
                                <span class="contact-email-icon">"📧"</span>
                                "contact@midmanstudio.com"
                            </a>
                        </div>

                        <div class="socials-block">
                            <h4 class="socials-heading">"Find Me Online"</h4>
                            <div class="socials-list">
                                <SocialLink
                                    href="https://github.com/mid-d-man"
                                    icon="⚡"
                                    label="GitHub"
                                    handle="mid-d-man"
                                />
                                <SocialLink
                                    href="https://linkedin.com/company/MidManStudio"
                                    icon="💼"
                                    label="LinkedIn"
                                    handle="MidManStudio"
                                />
                                <SocialLink
                                    href="https://mid-d-man.github.io"
                                    icon="🌐"
                                    label="Portfolio"
                                    handle="mid-d-man.github.io"
                                />
                            </div>
                        </div>
                    </div>

                    // Right: Form
                    <div class="contact-form-wrap reveal-right">
                        <form
                            class=move || {
                                if status.get() == FormStatus::Success {
                                    "contact-form form-success"
                                } else {
                                    "contact-form"
                                }
                            }
                            on:submit=handle_submit
                        >
                            {move || if status.get() == FormStatus::Success {
                                view! {
                                    <div class="form-success-msg">
                                        <span class="success-icon">"✓"</span>
                                        <p>"Message sent! I'll be in touch soon."</p>
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <div class="form-fields">
                                        <div class="form-group">
                                            <label class="form-label">"Name"</label>
                                            <input
                                                class="form-input"
                                                type="text"
                                                placeholder="Your name"
                                                prop:value=move || name.get()
                                                on:input=move |ev| set_name.set(event_target_value(&ev))
                                                required
                                            />
                                        </div>
                                        <div class="form-group">
                                            <label class="form-label">"Email"</label>
                                            <input
                                                class="form-input"
                                                type="email"
                                                placeholder="your@email.com"
                                                prop:value=move || email.get()
                                                on:input=move |ev| set_email.set(event_target_value(&ev))
                                                required
                                            />
                                        </div>
                                        <div class="form-group">
                                            <label class="form-label">"Message"</label>
                                            <textarea
                                                class="form-textarea"
                                                placeholder="Tell me about your project..."
                                                prop:value=move || message.get()
                                                on:input=move |ev| set_message.set(event_target_value(&ev))
                                                rows="5"
                                                required
                                            ></textarea>
                                        </div>
                                        <button
                                            type="submit"
                                            class=move || {
                                                if status.get() == FormStatus::Sending {
                                                    "form-submit form-submit--loading"
                                                } else {
                                                    "form-submit"
                                                }
                                            }
                                            prop:disabled=move || status.get() == FormStatus::Sending
                                        >
                                            {move || if status.get() == FormStatus::Sending {
                                                "Sending..."
                                            } else {
                                                "Send Message"
                                            }}
                                        </button>
                                    </div>
                                }.into_any()
                            }}
                        </form>
                    </div>
                </div>
            </div>
        </section>

        // Footer
        <footer class="site-footer">
            <div class="footer-inner">
                <div class="footer-brand">
                    <span class="footer-name">"Mid-D-Man"</span>
                    <span class="footer-sep">"/"</span>
                    <span class="footer-studio">"MidManStudio"</span>
                </div>
                <p class="footer-copy">"© 2025 MidManStudio. All rights reserved."</p>
                <p class="footer-built">
                    "Built with Rust + Leptos — compiled to WebAssembly"
                    <span class="footer-crimson">" ◆"</span>
                </p>
            </div>
        </footer>
    }
}

#[component]
fn SocialLink(
    href: &'static str,
    icon: &'static str,
    label: &'static str,
    handle: &'static str,
) -> impl IntoView {
    view! {
        <a
            href=href
            target="_blank"
            rel="noopener noreferrer"
            class="social-link"
        >
            <span class="social-icon">{icon}</span>
            <div class="social-text">
                <span class="social-label">{label}</span>
                <span class="social-handle">{handle}</span>
            </div>
            <span class="social-arrow">"↗"</span>
        </a>
    }
                                                             }
