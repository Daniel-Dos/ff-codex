use axum::Router;
use std::env::var;
use tracing::info;

pub async fn server(app: Router) -> Result<(), anyhow::Error> {
    let host = var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = var("PORT").unwrap_or_else(|_| "8080".to_string());
    let address = format!("{host}:{port}");
    info!("Server starting on http://{address}");
    let listener = tokio::net::TcpListener::bind(&address).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    info!("Desligamento gracioso concluído. Servidor encerrado.");
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => info!("Sinal de parada recebido (Ctrl+C). Drenando requisições..."),
        _ = terminate => info!("Sinal de parada recebido (SIGTERM). Drenando requisições..."),
    }
}
