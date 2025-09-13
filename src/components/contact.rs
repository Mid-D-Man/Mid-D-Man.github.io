use leptos::*;

#[component]
pub fn Contact() -> impl IntoView {
    let (name, set_name) = create_signal(String::new());
    let (email, set_email) = create_signal(String::new());
    let (service, set_service) = create_signal(String::new());
    let (message, set_message) = create_signal(String::new());
    let (is_submitting, set_is_submitting) = create_signal(false);
    let (submit_status, set_submit_status) = create_signal(None::<String>);

    let handle_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        
        set_is_submitting.set(true);
        set_submit_status.set(Some("Sending message...".to_string()));
        
        // Simulate form submission
        set_timeout(
            move || {
                set_submit_status.set(Some("✓ Message Sent Successfully!".to_string()));
                set_is_submitting.set(false);
                
                // Reset form after success
                set_timeout(
                    move || {
                        set_name.set(String::new());
                        set_email.set(String::new());
                        set_service.set(String::new());
                        set_message.set(String::new());
                        set_submit_status.set(None);
                    },
                    std::time::Duration::from_secs(3),
                );
            },
            std::time::Duration::from_secs(2),
        );
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
