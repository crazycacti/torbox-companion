
#![recursion_limit = "768"]

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use axum::middleware;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use std::sync::Arc;
    use torbox_companion::app::*;
    use torbox_companion::automation::{Database, AutomationScheduler, create_routes};
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

    let db_path = std::env::var("TORBOX_DB_PATH")
        .unwrap_or_else(|_| "data/torbox.db".to_string());

    let max_rules_per_user = std::env::var("TORBOX_MAX_RULES_PER_USER")
        .unwrap_or_else(|_| "100".to_string())
        .parse::<i64>()
        .unwrap_or(100);

    let log_retention_days = std::env::var("TORBOX_LOG_RETENTION_DAYS")
        .unwrap_or_else(|_| "90".to_string())
        .parse::<i64>()
        .unwrap_or(90);

    let rule_execution_timeout_secs = std::env::var("TORBOX_RULE_EXECUTION_TIMEOUT_SECS")
        .unwrap_or_else(|_| "130".to_string())
        .parse::<u64>()
        .unwrap_or(130);

    log!("Initializing automation database at: {}", db_path);
    log!("Configuration: max_rules_per_user={}, log_retention_days={}, execution_timeout_secs={}", 
         max_rules_per_user, log_retention_days, rule_execution_timeout_secs);
    
    let database = Arc::new(
        Database::new(&db_path)
            .await
            .expect("Failed to initialize database")
    );

    let database_for_cleanup = database.clone();
    let log_retention_days_clone = log_retention_days;
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(86400));
        loop {
            interval.tick().await;
            match database_for_cleanup.cleanup_old_logs(log_retention_days_clone).await {
                Ok(count) => {
                    if count > 0 {
                        log!("Cleaned up {} old execution logs (older than {} days)", count, log_retention_days_clone);
                    }
                }
                Err(e) => {
                    log!("Failed to cleanup old logs: {}", e);
                }
            }
        }
    });

    log!("Initializing automation scheduler...");
    let scheduler = Arc::new(
        AutomationScheduler::new(database.clone(), rule_execution_timeout_secs)
            .await
            .expect("Failed to initialize scheduler")
    );

    scheduler.start().await.expect("Failed to start scheduler");
    log!("Automation scheduler started");

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

    let automation_routes = create_routes(database.clone(), scheduler.clone(), max_rules_per_user, rule_execution_timeout_secs);
    let app = app.merge(automation_routes);

    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    let shutdown_scheduler = scheduler.clone();
    let shutdown = async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install CTRL+C signal handler");
        log!("Shutdown signal received, stopping scheduler...");
        if let Err(e) = shutdown_scheduler.shutdown().await {
            log!("Error shutting down scheduler: {}", e);
        }
        log!("Shutting down server...");
    };

    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown)
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // Client-side entry point is in lib.rs (hydrate function)
}
