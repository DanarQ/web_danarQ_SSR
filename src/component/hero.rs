use leptos::prelude::*;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <div class="hero" id="about">
            <h1>"Danar Qusyairi"</h1>
            <div class="hero-medium-text">"Junior Software Engineer"</div>
            <p>"Passionate software engineer experienced in web development with Rust and JavaScript, focused on building efficient, scalable applications that solve real-world problems."</p>
        </div>
    }
}
