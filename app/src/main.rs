mod rest;

use crate::rest::{router, server};

use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|e| {
                warn!("Não foi possivel ler a variavel RUST_LOG, ira seguir no padrao: {e}");
                tracing_subscriber::EnvFilter::new("warn,ff_codex=trace,reqwest=trace")
            }),
        )
        .json()
        .init();

    info!("Inciando a api de Final Fantasy.");
    let app = router();
    server(app).await?;

    Ok(())
}
