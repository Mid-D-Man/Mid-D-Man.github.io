use leptos::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <section id="about" class="about">
            <div class="container">
                <div class="about-content">
                    <div class="about-text">
                        <h2 class="section-title">"About MidManStudio"</h2>
                        <p>
                            "We are passionate game developers and digital creators who believe in bridging the gap between vision and reality. 
                            Our primary focus is game development, where we create immersive experiences that captivate and engage players."
                        </p>
                        <p>
                            "Beyond games, we excel in web development, software solutions, and digital art, always pushing the boundaries 
                            of what's possible with technology and creativity."
                        </p>
                        <div class="stats">
                            <div class="stat">
                                <span class="stat-number">"95%"</span>
                                <span class="stat-label">"Game Development"</span>
                            </div>
                            <div class="stat">
                                <span class="stat-number">"88%"</span>
                                <span class="stat-label">"Software Engineering"</span>
                            </div>
                            <div class="stat">
                                <span class="stat-number">"85%"</span>
                                <span class="stat-label">"Digital Art"</span>
                            </div>
                        </div>
                    </div>
                    <div class="about-visual">
                        <div class="tech-orbit">
                            <div class="tech-item unity">"Unity"</div>
                            <div class="tech-item csharp">"C#"</div>
                            <div class="tech-item blazor">"Rust"</div>
                            <div class="tech-item js">"Leptos"</div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
  }
