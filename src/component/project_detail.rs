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
                            <article class="project-detail-content">
                                <header class="project-detail-header">
                                    <div class="project-detail-category">{proj.category.clone()}</div>
                                    <h1>{proj.title.clone()}</h1>
                                    <p class="project-detail-summary">{proj.description.clone()}</p>

                                    <div class="project-detail-tech">
                                        {techs.into_iter().map(|tech| {
                                            view! { <span class="tech-badge">{tech}</span> }
                                        }).collect_view()}
                                    </div>
                                </header>

                                <div class="project-detail-body">
                                    <h2>"Project Overview"</h2>
                                    <p>{proj.long_description.clone()}</p>
                                </div>
                            </article>
                        }
                    })
                }}
            </Suspense>
        </div>
    }
}
