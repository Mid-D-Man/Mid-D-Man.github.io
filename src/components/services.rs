use leptos::prelude::*;

#[derive(Clone)]
struct Service {
    icon: &'static str,
    title: &'static str,
    description: &'static str,
    tech: &'static str,
    primary: bool,
}

#[component]
pub fn Services() -> impl IntoView {
    let services = vec![
        Service {
            icon: "🎮",
            title: "Game Development",
            description: "Creating immersive gaming experiences with Unity and cutting-edge technology. From concept to launch, we bring your game vision to life.",
            tech: "Unity • C# • Game Design",
            primary: true,
        },
        Service {
            icon: "🌐",
            title: "Web Development", 
            description: "Building responsive, interactive web applications and platforms that engage users and drive results.",
            tech: "Blazor • JavaScript • React",
            primary: false,
        },
        Service {
            icon: "💻",
            title: "Software Development",
            description: "Developing robust desktop and mobile applications with modern frameworks and best practices.",
            tech: "C# • .NET • Cross-Platform", 
            primary: false,
        },
        Service {
            icon: "🎨",
            title: "Digital Art & Design",
            description: "Crafting stunning visuals, game assets, and user interfaces that captivate and inspire.",
            tech: "UI/UX • Game Assets • Branding",
            primary: false,
        },
    ];

    view! {
        <section id="services" class="services">
            <div class="container">
                <h2 class="section-title">"Our Expertise"</h2>
                <div class="services-grid">
                    {services.into_iter().map(|service| {
                        view! {
                            <div class="service-card" class:primary=service.primary>
                                <div class="service-icon">{service.icon}</div>
                                <h3>{service.title}</h3>
                                <p>{service.description}</p>
                                <div class="service-tech">{service.tech}</div>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </section>
    }
}
