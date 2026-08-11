use axum::http::StatusCode;

pub async fn get_games() ->  Result<(StatusCode, String), (StatusCode, String)>  {
    Ok((StatusCode::OK, "Hello World!".to_string()))
}