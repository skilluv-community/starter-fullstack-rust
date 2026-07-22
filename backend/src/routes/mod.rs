use axum::Router;

use crate::AppState;

pub mod health;
pub mod hello;
pub mod notes;

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(health::routes())
        .nest("/api", hello::routes().merge(notes::routes()))
}
