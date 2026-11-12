// =============================================================================
// src/components/portfolio/contact.rs
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use crate::components::portfolio::icons::*;

// Where the contact form sends its messages. FormSubmit (formsubmit.co) is free and needs no account: it emails every submission
// to the address in the URL. The first message sent through the live site makes it email that address a one-time activation link;
// click it once and the form works. After activating you can swap the address below for the random-looking alias FormSubmit gives
// you, so the page source shows no address in the endpoint.
const CONTACT_ENDPOINT: &str = "https://formsubmit.co/ajax/midmanstudio@proton.me";

fn encode(s: &str) -> String {
    String::from(js_sys::encode_uri_component(s))
}

/// POSTs a form-encoded body to `url` and calls `done(true)` when the server answers with "success":"true", and `done(false)` for
/// anything else: an error, a timeout, no connection, or a form that is not activated yet. XmlHttpRequest rather than fetch: plain
/// event callbacks (the same closure pattern as the rest of the site), and a form-encoded body is a "simple" request, so the
/// browser sends no preflight.
fn post_form(url: &str, body: &str, done: impl Fn(bool) + 'static) {
    let done = std::rc::Rc::new(done);
    let xhr = match web_sys::XmlHttpRequest::new() {
        Ok(x) => x,
        Err(_) => {
            (*done)(false);
            return;
        }
    };
    if xhr.open_with_async("POST", url, true).is_err() {
        (*done)(false);
        return;
    }
    let _ = xhr.set_request_header("Content-Type", "application/x-www-form-urlencoded");
    let _ = xhr.set_request_header("Accept", "application/json");
    xhr.set_timeout(15_000);

    let on_load = {
        let xhr = xhr.clone();
        let done = done.clone();
        wasm_bindgen::closure::Closure::wrap(Box::new(move || {
            let status = xhr.status().unwrap_or(0);
            let text = xhr.response_text().ok().flatten().unwrap_or_default();
            let compact: String = text.chars().filter(|c| !c.is_whitespace()).collect();
            let ok = (200..300).contains(&status)
                && (compact.contains("\"success\":\"true\"") || compact.contains("\"success\":true"));
            (*done)(ok);
        }) as Box<dyn Fn()>)
    };
    let on_fail = {
        let done = done.clone();
        wasm_bindgen::closure::Closure::wrap(Box::new(move || {
            (*done)(false);
        }) as Box<dyn Fn()>)
    };
    xhr.set_onload(Some(on_load.as_ref().unchecked_ref()));
    xhr.set_onerror(Some(on_fail.as_ref().unchecked_ref()));
    xhr.set_ontimeout(Some(on_fail.as_ref().unchecked_ref()));
    if xhr.send_with_opt_str(Some(body)).is_err() {
        (*done)(false);
    }
    on_load.forget();
    on_fail.forget();
}

#[component]
pub fn Contact() -> impl IntoView {
    let (name, set_name)       = signal(String::new());
    let (email, set_email)     = signal(String::new());
    let (message, set_message) = signal(String::new());
    let (honey, set_honey)     = signal(String::new()); // a field people never see: bots that fill every field fill it too
    let (status, set_status)   = signal(FormStatus::Idle);

    #[derive(Clone, PartialEq)]
    enum FormStatus { Idle, Sending, Success, Failed }

    let handle_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        if status.get() == FormStatus::Sending { return; }
        set_status.set(FormStatus::Sending);

        let body = format!(
            "name={}&email={}&message={}&_subject={}&_template=table&_captcha=false&_honey={}",
            encode(&name.get()),
            encode(&email.get()),
            encode(&message.get()),
            encode("New message from the portfolio site"),
            encode(&honey.get()),
        );

        post_form(CONTACT_ENDPOINT, &body, move |ok| {
            if !ok {
                set_status.set(FormStatus::Failed);
                return;
            }
            set_status.set(FormStatus::Success);
            set_name.set(String::new());
            set_email.set(String::new());
            set_message.set(String::new());

            // back to the form after a few seconds
            let reset = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
                set_status.set(FormStatus::Idle);
            }) as Box<dyn Fn()>);
            if let Some(window) = web_sys::window() {
                let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    reset.as_ref().unchecked_ref(), 4000,
                );
            }
            reset.forget();
        });
    };

    view! {
        <section id="contact" class="contact-section section">
            <div class="section-container">
                <div class="section-label">
                    <span class="label-line"></span>
                    <span>"04 — Contact"</span>
                </div>

                <h2 class="section-title reveal">
                    "Let's Build"<br/><em>"Something Real"</em>
                </h2>

                <div class="contact-grid">
                    // Left: Info + Socials
                    <div class="contact-info reveal-left">
                        <p class="contact-tagline">
                            "Have a game idea, a project that needs a developer, "
                            "or just want to say hi? I'm always open to interesting conversations."
                        </p>

                        <div class="contact-direct">
                            <a href="mailto:midmanstudio@proton.me" class="contact-email">
                                <IconMail class="contact-email-icon-svg" />
                                "midmanstudio@proton.me"
                            </a>
                        </div>

                        <div class="socials-block">
                            <h4 class="socials-heading">"Find Me Online"</h4>
                            <div class="socials-list">
                                <SocialLink
                                    href="https://github.com/mid-d-man"
                                    label="GitHub"
                                    handle="mid-d-man"
                                    icon_type="github"
                                />
                                <SocialLink
                                    href="https://t.me/midmanstudio"
                                    label="Telegram"
                                    handle="@midmanstudio"
                                    icon_type="telegram"
                                />
                                <SocialLink
                                    href="https://discord.gg/vtxVx4XNU"
                                    label="Discord"
                                    handle="discord.gg/vtxVx4XNU"
                                    icon_type="discord"
                                />
                                <SocialLink
                                    href="https://linkedin.com/company/MidManStudio"
                                    label="LinkedIn"
                                    handle="MidManStudio"
                                    icon_type="linkedin"
                                />
                                <SocialLink
                                    href="https://mid-d-man.github.io"
                                    label="Portfolio"
                                    handle="mid-d-man.github.io"
                                    icon_type="globe"
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
                                        <span class="success-icon-wrap">
                                            <IconCheck class="success-icon-svg" />
                                        </span>
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
                                        <div class="form-hp" aria-hidden="true">
                                            <label>"Leave this empty"</label>
                                            <input
                                                type="text"
                                                tabindex="-1"
                                                autocomplete="off"
                                                prop:value=move || honey.get()
                                                on:input=move |ev| set_honey.set(event_target_value(&ev))
                                            />
                                        </div>
                                        {move || (status.get() == FormStatus::Failed).then(|| view! {
                                            <p class="form-error-msg" role="alert">
                                                "That did not go through. Please try again, or email "
                                                <a href="mailto:midmanstudio@proton.me">"midmanstudio@proton.me"</a>
                                                " directly."
                                            </p>
                                        })}
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
            </div>
        </footer>
    }
}

#[component]
fn SocialLink(
    href: &'static str,
    label: &'static str,
    handle: &'static str,
    icon_type: &'static str,
) -> impl IntoView {
    let icon = match icon_type {
        "github"   => view! { <IconGitHub  class="social-icon-svg" /> }.into_any(),
        "linkedin" => view! { <IconLinkedIn class="social-icon-svg" /> }.into_any(),
        "telegram" => view! { <IconTelegram class="social-icon-svg" /> }.into_any(),
        "discord"  => view! { <IconDiscord  class="social-icon-svg" /> }.into_any(),
        _          => view! { <IconGlobe   class="social-icon-svg" /> }.into_any(),
    };

    view! {
        <a href=href target="_blank" rel="noopener noreferrer" class="social-link">
            <span class="social-icon-wrap">{icon}</span>
            <div class="social-text">
                <span class="social-label">{label}</span>
                <span class="social-handle">{handle}</span>
            </div>
            <span class="social-arrow">"↗"</span>
        </a>
    }
                                        }
