use axum::http::StatusCode;
use axum::{extract::State, routing::post, Json, Router};
use libpasskey::{
    auth::{
        start_authentication, verify_authentication, AuthenticationOptions, AuthenticatorResponse,
    },
    register::{finish_registration, start_registration, RegisterCredential, RegistrationOptions},
    AppState,
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/register/start", post(handle_start_registration))
        .route("/register/finish", post(handle_finish_registration))
        .route("/auth/start", post(handle_start_authentication))
        .route("/auth/finish", post(handle_finish_authentication))
        .with_state(state)
}

async fn handle_start_registration(
    State(state): State<AppState>,
    Json(username): Json<String>,
) -> Json<RegistrationOptions> {
    Json(start_registration(&state, username).await)
}

async fn handle_finish_registration(
    State(state): State<AppState>,
    Json(reg_data): Json<RegisterCredential>,
) -> Result<&'static str, (StatusCode, String)> {
    finish_registration(&state, reg_data)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))
}

async fn handle_start_authentication(State(state): State<AppState>) -> Json<AuthenticationOptions> {
    Json(start_authentication(&state).await)
}

async fn handle_finish_authentication(
    State(state): State<AppState>,
    Json(auth_response): Json<AuthenticatorResponse>,
) -> Result<String, (StatusCode, String)> {
    verify_authentication(&state, auth_response)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))
}
