use axum::extract::Query;
use dotenv::dotenv;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct WeeklyWeatherQuery {
    pub latitude: f64,
    pub longitude: f64,
}

pub async fn weekly_weather(location: Query<WeeklyWeatherQuery>) -> String {
    let latitude = location.latitude;
    let longitude = location.longitude;

    let url = format!(
        "https://weather.googleapis.com/v1/forecast/days:lookup?key={}location.latitude={}&location.longitude={}&days=7",
        get_api_key(),
        latitude,
        longitude,
    );

    let response = reqwest::get(url).await.unwrap();
    let body = response.text().await.unwrap();
    body
}

fn get_api_key() -> String {
    dotenv().ok();
    env::var("GOOGLE_MAPS_API_KEY").expect("GOOGLE_MAPS_API_KEY must be set")
}
