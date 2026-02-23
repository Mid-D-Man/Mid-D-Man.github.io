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
    status: ProjectStatus,
    category: &'static str,
}

#[derive(PartialEq, Clone, Copy)]
enum ProjectStatus {
    Featured,
    InProgress,
    Completed,
}

#[component]
pub fn Projects() -> impl IntoView {
    let projects = vec![
        ProjectData {
            number: "01",
            title: "AirCode Platform",
            subtitle: "Attendance & QR System",
            description: "Higher-institution attendance tracking via QR codes. Real-time dashboards for lecturers and admins, tamper-resistant check-ins, built with Blazor WebAssembly and Firebase.",
            tech: &["C#", "Blazor", "Firebase", "QR Code", "WebAssembly"],
            link: "https://mid-d-man.github.io/AirCode/",
            link_label: "Live Demo",
            status: ProjectStatus::Featured,
            category: "Web App",
        },
        ProjectData {
            number: "02",
            title: "MBFA",
            subtitle: "MidMan's Bit Folding Algorithm",
            description: "A novel multi-fold iterative compression algorithm under active research. Instead of a single pass, MBFA folds data through multiple passes — each fold sees a structurally different layer than the last. Already beating gzip on highly repetitive data. Entropy coding in progress.",
            tech: &["Rust", "Compression", "Bit Manipulation", "Algorithms", "Research"],
            link: "https://github.com/Mid-D-Man/mbfa",
            link_label: "GitHub",
            status: ProjectStatus::InProgress,
            category: "Systems / Research",
        },
        ProjectData {
            number: "03",
            title: "DixScript",
            subtitle: "Config · Code · Crypto in One Format",
            description: "A data interchange format that combines config, compile-time functions, built-in AES-256 encryption, and automatic compression — all in one .mdix file. Cut an 800-line JSON config to 240 lines. C# v1.0 shipped; Rust port in active development.",
            tech: &["Rust", "C#", "Language Design", "Parser", "Compiler"],
            link: "https://github.com/Mid-D-Man/DixScript-Rust",
            link_label: "GitHub",
            status: ProjectStatus::InProgress,
            category: "Language / Tooling",
        },
        ProjectData {
            number: "04",
            title: "MidManStudio Portfolio",
            subtitle: "This Site",
            description: "Built entirely in Rust — compiled to WebAssembly with Leptos 0.8, deployed via GitHub Actions. A real production site, not a demo.",
            tech: &["Rust", "Leptos 0.8", "WASM", "Trunk", "GitHub Actions"],
            link: "https://github.com/mid-d-man/Mid-D-Man.github.io",
            link_label: "Source",
            status: ProjectStatus::Featured,
            category: "Web Dev",
        },
        ProjectData {
            number: "05",
            title: "Unity Game Projects",
            subtitle: "Interactive Experiences",
            description: "A collection of Unity-powered games and interactive prototypes — from mobile casual to WebGL. Game design, art, and code all in one.",
            tech: &["Unity", "C#", "Game Design", "Mobile", "WebGL"],
            link: "https://github.com/mid-d-man",
            link_label: "GitHub",
            status: ProjectStatus::Completed,
            category: "Game Dev",
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

                // Legend
                <div class="projects-legend reveal">
                    <span class="legend-item">
                        <span class="status-dot dot-featured"></span>"Featured"
                    </span>
                    <span class="legend-item">
                        <span class="status-dot dot-progress"></span>"In Progress"
                    </span>
                    <span class="legend-item">
                        <span class="status-dot dot-done"></span>"Completed"
                    </span>
                </div>

                <div class="projects-list">
                    {projects.into_iter().enumerate().map(|(i, p)| {
                        let status_class = match p.status {
                            ProjectStatus::Featured  => "project-card project-card--featured",
                            ProjectStatus::InProgress => "project-card project-card--progress",
                            ProjectStatus::Completed  => "project-card",
                        };
                        let badge_label = match p.status {
                            ProjectStatus::Featured   => Some("Featured"),
                            ProjectStatus::InProgress => Some("In Progress"),
                            ProjectStatus::Completed  => None,
                        };
                        let badge_class = match p.status {
                            ProjectStatus::Featured   => "project-badge badge--featured",
                            ProjectStatus::InProgress => "project-badge badge--progress",
                            ProjectStatus::Completed  => "",
                        };

                        view! {
                            <div class=format!("{} reveal-up delay-{}", status_class, i % 4)>
                                <div class="project-header">
                                    <div class="project-meta">
                                        <span class="project-number">{p.number}</span>
                                        <span class="project-category">{p.category}</span>
                                        {badge_label.map(|label| view! {
                                            <span class=badge_class>{label}</span>
                                        })}
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
