use axum::{Router, routing::get};

use crate::handler::ai::chat;

/// 天気用のルータを作成する関数
pub fn build_ai_routers() -> Router {
    let routers = Router::new().route("/chat", get(chat));
    Router::new().nest("/ai", routers)
}
