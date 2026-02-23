// =============================================================================
// src/components/portfolio/services.rs
use leptos::prelude::*;
use crate::components::portfolio::icons::*;

// Service icon enum so we can pass them cleanly
#[derive(Clone, Copy)]
enum ServiceIcon { Gamepad, Globe, Terminal, Pen }

struct ServiceData {
    icon: ServiceIcon,
    title: &'static str,
    tagline: &'static str,
    description: &'static str,
    tags: &'static [&'static str],
    accent: &'static str,
}

#[component]
pub fn Services() -> impl IntoView {
    let services = vec![
        ServiceData {
            icon: ServiceIcon::Gamepad,
            title: "Game Development",
            tagline: "Primary Expertise",
            description: "End-to-end Unity game development — from concept and game design through to polished, shippable builds. Mobile, PC, and WebGL targets.",
            tags: &["Unity", "C#", "Game Design", "WebGL", "Mobile"],
            accent: "crimson",
        },
        ServiceData {
            icon: ServiceIcon::Globe,
            title: "Web Development",
            tagline: "Full-Stack",
            description: "Modern web applications with a unique edge — including Rust/WASM frontends with Leptos, .NET Blazor, and JavaScript frameworks.",
            tags: &["Rust", "Leptos", "Blazor", "JavaScript", "WASM"],
            accent: "blue",
        },
        ServiceData {
            icon: ServiceIcon::Terminal,
            title: "Software Development",
            tagline: "Systems & Apps",
            description: "Robust desktop and backend software. From attendance systems to productivity tools — reliable, well-architected, and maintainable.",
            tags: &["C#", ".NET", "Rust", "Firebase", "Supabase"],
            accent: "cyan",
        },
        ServiceData {
            icon: ServiceIcon::Pen,
            title: "Digital Art",
            tagline: "Visual Creation",
            description: "Game assets, concept art, UI design, and illustrations. Crafting visuals that serve the product — from quick icons to detailed character work.",
            tags: &["Krita", "MediBang", "Figma", "Blender", "Asset Design"],
            accent: "gold",
        },
    ];

    view! {
        <section id="services" class="services-section section">
            <div class="section-container">
                <div class="section-label">
                    <span class="label-line"></span>
                    <span>"02 — Services"</span>
                </div>

                <h2 class="section-title reveal">"What I Build"</h2>

                <div class="services-grid">
                    {services.into_iter().enumerate().map(|(i, s)| {
                        let icon_view = match s.icon {
                            ServiceIcon::Gamepad  => view! { <IconGamepad  class="service-icon-svg" /> }.into_any(),
                            ServiceIcon::Globe    => view! { <IconGlobe    class="service-icon-svg" /> }.into_any(),
                            ServiceIcon::Terminal => view! { <IconTerminal class="service-icon-svg" /> }.into_any(),
                            ServiceIcon::Pen      => view! { <IconPen      class="service-icon-svg" /> }.into_any(),
                        };
                        view! {
                            <div class=format!(
                                "service-card service-card--{} reveal-up delay-{}",
                                s.accent, i
                            )>
                                <div class="service-icon-wrap">{icon_view}</div>
                                <div class="service-tagline">{s.tagline}</div>
                                <h3 class="service-title">{s.title}</h3>
                                <p class="service-desc">{s.description}</p>
                                <div class="service-tags">
                                    {s.tags.iter().map(|t| view! {
                                        <span class="service-tag">{*t}</span>
                                    }).collect_view()}
                                </div>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>
        </section>
    }
                        }
