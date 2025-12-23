use axum::extract::DefaultBodyLimit;
use axum::Router;
use ruoyi_rust::build_api;
use ruoyi_rust::context::CONTEXT;
use std::net::SocketAddr;
use std::path::PathBuf;
use tower_http::compression::predicate::SizeAbove;
use tower_http::compression::CompressionLayer;
use tower_http::services::{ServeDir, ServeFile};

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
        .nest(
            &CONTEXT.config.base_api,
            build_api().nest_service(//todo 加入验证
                "/profile",
                ServeDir::new(PathBuf::from(&CONTEXT.config.upload_path).join("profile")),
            ).nest_service(
                "/attach",
                ServeDir::new(PathBuf::from(&CONTEXT.config.upload_path).join("attach")),
            ),
        )
        .layer(CompressionLayer::new().compress_when(SizeAbove::new(2048))) //启动压缩
        .layer(DefaultBodyLimit::disable())
        .layer(tower_http::limit::RequestBodyLimitLayer::new(
            CONTEXT.config.upload_max_size * 1024 * 1024,
        ))
        .fallback_service(
            ServeDir::new("./dist/").not_found_service(ServeFile::new("./dist/index.html")),
        )
        .nest_service(
            "/assets",
            ServeDir::new("./dist/assets/")
                .not_found_service(ServeFile::new("../dist/index.html")),
        );

    println!(
        "[ruoyi_rust] http server listen on {}",
        CONTEXT.config.server_url
    );
    let listener = tokio::net::TcpListener::bind(&CONTEXT.config.server_url)
        .await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
}


