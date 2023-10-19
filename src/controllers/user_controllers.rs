use crate::config::settings::Settings;
use crate::models::user_model::{LoginUser, NewUser, TokenClaims, User};
use crate::repository::user_repository::UserRepository;

use axum::extract::{Extension, Json, State};
use axum::{http::StatusCode, response::IntoResponse};
use bcrypt::{hash, verify, DEFAULT_COST};
use std::sync::Arc;

use jsonwebtoken::{encode, EncodingKey, Header};

type AppState = State<Arc<UserRepository>>;

pub async fn find_user(Extension(current_user): Extension<User>) -> impl IntoResponse {
    Json(current_user)
}

pub async fn create_new_user(
    State(user_repo): AppState,
    Json(mut new_user): Json<NewUser>,
) -> impl IntoResponse {
    new_user.password = hash(new_user.password, DEFAULT_COST).expect("");

    match user_repo.create_user(new_user).await {
        Ok(user) => Ok((StatusCode::CREATED, Json(user))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_user(
    State(user_repo): AppState,
    Extension(current_user): Extension<User>,
    Json(new_user): Json<NewUser>,
) -> impl IntoResponse {
    match user_repo.update_user(current_user.id, new_user).await {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_user(
    State(user_repo): AppState,
    Extension(current_user): Extension<User>,
) -> impl IntoResponse {
    match user_repo.delete_user(current_user.id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn login_user(
    State(user_repo): AppState,
    Json(user_login): Json<LoginUser>,
) -> impl IntoResponse {
    match user_repo.find_user_by_email(user_login.email).await {
        Ok(Some(user)) => {
            if verify(user_login.password, &user.password).is_ok() {
                let now = chrono::Utc::now();
                let iat = now.timestamp() as usize;
                let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;

                let claims = TokenClaims {
                    sub: user.id,
                    exp,
                    iat,
                };
                let token = encode(
                    &Header::default(),
                    &claims,
                    &EncodingKey::from_secret(Settings::from_env().jwt_secret.as_bytes()),
                )
                .unwrap();
                Ok(serde_json::json!({"status": "success", "token": token}).to_string())
            } else {
                Err(StatusCode::BAD_REQUEST)
            }
        }
        Ok(None) => Err(StatusCode::BAD_REQUEST),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}
