use std::net::SocketAddr;
use axum::Router;
use tower_http::compression::{CompressionLayer};
use tower_http::compression::predicate::SizeAbove;
use ruoyi_rust::context::CONTEXT;
use tower_http::services::{ServeDir, ServeFile};
use ruoyi_rust::build_api;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    //log
    ruoyi_rust::config::log::init_log();
    if CONTEXT.config.debug {
        log::info!(
            "[ruoyi_rust] ///////////////////// Start On Debug Mode //////////////////////////////"
        );
    } else {
        log::info!(
            "[ruoyi_rust] ///////////////////// Start On Release Mode ////////////////////////////"
        );
    }
    //database
    CONTEXT.init_database().await;
    let _ = CONTEXT.sys_menu_service.update_cache().await;
    let _ = CONTEXT.sys_dict_data_service.update_cache().await;
    //table::sync_tables(&CONTEXT.rb).await;
    //  table::sync_tables_data(&CONTEXT.rb).await;

    log::info!(
        "[ruoyi_rust] ////////////////////////////////////////////////////////////////////////"
    );
    //router
    let app = Router::new()
        .nest(&CONTEXT.config.base_api, build_api())
        .layer(CompressionLayer::new().compress_when(SizeAbove::new(2048)))
        .fallback_service(
            ServeDir::new("./dist/").not_found_service(ServeFile::new("./dist/index.html")),
        )
        .nest_service(
            "/assets",
            ServeDir::new("../dist/assets/")
                .not_found_service(ServeFile::new("../dist/index.html")),
        );

    log::info!(
        "[ruoyi_rust] http server listen on {}",
        CONTEXT.config.server_url
    );
    let listener = tokio::net::TcpListener::bind(&CONTEXT.config.server_url)
        .await
        .unwrap();
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await
}
