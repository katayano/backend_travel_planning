use axum::{Router, routing::get};

use crate::handler::weather::weekly_weather;

/// 天気用のルータを作成する関数
pub fn build_weather_routers() -> Router {
    let routers = Router::new().route("/weekly", get(weekly_weather));
    Router::new().nest("/weather", routers)
}
