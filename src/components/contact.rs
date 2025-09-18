use leptos::prelude::*;
use wasm_bindgen::JsCast;

#[component]
pub fn Contact() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (email, set_email) = signal(String::new());
    let (service, set_service) = signal(String::new());
    let (message, set_message) = signal(String::new());
    let (is_submitting, set_is_submitting) = signal(false);
    let (submit_status, set_submit_status) = signal(None::<String>);

    let handle_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        
        set_is_submitting.set(true);
        set_submit_status.set(Some("Sending message...".to_string()));
        
        // Use setTimeout with web-sys instead of spawn_local
        let window = web_sys::window().unwrap();
        let closure = wasm_bindgen::closure::Closure::wrap(Box::new({
            let set_submit_status = set_submit_status.clone();
            let set_is_submitting = set_is_submitting.clone();
            let set_name = set_name.clone();
            let set_email = set_email.clone();
            let set_service = set_service.clone();
            let set_message = set_message.clone();
            
            move || {
                set_submit_status.set(Some("✓ Message Sent Successfully!".to_string()));
                set_is_submitting.set(false);
                
                // Reset form after 3 seconds
                let window = web_sys::window().unwrap();
                let reset_closure = wasm_bindgen::closure::Closure::wrap(Box::new({
                    let set_name = set_name.clone();
                    let set_email = set_email.clone();
                    let set_service = set_service.clone();
                    let set_message = set_message.clone();
                    let set_submit_status = set_submit_status.clone();
                    
                    move || {
                        set_name.set(String::new());
                        set_email.set(String::new());
                        set_service.set(String::new());
                        set_message.set(String::new());
                        set_submit_status.set(None);
                    }
                }) as Box<dyn Fn()>);
                
                let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    reset_closure.as_ref().unchecked_ref(),
                    3000,
                );
                reset_closure.forget();
            }
        }) as Box<dyn Fn()>);
        
        let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            2000,
        );
        closure.forget();
    };

    view! {
        <section id="contact" class="contact">
            <div class="container">
                <h2 class="section-title">"Let's Create Something Amazing"</h2>
                <p class="contact-subtitle">"Ready to bridge the gap between your vision and reality?"</p>
                <div class="contact-grid">
                    <div class="contact-info">
                        <div class="contact-item">
                            <div class="contact-icon">"📧"</div>
                            <div>
                                <h4>"Email"</h4>
                                <p>"contact@midmanstudio.com"</p>
                            </div>
                        </div>
                        <div class="contact-links">
                            <a href="https://github.com/mid-d-man" class="social-link" target="_blank" rel="noopener">
                                <span class="social-icon">"⚡"</span>
                                "GitHub"
                            </a>
                            <a href="https://linkedin.com/company/MidManStudio" class="social-link" target="_blank" rel="noopener">
                                <span class="social-icon">"💼"</span>
                                "LinkedIn"
                            </a>
                            <a href="https://mid-d-man.github.io" class="social-link" target="_blank" rel="noopener">
                                <span class="social-icon">"🎮"</span>
                                "Portfolio"
                            </a>
                        </div>
                    </div>
                    <div class="contact-form">
                        <form on:submit=handle_submit>
                            <div class="form-group">
                                <input
                                    type="text"
                                    placeholder="Your Name"
                                    required
                                    prop:value=name
                                    on:input=move |ev| set_name.set(event_target_value(&ev))
                                    prop:disabled=is_submitting
                                />
                            </div>
                            <div class="form-group">
                                <input
                                    type="email"
                                    placeholder="Your Email"
                                    required
                                    prop:value=email
                                    on:input=move |ev| set_email.set(event_target_value(&ev))
                                    prop:disabled=is_submitting
                                />
                            </div>
                            <div class="form-group">
                                <select
                                    required
                                    prop:value=service
                                    on:change=move |ev| set_service.set(event_target_value(&ev))
                                    prop:disabled=is_submitting
                                >
                                    <option value="">"Select Service"</option>
                                    <option value="game-dev">"Game Development"</option>
                                    <option value="web-dev">"Web Development"</option>
                                    <option value="software-dev">"Software Development"</option>
                                    <option value="digital-art">"Digital Art & Design"</option>
                                </select>
                            </div>
                            <div class="form-group">
                                <textarea
                                    placeholder="Tell us about your project..."
                                    rows="5"
                                    required
                                    prop:value=message
                                    on:input=move |ev| set_message.set(event_target_value(&ev))
                                    prop:disabled=is_submitting
                                ></textarea>
                            </div>
                            <button
                                type="submit"
                                class="btn btn-primary"
                                class:loading=is_submitting
                                prop:disabled=is_submitting
                            >
                                {move || match submit_status.get() {
                                    Some(status) => status,
                                    None => "Send Message".to_string(),
                                }}
                            </button>
                        </form>
                    </div>
                </div>
            </div>
        </section>
    }
        }
