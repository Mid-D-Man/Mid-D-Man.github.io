// =============================================================================
// src/app.rs
use leptos::prelude::*;
use wasm_bindgen::JsCast; // Add this import
use crate::components::*;

#[component]
pub fn App() -> impl IntoView {
    // Scroll effect for parallax
    Effect::new(move |_| {
        if let Some(window) = web_sys::window() {
            let window_clone = window.clone();
            let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
                let scroll_y = window_clone.scroll_y().unwrap_or(0.0);
                
                // Apply parallax to background layers
                if let Some(document) = window_clone.document() {
                    // Background parallax (slowest)
                    if let Some(bg) = document.query_selector(".parallax-background").unwrap() {
                        let _ = bg.set_attribute("style", &format!(
                            "transform: translateY({}px)", 
                            scroll_y * 0.1
                        ));
                    }
                    
                    // Cityscape parallax (medium)
                    if let Some(city) = document.query_selector(".cityscape-layer").unwrap() {
                        let _ = city.set_attribute("style", &format!(
                            "transform: translateY({}px)", 
                            scroll_y * 0.3
                        ));
                    }
                }
            }) as Box<dyn Fn()>);
            
            let _ = window.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
            closure.forget();
        }
    });

    view! {
        <div class="app">
            // Background layers
            <Background />
            <Cityscape />
            
            // Main content rooms
            <main class="main-content">
                // Hero Room
                <Room 
                    config=RoomConfig {
                        window_config: WindowConfig::Panoramic,
                        wall_color: "#2c3e50".to_string(),
                        has_side_windows: true,
                        ..Default::default()
                    }
                    room_id="hero-room".to_string()
                >
                    <div class="hero-content">
                        <h1>"MidManStudio"</h1>
                        <p>"Bridging Vision and Reality"</p>
                        <p>"Game Development • Web Development • Digital Art"</p>
                    </div>
                </Room>
                
                <RoomSeparator has_decorations=true />
                
                // Services Room  
                <Room
                    config=RoomConfig {
                        window_config: WindowConfig::Multiple,
                        wall_color: "#34495e".to_string(),
                        floor_type: FloorType::Tech,
                        ..Default::default()
                    }
                    room_id="services-room".to_string()
                >
                    <div class="services-content">
                        <h2>"Our Services"</h2>
                        <div class="service-grid">
                            <div class="service">"🎮 Game Development"</div>
                            <div class="service">"🌐 Web Development"</div>
                            <div class="service">"💻 Software Development"</div>
                            <div class="service">"🎨 Digital Art"</div>
                        </div>
                    </div>
                </Room>
                
                <RoomSeparator />
                
                // Projects Room
                <Room
                    config=RoomConfig {
                        window_config: WindowConfig::Large,
                        wall_color: "#2c3e50".to_string(),
                        floor_type: FloorType::Wood,
                        ..Default::default()
                    }
                    room_id="projects-room".to_string()
                >
                    <div class="projects-content">
                        <h2>"Featured Projects"</h2>
                        <div class="project-showcase">
                            <div class="project">
                                <h3>"AirCode Platform"</h3>
                                <p>"QR-Code attendance system"</p>
                            </div>
                            <div class="project">
                                <h3>"Unity Games Portfolio"</h3>
                                <p>"Interactive gaming experiences"</p>
                            </div>
                        </div>
                    </div>
                </Room>
                
                <RoomSeparator />
                
                // About Room
                <Room
                    config=RoomConfig {
                        top_wall_type: TopWallType::SkyView,
                        window_config: WindowConfig::Small,
                        wall_color: "#34495e".to_string(),
                        floor_type: FloorType::Carpet,
                        ..Default::default()
                    }
                    room_id="about-room".to_string()
                >
                    <div class="about-content">
                        <h2>"About Us"</h2>
                        <p>"Passionate developers bridging vision and reality through innovative technology."</p>
                        <div class="skills">
                            <span class="skill">"Unity"</span>
                            <span class="skill">"C#"</span>
                            <span class="skill">"Rust"</span>
                            <span class="skill">"Leptos"</span>
                        </div>
                    </div>
                </Room>
                
                <RoomSeparator has_decorations=true />
            </main>
            
            // Contact footer
            <ContactFooter />
        </div>
    }
        }
