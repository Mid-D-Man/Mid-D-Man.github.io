use leptos::*;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <section id="home" class="hero">
            <div class="hero-content">
                <div class="hero-text">
                    <h1 class="hero-title">
                        <span class="title-line">"Welcome to"</span>
                        <span class="title-main">"MidManStudio"</span>
                        <span class="title-sub">"Bridging the Gap Between Vision and Reality"</span>
                    </h1>
                    <p class="hero-description">
                        "We specialize in game development, creating immersive experiences with Unity. 
                        From interactive games to web applications and digital art, we transform your wildest ideas into reality."
                    </p>
                    <div class="hero-buttons">
                        <a href="#projects" class="btn btn-primary">"View Our Games"</a>
                        <a href="#contact" class="btn btn-secondary">"Start a Project"</a>
                    </div>
                </div>
                <div class="hero-visual">
                    <div class="floating-icon game-controller"></div>
                    <div class="floating-icon code-bracket"></div>
                    <div class="floating-icon design-brush"></div>
                </div>
            </div>
            <div class="scroll-indicator">
                <div class="scroll-arrow"></div>
            </div>
        </section>
    }
}
