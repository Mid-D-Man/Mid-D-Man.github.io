// =============================================================================
// src/components/portfolio/about.rs
use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <section id="about" class="about-section section">
            <div class="section-container">
                <div class="section-label">
                    <span class="label-line"></span>
                    <span>"01 — About"</span>
                </div>

                <div class="about-grid">
                    // Left: Text
                    <div class="about-text reveal-left">
                        <h2 class="section-title">
                            "The Person"
                            <br/>
                            <em>"Behind the Studio"</em>
                        </h2>

                        <p class="about-para">
                            "I'm Abdulhamid — known online as Mid-D-Man. I'm a game developer, "
                            "software engineer, and digital artist based in Nigeria. I founded "
                            "MidManStudio to create experiences that don't just work — they resonate."
                        </p>

                        <p class="about-para">
                            "My journey started with Unity and C#, building games that tell stories. "
                            "That curiosity expanded into web development, systems programming with "
                            "Rust, and the wild frontier of compiling to WebAssembly. I love working "
                            "at the edges of what technology can do."
                        </p>

                        <div class="about-details">
                            <div class="detail-item">
                                <span class="detail-icon">"📍"</span>
                                <span>"Nigeria"</span>
                            </div>
                            <div class="detail-item">
                                <span class="detail-icon">"🎮"</span>
                                <span>"Game Dev — Primary Focus"</span>
                            </div>
                            <div class="detail-item">
                                <span class="detail-icon">"⚡"</span>
                                <span>"Open to Collaborations"</span>
                            </div>
                        </div>
                    </div>

                    // Right: Skills + Stats
                    <div class="about-skills-panel reveal-right">
                        // Stats row
                        <div class="stats-row">
                            <div class="stat-card">
                                <span class="stat-number">"4+"</span>
                                <span class="stat-label">"Years Building"</span>
                            </div>
                            <div class="stat-card">
                                <span class="stat-number">"10+"</span>
                                <span class="stat-label">"Projects Shipped"</span>
                            </div>
                            <div class="stat-card">
                                <span class="stat-number">"5"</span>
                                <span class="stat-label">"Tech Stacks"</span>
                            </div>
                        </div>

                        // Skills
                        <div class="skills-block">
                            <h3 class="skills-heading">"Core Stack"</h3>
                            <div class="skills-grid">
                                <SkillPill label="Unity" level=95 />
                                <SkillPill label="C#" level=92 />
                                <SkillPill label="Rust" level=75 />
                                <SkillPill label="Leptos / WASM" level=72 />
                                <SkillPill label="Blazor / .NET" level=80 />
                                <SkillPill label="JavaScript" level=78 />
                                <SkillPill label="Digital Art" level=85 />
                                <SkillPill label="UI / UX" level=80 />
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn SkillPill(label: &'static str, level: u32) -> impl IntoView {
    view! {
        <div class="skill-pill">
            <div class="skill-pill-top">
                <span class="skill-name">{label}</span>
                <span class="skill-pct">{level}"%"</span>
            </div>
            <div class="skill-bar-track">
                <div
                    class="skill-bar-fill"
                    style=format!("width: {}%", level)
                ></div>
            </div>
        </div>
    }
  }
