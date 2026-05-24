use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Meta, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::component::contact::Contact;
use crate::component::hero::Hero;
use crate::component::navbar::Navbar;
use crate::component::project::Project;
use crate::component::skills::Skills;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/web-danar-q-ssr.css"/>

        // sets the document title
        <Title text="Danar Qusyairi | Portfolio"/>

        // sets other SEO tags
        <Meta name="description" content="Portfolio of Danar Qusyairi - Junior Software Engineer specializing in Web and Mobile development with Rust, JavaScript, and Flutter."/>
        <Meta name="author" content="Danar Qusyairi"/>
        <Meta name="keywords" content="Danar Qusyairi, Portfolio, Rust, Leptos, Developer, Software Engineer, Web Development, Flutter"/>
        <Meta property="og:type" content="website"/>
        <Meta property="og:title" content="Danar Qusyairi | Portfolio"/>
        <Meta property="og:description" content="Junior Software Engineer passionate about building efficient, scalable applications."/>
        <Meta property="twitter:card" content="summary_large_image"/>
        <Meta property="twitter:title" content="Danar Qusyairi | Portfolio"/>
        <Meta property="twitter:description" content="Junior Software Engineer passionate about building efficient, scalable applications."/>

        // content for this welcome page
        <Router>
            <Routes fallback=|| "Page not found.".into_view()>
                <Route path=StaticSegment("") view=HomePage/>
            </Routes>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <main>
            <Navbar/>
            <Hero/>
            <Project/>
            <Skills/>
            <Contact/>
        </main>
    }
}
