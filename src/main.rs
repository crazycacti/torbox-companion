
#![recursion_limit = "768"]

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use axum::middleware;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use torbox_companion::app::*;
    use torbox_companion::logging::logging_middleware;

    let conf = get_configuration(Some("Cargo.toml")).unwrap();
    let addr = std::env::var("LEPTOS_SITE_ADDR")
        .unwrap_or_else(|_| conf.leptos_options.site_addr.to_string());
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);
    let enable_logging = std::env::var("ENABLE_LOGGING")
        .unwrap_or_else(|_| "true".to_string())
        .parse::<bool>()
        .unwrap_or(true);

    let mut app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);
    
    let log_level = std::env::var("LOG_LEVEL")
        .unwrap_or_else(|_| "errors".to_string());
    if enable_logging {
        app = app.layer(middleware::from_fn(logging_middleware));
        if log_level == "full" {
            log!("Request logging enabled (full mode)");
        } else {
            log!("Request logging enabled (errors only)");
        }
    } else {
        log!("Request logging disabled");
    }
    
    let download_routes = torbox_companion::api::server_routes::create_download_routes()
        .with_state(());
    let app = app.merge(download_routes);
    
    let api_proxy_routes = torbox_companion::api::proxy::create_api_proxy_routes();
    let app = app.merge(api_proxy_routes);
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // Client-side entry point is in lib.rs (hydrate function)
}
