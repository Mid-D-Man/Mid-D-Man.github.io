use leptos::prelude::*;
// Import from crate root, not as external crate
use crate::App;

fn main() {
    console_error_panic_hook::set_once();
    
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
