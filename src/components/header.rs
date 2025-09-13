use leptos::*;

#[component]
pub fn Header() -> impl IntoView {
    let (nav_open, set_nav_open) = create_signal(false);
    
    view! {
        <nav class="navbar" class:scrolled=move || false>
            <div class="nav-container">
                <div class="nav-logo">
                    <span class="logo-text">"MidManStudio"</span>
                    <span class="logo-tagline">"Vision → Reality"</span>
                </div>
                <div class="nav-menu" class:active=nav_open>
                    <a href="#home" class="nav-link">"Home"</a>
                    <a href="#services" class="nav-link">"Services"</a>
                    <a href="#projects" class="nav-link">"Projects"</a>
                    <a href="#about" class="nav-link">"About"</a>
                    <a href="#contact" class="nav-link">"Contact"</a>
                </div>
                <div 
                    class="nav-toggle" 
                    class:active=nav_open
                    on:click=move |_| set_nav_open.update(|open| *open = !*open)
                >
                    <span class="bar"></span>
                    <span class="bar"></span>
                    <span class="bar"></span>
                </div>
            </div>
        </nav>
    }
      }
