use leptos::prelude::*;
use leptos_router::components::A;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProjectInfo {
    pub id: usize,
    pub title: String,
    pub description: String,
    pub technologies: Vec<String>,
    pub category: String,
    pub long_description: String,
}

pub fn get_mock_projects() -> Vec<ProjectInfo> {
    vec![
        ProjectInfo {
            id: 1,
            title: "Point Of Sale System".to_string(),
            description: "A simple point of sale application built with standard HTML, CSS, JavaScript, and Express for simplified development.".to_string(),
            technologies: vec!["HTML".to_string(), "CSS".to_string(), "JavaScript".to_string(), "Express".to_string()],
            category: "WebApp".to_string(),
            long_description: "A comprehensive Point Of Sale system designed for small to medium retail businesses. It allows you to track sales, manage inventory in real-time, generate invoice receipts, and review daily sales analytics reports. Features client-side state caching for offline-resilience and local data synchronization.".to_string(),
        },
        ProjectInfo {
            id: 2,
            title: "Persuratan Disporapar".to_string(),
            description: "A simple document management system for Disporapar.".to_string(),
            technologies: vec!["Python".to_string(), "React".to_string(), "Node.js".to_string()],
            category: "WebApp".to_string(),
            long_description: "A secure document management and tracking system built for Disporapar. It simplifies incoming and outgoing mail administration, provides digital approval workflows, role-based access control, and full audit logging for government compliance.".to_string(),
        },
        ProjectInfo {
            id: 3,
            title: "EcoTracker Mobile App".to_string(),
            description: "A cross-platform mobile application to track carbon footprints, set green challenges, and log daily eco-friendly activities.".to_string(),
            technologies: vec!["Flutter".to_string(), "Dart".to_string(), "Firebase".to_string()],
            category: "MobileApp".to_string(),
            long_description: "EcoTracker is a mobile application dedicated to environmental awareness. It calculates individual carbon footprints based on transport, diet, and energy use. Users can join community challenges, earn achievements, and view visual analytics charts showing their carbon reduction progress.".to_string(),
        },
        ProjectInfo {
            id: 4,
            title: "Rust Web Compiler".to_string(),
            description: "A secure, in-browser online compiler and playground for Rust code built using WebAssembly, Leptos, and isolated sandboxed execution.".to_string(),
            technologies: vec!["Rust".to_string(), "Leptos".to_string(), "WebAssembly".to_string()],
            category: "WebApp".to_string(),
            long_description: "An interactive online coding platform for compiling and running Rust code directly in the browser. It compiles source files to WebAssembly binaries and runs them in a sandboxed, isolated environment. Ideal for learning, rapid prototyping, and sharing Rust code snippets.".to_string(),
        },
    ]
}

#[cfg(feature = "ssr")]
pub mod db {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "projects")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i64,
        pub title: String,
        pub description: String,
        pub technologies: Vec<String>,
        pub category: String,
        pub long_description: String,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

#[server(GetProjects, "/api")]
pub async fn get_projects() -> Result<Vec<ProjectInfo>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use sea_orm::EntityTrait;
        if let Some(conn) = use_context::<sea_orm::DatabaseConnection>() {
            match db::Entity::find().all(&conn).await {
                Ok(models) => {
                    let projects = models.into_iter().map(|m| ProjectInfo {
                        id: m.id as usize,
                        title: m.title,
                        description: m.description,
                        technologies: m.technologies,
                        category: m.category,
                        long_description: m.long_description,
                    }).collect::<Vec<_>>();
                    return Ok(projects);
                }
                Err(err) => {
                    leptos::logging::log!("Database query error: {:?}", err);
                }
            }
        }
        // Fallback
        Ok(get_mock_projects())
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(GetProjectById, "/api")]
pub async fn get_project_by_id(id: usize) -> Result<ProjectInfo, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use sea_orm::EntityTrait;
        if let Some(conn) = use_context::<sea_orm::DatabaseConnection>() {
            match db::Entity::find_by_id(id as i64).one(&conn).await {
                Ok(Some(m)) => {
                    return Ok(ProjectInfo {
                        id: m.id as usize,
                        title: m.title,
                        description: m.description,
                        technologies: m.technologies,
                        category: m.category,
                        long_description: m.long_description,
                    });
                }
                Ok(None) => {
                    return Err(ServerFnError::new("Project not found in database"));
                }
                Err(err) => {
                    leptos::logging::log!("Database query error: {:?}", err);
                }
            }
        }
        // Fallback
        get_mock_projects()
            .into_iter()
            .find(|p| p.id == id)
            .ok_or_else(|| ServerFnError::new("Project not found"))
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = id;
        unreachable!()
    }
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
    let (search_query, set_search_query) = signal(String::new());

    // Fetch projects from Server Function using Resource
    let projects_resource = Resource::new(|| (), |_| async move {
        get_projects().await.unwrap_or_default()
    });

    // Cache the lowercased strings so we don't allocate during the filter loop.
    // This retains the full Unicode case-folding semantics of `to_lowercase()`,
    // and allows us to use the standard library's highly optimized `contains()`.
    let lowercased_projects = Memo::new(move |_| {
        let current_projects = projects_resource.get().unwrap_or_default();
        current_projects.into_iter().map(|p| {
            let title = p.title.to_lowercase();
            let desc = p.description.to_lowercase();
            let techs = p.technologies.iter().map(|t| t.to_lowercase()).collect::<Vec<_>>();
            let cat = p.category.to_lowercase();
            (p, title, desc, techs, cat)
        }).collect::<Vec<_>>()
    });

    view! {
        <div class="project-container" id="projects">
            // Lempar signal ke SearchBar
            <div class="project-search">
                <SearchBar query=search_query set_query=set_search_query />
            </div>

            // Tampilkan hasil filter menggunakan for loop dari leptos
            <Suspense fallback=move || view! { <div class="project-loading">"Loading projects..."</div> }>
                {move || {
                    let search_q = search_query.get().to_lowercase();
                    // Track the resource to preserve the Suspense loading state
                    projects_resource.track();
                    let filtered = lowercased_projects.with(|cached| {
                        cached
                            .iter()
                            .filter(|(_, title, desc, techs, cat)| {
                                search_q.is_empty()
                                    || title.contains(&search_q)
                                    || desc.contains(&search_q)
                                    || techs.iter().any(|tech| tech.contains(&search_q))
                                    || cat.contains(&search_q)
                            })
                            .map(|(p, _, _, _, _)| p.clone())
                            .collect::<Vec<_>>()
                    });

                    view! {
                        <div class="project-list" class:searching=move || !search_query.get().is_empty()>
                            <For
                                each=move || filtered.clone()
                                key=|proj| proj.id
                                children=move |proj| {
                                    let techs = proj.technologies.clone();
                                    let details_url = format!("/project/{}", proj.id);
                                    let display_desc = if proj.description.chars().count() > 100 {
                                        let mut truncated: String = proj.description.chars().take(100).collect();
                                        truncated.push_str("...");
                                        truncated
                                    } else {
                                        proj.description.clone()
                                    };
                                    view! {
                                        <A href=details_url attr:class="project-card-link">
                                            <div class="project-card">
                                                <h3>{proj.title.clone()}</h3>
                                                <p>{display_desc}</p>
                                                <div class="project-technologies">
                                                    {techs.into_iter().map(|tech| {
                                                        view! { <span class="project-technologies-item">{tech}</span> }
                                                    }).collect_view()}
                                                </div>
                                                <div class="project-category">{proj.category.clone()}</div>
                                            </div>
                                        </A>
                                    }
                                }
                            />
                        </div>
                    }
                }}
            </Suspense>
        </div>
    }
}
