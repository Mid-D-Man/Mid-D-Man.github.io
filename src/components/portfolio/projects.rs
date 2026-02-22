// =============================================================================
// src/components/portfolio/projects.rs
use leptos::prelude::*;

struct ProjectData {
    number: &'static str,
    title: &'static str,
    subtitle: &'static str,
    description: &'static str,
    tech: &'static [&'static str],
    link: &'static str,
    link_label: &'static str,
    featured: bool,
    category: &'static str,
}

#[component]
pub fn Projects() -> impl IntoView {
    let projects = vec![
        ProjectData {
            number: "01",
            title: "AirCode Platform",
            subtitle: "Attendance & QR System",
            description: "A higher-institution attendance tracking platform using QR codes for fast, tamper-resistant check-ins. Built with Blazor WebAssembly and Firebase, with real-time dashboards for lecturers and admins.",
            tech: &["C#", "Blazor", "Firebase", "QR Code", "WebAssembly"],
            link: "https://mid-d-man.github.io/AirCode/",
            link_label: "Live Demo",
            featured: true,
            category: "Web App",
        },
        ProjectData {
            number: "02",
            title: "MidManStudio Portfolio",
            subtitle: "This Site",
            description: "A portfolio site built entirely in Rust — compiled to WebAssembly using the Leptos framework, deployed to GitHub Pages via CI/CD. A technical experiment that became a production showcase.",
            tech: &["Rust", "Leptos 0.8", "WASM", "Trunk", "GitHub Actions"],
            link: "https://github.com/mid-d-man/Mid-D-Man.github.io",
            link_label: "Source Code",
            featured: true,
            category: "Web Dev",
        },
        ProjectData {
            number: "03",
            title: "Unity Game Projects",
            subtitle: "Interactive Experiences",
            description: "A collection of Unity-powered games and interactive demos — ranging from mobile casual games to prototype explorations of game mechanics. Game design, art, and code all in one.",
            tech: &["Unity", "C#", "Game Design", "Mobile", "WebGL"],
            link: "https://github.com/mid-d-man",
            link_label: "GitHub",
            featured: false,
            category: "Game Dev",
        },
        ProjectData {
            number: "04",
            title: "System Utility Tools",
            subtitle: "Low-Level Development",
            description: "Experiments in systems programming — including x86-64 assembly routines, native system info tools, and low-level performance utilities. Exploring the metal beneath the frameworks.",
            tech: &["Rust", "Assembly (x86-64)", "C", "Linux", "NASM"],
            link: "https://github.com/mid-d-man",
            link_label: "GitHub",
            featured: false,
            category: "Systems",
        },
    ];

    view! {
        <section id="projects" class="projects-section section">
            <div class="section-container">
                <div class="section-label">
                    <span class="label-line"></span>
                    <span>"03 — Projects"</span>
                </div>

                <h2 class="section-title reveal">"Selected Work"</h2>

                <div class="projects-list">
                    {projects.into_iter().enumerate().map(|(i, p)| {
                        view! {
                            <div class=format!(
                                "project-card {} reveal-up delay-{}",
                                if p.featured { "project-card--featured" } else { "" },
                                i
                            )>
                                <div class="project-header">
                                    <div class="project-meta">
                                        <span class="project-number">{p.number}</span>
                                        <span class="project-category">{p.category}</span>
                                        {if p.featured {
                                            view! { <span class="project-badge">"Featured"</span> }.into_any()
                                        } else {
                                            view! { <span></span> }.into_any()
                                        }}
                                    </div>
                                    <a
                                        href=p.link
                                        target="_blank"
                                        rel="noopener noreferrer"
                                        class="project-link-btn"
                                    >
                                        {p.link_label}
                                        <span class="link-arrow">"↗"</span>
                                    </a>
                                </div>

                                <div class="project-body">
                                    <h3 class="project-title">{p.title}</h3>
                                    <p class="project-subtitle">{p.subtitle}</p>
                                    <p class="project-desc">{p.description}</p>
                                </div>

                                <div class="project-footer">
                                    <div class="project-tech">
                                        {p.tech.iter().map(|t| view! {
                                            <span class="tech-chip">{*t}</span>
                                        }).collect_view()}
                                    </div>
                                </div>
                            </div>
                        }
                    }).collect_view()}
                </div>

                <div class="projects-more reveal">
                    <a
                        href="https://github.com/mid-d-man"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="btn btn-ghost"
                    >
                        "View All on GitHub  ↗"
                    </a>
                </div>
            </div>
        </section>
    }
          }
