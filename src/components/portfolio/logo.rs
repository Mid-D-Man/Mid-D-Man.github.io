// =============================================================================
// src/components/portfolio/logo.rs
use leptos::prelude::*;

#[component]
pub fn LogoMark() -> impl IntoView {
    view! {
        <svg
            class="logo-mark"
            width="40" height="40"
            viewBox="0 0 40 40"
            xmlns="http://www.w3.org/2000/svg"
            aria-label="Mid-D-Man logo"
        >
            <defs>
                <linearGradient id="logo-grad" x1="0%" y1="0%" x2="100%" y2="100%">
                    <stop offset="0%" stop-color="#4169e1"/>
                    <stop offset="60%" stop-color="#0047ab"/>
                    <stop offset="100%" stop-color="#dc143c"/>
                </linearGradient>
                <filter id="logo-glow">
                    <feGaussianBlur stdDeviation="1.5" result="blur"/>
                    <feMerge>
                        <feMergeNode in="blur"/>
                        <feMergeNode in="SourceGraphic"/>
                    </feMerge>
                </filter>
            </defs>
            // Outer hexagon frame
            <polygon
                points="20,2 36,11 36,29 20,38 4,29 4,11"
                fill="none"
                stroke="url(#logo-grad)"
                stroke-width="1.5"
                opacity="0.6"
            />
            // M letterform
            <path
                d="M 9 29 L 9 11 L 20 22 L 31 11 L 31 29 M 9 11 L 20 22 L 31 11"
                fill="none"
                stroke="url(#logo-grad)"
                stroke-width="3"
                stroke-linecap="round"
                stroke-linejoin="round"
                filter="url(#logo-glow)"
            />
            // Crimson accent dot
            <circle cx="20" cy="22" r="2.5" fill="#dc143c" opacity="0.9"/>
        </svg>
    }
  }
