use axum::{routing::post, Router};
use libpasskey::{
    auth::{start_authentication, verify_authentication},
    passkey::AppState,
    register::{finish_registration, start_registration},
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/register/start", post(start_registration))
        .route("/register/finish", post(finish_registration))
        .route("/auth/start", post(start_authentication))
        .route("/auth/finish", post(verify_authentication))
        .with_state(state)
}
