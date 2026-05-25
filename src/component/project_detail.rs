use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;
use crate::component::project::get_project_by_id;

#[derive(Params, PartialEq, Clone)]
pub struct ProjectParams {
    pub id: usize,
}

#[component]
pub fn ProjectDetail() -> impl IntoView {
    let params = use_params::<ProjectParams>();

    // We use a Resource that reacts to the parameter ID
    let project_resource = Resource::new(
        move || {
            params.get().ok().map(|p| p.id)
        },
        move |id_opt| async move {
            if let Some(id) = id_opt {
                get_project_by_id(id).await.ok()
            } else {
                None
            }
        }
    );

    view! {
        <div class="project-detail-container">
            <A href="/#projects" attr:class="back-btn">
                <span class="back-icon">"←"</span>
                <span>"Back to Portfolio"</span>
            </A>

            <Suspense fallback=move || view! { <div class="project-detail-loading">"Loading project details..."</div> }>
                {move || {
                    project_resource.get().flatten().map(|proj| {
                        let techs = proj.technologies.clone();
                        view! {
                            <div class="project-detail-layout">
                                // Left Column: Banner & Detailed Content
                                <div class="project-detail-main">
                                    {proj.image_url.as_ref().map(|url| {
                                        view! {
                                            <div class="project-detail-banner-container">
                                                <img src=url.clone() alt=proj.title.clone() class="project-detail-banner" />
                                            </div>
                                        }
                                    })}

                                    <article class="project-detail-content">
                                        <header class="project-detail-header-inner">
                                            <div class="project-detail-category">{proj.category.clone()}</div>
                                            <h1>{proj.title.clone()}</h1>
                                            <p class="project-detail-summary">{proj.description.clone()}</p>
                                        </header>

                                        <div class="project-detail-body">
                                            <h2>"Project Overview"</h2>
                                            <p>{proj.long_description.clone()}</p>
                                        </div>
                                    </article>
                                </div>

                                // Right Column: Sidebar (Links, technologies, metadata)
                                <aside class="project-detail-sidebar">
                                    // Show links card if at least one link is present
                                    {
                                        let github_url = proj.github_url.clone();
                                        let live_url = proj.live_url.clone();
                                        let download_url = proj.download_url.clone();
                                        let has_any_links = github_url.is_some() || live_url.is_some() || download_url.is_some();
                                        has_any_links.then(move || {
                                            view! {
                                                <div class="project-detail-card links-card">
                                                    <h3>"Links & Actions"</h3>
                                                    <div class="action-buttons-group">
                                                        {live_url.as_ref().map(|url| {
                                                            view! {
                                                                <a href=url.clone() target="_blank" rel="noopener noreferrer" class="action-btn live-btn">
                                                                    <svg class="btn-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                                                                        <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" />
                                                                        <polyline points="15 3 21 3 21 9" />
                                                                        <line x1="10" y1="14" x2="21" y2="3" />
                                                                    </svg>
                                                                    <span>"Live Demo"</span>
                                                                </a>
                                                            }
                                                        })}
                                                        {github_url.as_ref().map(|url| {
                                                            view! {
                                                                <a href=url.clone() target="_blank" rel="noopener noreferrer" class="action-btn github-btn">
                                                                    <svg class="btn-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                                                                        <path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22" />
                                                                    </svg>
                                                                    <span>"GitHub Repo"</span>
                                                                </a>
                                                            }
                                                        })}
                                                        {download_url.as_ref().map(|url| {
                                                            view! {
                                                                <a href=url.clone() download target="_blank" rel="noopener noreferrer" class="action-btn download-btn">
                                                                    <svg class="btn-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                                                                        <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
                                                                        <polyline points="7 10 12 15 17 10" />
                                                                        <line x1="12" y1="15" x2="12" y2="3" />
                                                                    </svg>
                                                                    <span>"Download"</span>
                                                                </a>
                                                            }
                                                        })}
                                                    </div>
                                                </div>
                                            }
                                        })
                                    }

                                    <div class="project-detail-card tech-card">
                                        <h3>"Technologies Used"</h3>
                                        <div class="project-detail-tech">
                                            {techs.into_iter().map(|tech| {
                                                view! { <span class="tech-badge">{tech}</span> }
                                            }).collect_view()}
                                        </div>
                                    </div>

                                    <div class="project-detail-card info-card">
                                        <h3>"Project Details"</h3>
                                        <div class="info-row">
                                            <span class="info-label">"Category"</span>
                                            <span class="info-value">{proj.category.clone()}</span>
                                        </div>
                                        <div class="info-row">
                                            <span class="info-label">"Status"</span>
                                            <span class="info-value">
                                                {if proj.live_url.is_some() { "Live / Production" } else { "Completed" }}
                                            </span>
                                        </div>
                                    </div>
                                </aside>
                            </div>
                        }
                    })
                }}
            </Suspense>
        </div>
    }
}
