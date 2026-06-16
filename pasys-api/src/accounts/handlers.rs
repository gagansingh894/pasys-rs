use crate::accounts::models::{
    CreateAccountRequest, CreateAccountResponse, ErrorResponse, GetAccountResponse,
};
use crate::accounts::parsers::parse_account_proto;
use crate::helpers::map_grpc_code_to_http;
use crate::state::AppState;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use std::sync::Arc;
use tracing::instrument;

#[instrument(skip(app_state, payload))]
pub async fn create_account(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<CreateAccountRequest>,
) -> Result<(StatusCode, Json<CreateAccountResponse>), (StatusCode, Json<ErrorResponse>)> {
    let account_name = payload.name;
    let account_type = match payload.account_type.as_str() {
        "CUSTOMER" => accounts_proto::accounts_v1::AccountType::Customer,
        "MERCHANT" => accounts_proto::accounts_v1::AccountType::Merchant,
        "SYSTEM" => accounts_proto::accounts_v1::AccountType::System,
        _ => {
            let resp = ErrorResponse {
                error: "".to_string(),
            };
            return Err((StatusCode::BAD_REQUEST, Json(resp)));
        }
    };
    let created_by = payload.created_by;

    let mut accounts_client = app_state.accounts_client.clone();
    match accounts_client
        .create_account(accounts_proto::accounts_v1::CreateAccountRequest {
            name: account_name,
            r#type: account_type as i32,
            created_by,
        })
        .await
    {
        Ok(resp) => match resp.into_inner().account {
            Some(account) => match parse_account_proto(account) {
                Ok(account) => Ok((
                    StatusCode::OK,
                    Json(CreateAccountResponse {
                        account: Some(account),
                    }),
                )),
                Err(err) => {
                    let resp = ErrorResponse {
                        error: err.to_string(),
                    };
                    Err((StatusCode::INTERNAL_SERVER_ERROR, Json(resp)))
                }
            },
            None => Ok((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(CreateAccountResponse { account: None }),
            )),
        },
        Err(err) => {
            let status = map_grpc_code_to_http(err.code());
            let resp = ErrorResponse {
                error: "failed to create account".to_string(),
            };
            Err((status, Json(resp)))
        }
    }
}

#[instrument(skip(app_state), fields(account_id = %id))]
pub async fn get_account(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<(StatusCode, Json<GetAccountResponse>), (StatusCode, Json<ErrorResponse>)> {
    let mut accounts_client = app_state.accounts_client.clone();
    match accounts_client
        .get_account(accounts_proto::accounts_v1::GetAccountRequest {
            account_id: id.clone(),
        })
        .await
    {
        Ok(resp) => match resp.into_inner().account {
            Some(account) => match parse_account_proto(account) {
                Ok(account) => Ok((
                    StatusCode::OK,
                    Json(GetAccountResponse {
                        account: Some(account),
                    }),
                )),
                Err(err) => {
                    let resp = ErrorResponse {
                        error: err.to_string(),
                    };
                    Err((StatusCode::INTERNAL_SERVER_ERROR, Json(resp)))
                }
            },
            None => Ok((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(GetAccountResponse { account: None }),
            )),
        },
        Err(err) => {
            let status = map_grpc_code_to_http(err.code());
            let resp = ErrorResponse {
                error: format!("failed to get account: {}", id),
            };
            Err((status, Json(resp)))
        }
    }
}
