use common::shutdown;
use ledger::repo::PgLedgerRepository;
use ledger::service::LedgerService;
use ledger_proto::ledger_v1::FILE_DESCRIPTOR_SET;
use ledger_proto::ledger_v1::ledger_server::LedgerServer;
use std::env;
use tonic::codegen::tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // setup database
    let database_config = common::database::Config {
        reader_url: "".to_string(),
        reader_max_connections: 0,
        writer_url: "".to_string(),
        writer_max_connections: 0,
        timeout_in_secs: 0,
    };
    let db = common::database::Database::new(&database_config)
        .await
        .expect("failed to create database");

    // setup repo
    let repo = PgLedgerRepository::new(db);

    // setup service
    let ledger_service = LedgerService::new(repo);

    // setup reflection service
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build_v1()?;

    // setup grpc server
    let port = env::var("PORT")
        .unwrap_or("8000".to_string())
        .parse::<u32>()?;
    let address = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("Failed to create TCP listener ❌");
    Server::builder()
        .add_service(reflection_service)
        .add_service(LedgerServer::new(ledger_service))
        .serve_with_incoming_shutdown(
            TcpListenerStream::new(listener),
            shutdown::shutdown_signal(),
        )
        .await?;

    Ok(())
}
