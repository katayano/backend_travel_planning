use axum::Router;

use super::weather::build_weather_routers;

/// v1用のルータを作成する関数
pub fn routes() -> Router {
    let router = Router::new().merge(build_weather_routers());

    Router::new().nest("/v1", router)
}
