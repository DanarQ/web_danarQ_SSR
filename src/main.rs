#![recursion_limit = "512"]

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use web_danar_q_ssr::app::*;

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let database_url = std::env::var("DATABASE_URL")
        .or_else(|_| std::env::var("SUPABASE_URL"))
        .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:5432/postgres".to_string());

    let db_conn = match sea_orm::Database::connect(&database_url).await {
        Ok(conn) => {
            log!("Database connection established successfully!");
            // Self-healing migrations to add new columns if they do not exist
            use sea_orm::{ConnectionTrait, Statement};
            let migrations = vec![
                "ALTER TABLE projects ADD COLUMN IF NOT EXISTS image_url VARCHAR;",
                "ALTER TABLE projects ADD COLUMN IF NOT EXISTS github_url VARCHAR;",
                "ALTER TABLE projects ADD COLUMN IF NOT EXISTS download_url VARCHAR;",
                "ALTER TABLE projects ADD COLUMN IF NOT EXISTS live_url VARCHAR;",
            ];
            for sql in migrations {
                let stmt = Statement::from_string(conn.get_database_backend(), sql);
                if let Err(err) = conn.execute(stmt).await {
                    log!("Warning: Failed to run column migration: {:?}", err);
                }
            }
            Some(conn)
        }
        Err(err) => {
            log!("Warning: Database connection failed: {:?}", err);
            None
        }
    };
    let app = Router::new()
        .leptos_routes_with_context(&leptos_options, routes, {
            let db = db_conn.clone();
            move || {
                if let Some(ref db) = db {
                    provide_context(db.clone());
                }
            }
        }, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
