use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq)]
struct ProjectInfo {
    id: usize,
    title: String,
    description: String,
    technologies: Vec<String>,
    category: String,
}

#[component]
pub fn SearchBar(query: ReadSignal<String>, set_query: WriteSignal<String>) -> impl IntoView {
    view! {
        <div class="project-search-wrapper">
            <span class="project-search-icon">"⌕"</span>

            <input
                class="project-search-input"
                type="text"
                placeholder="Explore my work and skills..."
                prop:value=query
                on:input:target=move |ev| {
                    set_query.set(ev.target().value());
                }
            />
        </div>
    }
}

#[component]
pub fn Project() -> impl IntoView {
    // Dummy data untuk web portfolio
    let projects = vec![
        ProjectInfo {
            id: 1,
            title: "Point Of Sale System".to_string(),
            description: "A simple point of sale application built with standard HTML, CSS, JavaScript, and Express for simplified development.".to_string(),
            technologies: vec!["HTML".to_string(), "CSS".to_string(), "JavaScript".to_string(), "Express".to_string()],
            category: "WebApp".to_string(),
        },
        ProjectInfo {
            id: 2,
            title: "Persuratan Disporapar".to_string(),
            description: "A simple document management system for Disporapar.".to_string(),
            technologies: vec!["Python".to_string(), "React".to_string(), "Node.js".to_string()],
            category: "WebApp".to_string(),
        },
        ProjectInfo {
            id: 3,
            title: "EcoTracker Mobile App".to_string(),
            description: "A cross-platform mobile application to track carbon footprints, set green challenges, and log daily eco-friendly activities.".to_string(),
            technologies: vec!["Flutter".to_string(), "Dart".to_string(), "Firebase".to_string()],
            category: "MobileApp".to_string(),
        },
        ProjectInfo {
            id: 4,
            title: "Rust Web Compiler".to_string(),
            description: "A secure, in-browser online compiler and playground for Rust code built using WebAssembly, Leptos, and isolated sandboxed execution.".to_string(),
            technologies: vec!["Rust".to_string(), "Leptos".to_string(), "WebAssembly".to_string()],
            category: "WebApp".to_string(),
        },
    ];

    let (search_query, set_search_query) = signal(String::new());

    // Logika Filter
    let filtered_projects = move || {
        // Ubah query pencarian menjadi huruf kecil agar case-insensitive
        let q = search_query.get().to_lowercase();

        projects
            .clone()
            .into_iter()
            .filter(|p| {
                // Cek apakah judul atau deskripsinya mengandung kata yang diketik
                q.is_empty()
                    || p.title.to_lowercase().contains(&q)
                    || p.description.to_lowercase().contains(&q)
                    || p.technologies
                        .iter()
                        .any(|tech| tech.to_lowercase().contains(&q))
                    || p.category.to_lowercase().contains(&q)
            })
            .collect::<Vec<_>>()
    };

    view! {
        <div class="project-container" id="projects">
            // Lempar signal ke SearchBar
            <div class="project-search">
                <SearchBar query=search_query set_query=set_search_query />
            </div>

            // Tampilkan hasil filter menggunakan for loop dari leptos
            // class:searching=move untuk menambahkan class jika ada query pencarian
            <div class="project-list" class:searching=move || !search_query.get().is_empty()>
                <For
                    each=filtered_projects
                    key=|proj| proj.id
                    children=move |proj| {
                        let techs = proj.technologies.clone();
                        view! {
                            <div class="project-card">
                                <h3>{proj.title.clone()}</h3>
                                <p>{proj.description.clone()}</p>
                                <div class="project-technologies">
                                    {techs.into_iter().map(|tech| {
                                        view! { <span class="project-technologies-item">{tech}</span> }
                                    }).collect_view()}
                                </div>
                                <div class="project-category">{proj.category.clone()}</div>
                            </div>
                        }
                    }
                />
            </div>
        </div>
    }
}
