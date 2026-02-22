// =============================================================================
// src/components/portfolio/hero.rs
use leptos::prelude::*;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <section id="hero" class="hero-section">
            // Animated background grid
            <div class="hero-bg">
                <div class="grid-lines"></div>
                <div class="hero-glow-orb orb-blue"></div>
                <div class="hero-glow-orb orb-crimson"></div>
            </div>

            <div class="hero-content">
                // Eyebrow label
                <div class="hero-eyebrow">
                    <span class="eyebrow-dot"></span>
                    <span>"Game Developer  ·  Digital Artist  ·  Builder"</span>
                </div>

                // Main headline
                <h1 class="hero-headline">
                    <span class="hero-name">"Mid-D-Man"</span>
                    <br />
                    <span class="hero-sub">"Bridging Vision"</span>
                    <br />
                    <span class="hero-sub hero-sub-accent">"& Reality"</span>
                </h1>

                // Studio badge
                <div class="hero-studio-badge">
                    <span class="badge-label">"Founder —"</span>
                    <span class="badge-name">"MidManStudio"</span>
                </div>

                // Description
                <p class="hero-description">
                    "Crafting immersive games, sleek web experiences, and digital art "
                    "that push what's possible. Every project is a bridge between "
                    "imagination and execution."
                </p>

                // CTA row
                <div class="hero-cta-row">
                    <a href="#projects" class="btn btn-primary">"View My Work"</a>
                    <a href="#contact" class="btn btn-ghost">"Get In Touch"</a>
                </div>

                // Scroll hint
                <div class="hero-scroll-hint">
                    <div class="scroll-dot-track">
                        <div class="scroll-dot"></div>
                    </div>
                    <span>"Scroll"</span>
                </div>
            </div>

            // Floating tech tags
            <div class="hero-float-tags" aria-hidden="true">
                <span class="float-tag tag-1">"Unity"</span>
                <span class="float-tag tag-2">"Rust"</span>
                <span class="float-tag tag-3">"C#"</span>
                <span class="float-tag tag-4">"Leptos"</span>
                <span class="float-tag tag-5">"WASM"</span>
                <span class="float-tag tag-6">"Blazor"</span>
            </div>
        </section>
    }
  }
