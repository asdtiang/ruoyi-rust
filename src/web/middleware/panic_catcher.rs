use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};

/// Panic 捕获中间件
/// 这个中间件主要用于占位，实际 panic 捕获由 main 函数中的 panic hook 处理
/// 在异步中间件中捕获 panic 比较困难，因此我们依赖全局 panic hook
pub async fn catch_panic_middleware(
    req: Request,
    next: Next,
) -> Response {
    next.run(req).await
}
