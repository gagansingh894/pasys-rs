use axum::http::StatusCode;
use tonic::Code;

/// Helper to map gRPC error codes to HTTP status codes
pub fn map_grpc_code_to_http(code: Code) -> StatusCode {
    match code {
        Code::Ok => StatusCode::OK,
        Code::InvalidArgument => StatusCode::BAD_REQUEST,
        Code::NotFound => StatusCode::NOT_FOUND,
        Code::AlreadyExists => StatusCode::CONFLICT,
        Code::PermissionDenied => StatusCode::FORBIDDEN,
        Code::Unauthenticated => StatusCode::UNAUTHORIZED,
        Code::Unavailable => StatusCode::SERVICE_UNAVAILABLE,
        Code::DeadlineExceeded => StatusCode::GATEWAY_TIMEOUT,
        Code::Unimplemented => StatusCode::NOT_IMPLEMENTED,
        Code::Internal | Code::Unknown | Code::Aborted | Code::Cancelled | Code::DataLoss => {
            StatusCode::INTERNAL_SERVER_ERROR
        }
        Code::ResourceExhausted | Code::OutOfRange | Code::FailedPrecondition => {
            StatusCode::BAD_REQUEST
        }
    }
}
