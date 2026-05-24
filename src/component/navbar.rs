use leptos::prelude::*;
use crate::app::DarkMode;
use leptos_icons::Icon;

#[component]
pub fn Navbar() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);
    let DarkMode(dark_mode) = use_context::<DarkMode>().expect("DarkMode context missing");

    let toggle_theme = move |_| dark_mode.update(|d| *d = !*d);

    let theme_btn_view = move || {
        view! {
            <button class="theme-toggle-btn" on:click=toggle_theme aria-label="Toggle theme">
                {move || if dark_mode.get() {
                    view! {
                        <Icon icon=icondata::FiSun attr:class="theme-icon sun-icon" />
                    }.into_any()
                } else {
                    view! {
                        <Icon icon=icondata::FiMoon attr:class="theme-icon moon-icon" />
                    }.into_any()
                }}
            </button>
        }
    };

    view! {
        <nav class="navbar">
            <div class="navbar-logo">
                <a href="#about">"Danar Qusyairi"</a>
            </div>

            // Mobile actions: theme toggle + hamburger menu
            <div class="navbar-mobile-actions">
                {theme_btn_view()}
                <button 
                    class="navbar-toggle" 
                    class:active=is_open
                    on:click=move |_| set_is_open.update(|v| *v = !*v) 
                    aria-label="Toggle menu"
                >
                    <span class="bar"></span>
                    <span class="bar"></span>
                    <span class="bar"></span>
                </button>
            </div>

            <ul class="navbar-menu" class:active=is_open>
                <li><a href="#about" on:click=move |_| set_is_open.set(false)>"About"</a></li>
                <li><a href="#projects" on:click=move |_| set_is_open.set(false)>"Projects"</a></li>
                <li><a href="#skills" on:click=move |_| set_is_open.set(false)>"Skills"</a></li>
                <li><a href="#contact" on:click=move |_| set_is_open.set(false)>"Contact"</a></li>
                <li class="menu-theme-toggle">
                    {theme_btn_view()}
                </li>
            </ul>
        </nav>
    }
}
