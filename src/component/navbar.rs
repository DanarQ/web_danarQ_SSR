use leptos::prelude::*;

#[component]
pub fn Navbar() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);

    view! {
        <nav class="navbar">
            <div class="navbar-logo">
                <a href="#about">"Danar Qusyairi"</a>
            </div>

            // Responsive Hamburg Menu Toggle
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

            <ul class="navbar-menu" class:active=is_open>
                <li><a href="#about" on:click=move |_| set_is_open.set(false)>"About"</a></li>
                <li><a href="#projects" on:click=move |_| set_is_open.set(false)>"Projects"</a></li>
                <li><a href="#skills" on:click=move |_| set_is_open.set(false)>"Skills"</a></li>
                <li><a href="#contact" on:click=move |_| set_is_open.set(false)>"Contact"</a></li>
            </ul>
        </nav>
    }
}
