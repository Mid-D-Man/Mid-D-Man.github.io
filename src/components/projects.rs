use leptos::prelude::*;

#[derive(Clone)]
struct Project {
    title: &'static str,
    description: &'static str,
    tech_stack: Vec<&'static str>,
    live_url: Option<&'static str>,
    github_url: Option<&'static str>,
    featured: bool,
}

#[component]
pub fn Projects() -> impl IntoView {
    let projects = vec![
        Project {
            title: "AirCode Platform",
            description: "Higher institution QR-Code based attendance tracking application built with C# and Blazor.",
            tech_stack: vec!["C#", "Blazor", "QR-Code"],
            live_url: Some("https://mid-d-man.github.io/AirCode/"),
            github_url: Some("https://github.com/mid-d-man/AirCode"),
            featured: true,
        },
        Project {
            title: "Unity Game Portfolio",
            description: "Collection of Unity games and interactive experiences showcasing game development expertise.",
            tech_stack: vec!["Unity", "C#", "Game Design"],
            live_url: None,
            github_url: Some("https://github.com/mid-d-man"),
            featured: false,
        },
        Project {
            title: "MidManStudio Portfolio",
            description: "This very portfolio website built with Rust and Leptos, showcasing modern web technologies.",
            tech_stack: vec!["Rust", "Leptos", "WASM"],
            live_url: Some("https://mid-d-man.github.io/midmanstudio/"),
            github_url: Some("https://github.com/mid-d-man/midmanstudio"),
            featured: false,
        },
    ];

    view! {
        <section id="projects" class="projects">
            <div class="container">
                <h2 class="section-title">"Featured Projects"</h2>
                <div class="projects-grid">
                    {projects.into_iter().map(|project| {
                        view! {
                            <div class="project-card" class:featured=project.featured>
                                <div class="project-image">
                                    <div class="project-overlay">
                                        <div class="project-links">
                                            {if let Some(live_url) = project.live_url {
                                                view! {
                                                    <a href=live_url class="project-link" target="_blank" rel="noopener">
                                                        "Live Demo"
                                                    </a>
                                                }.into_view()
                                            } else {
                                                view! { <span></span> }.into_view()
                                            }}
                                            {if let Some(github_url) = project.github_url {
                                                view! {
                                                    <a href=github_url class="project-link" target="_blank" rel="noopener">
                                                        "GitHub"
                                                    </a>
                                                }.into_view()
                                            } else {
                                                view! { <span></span> }.into_view()
                                            }}
                                        </div>
                                    </div>
                                </div>
                                <div class="project-info">
                                    <h3>{project.title}</h3>
                                    <p>{project.description}</p>
                                    <div class="project-tech">
                                        {project.tech_stack.into_iter().map(|tech| {
                                            view! {
                                                <span class="tech-tag">{tech}</span>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                </div>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </section>
    }
        }
