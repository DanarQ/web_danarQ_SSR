use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Meta, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use crate::component::contact::Contact;
use crate::component::hero::Hero;
use crate::component::navbar::Navbar;
use crate::component::project::Project;
use crate::component::project_detail::ProjectDetail;
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
                <script>
                    "try {
                        var saved = localStorage.getItem('dark-mode');
                        var isDark = false;
                        if (saved === 'true') {
                            isDark = true;
                        } else if (saved === null) {
                            if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
                                isDark = true;
                            }
                        }
                        if (isDark) {
                            document.documentElement.classList.add('dark');
                        } else {
                            document.documentElement.classList.remove('dark');
                        }
                    } catch (_) {}"
                </script>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[derive(Clone, Copy, Debug)]
pub struct DarkMode(pub RwSignal<bool>);

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let dark_mode = RwSignal::new(false);
    provide_context(DarkMode(dark_mode));

    // Effect to run on hydration / client startup to sync current state
    #[cfg(target_arch = "wasm32")]
    {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let html_element = document.document_element().unwrap();
        
        let first_run = std::cell::Cell::new(true);
        Effect::new(move |_| {
            let dark = dark_mode.get();
            
            if first_run.get() {
                first_run.set(false);
                let is_dark = html_element.class_list().contains("dark");
                if is_dark != dark {
                    dark_mode.set(is_dark);
                }
            } else {
                if let Ok(Some(storage)) = window.local_storage() {
                    let _ = storage.set_item("dark-mode", if dark { "true" } else { "false" });
                }
                
                if dark {
                    let _ = html_element.class_list().add_1("dark");
                } else {
                    let _ = html_element.class_list().remove_1("dark");
                }
            }
        });
    }

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
                <Route path=path!("/") view=HomePage/>
                <Route path=path!("/project/:id") view=ProjectDetail/>
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
