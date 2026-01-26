use axum::Router;

use super::ai::build_ai_routers;
use super::weather::build_weather_routers;

/// v1用のルータを作成する関数
pub fn routes() -> Router {
    let router = Router::new()
        .merge(build_weather_routers())
        .merge(build_ai_routers());

    Router::new().nest("/v1", router)
}
