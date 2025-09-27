use accounts::repo;
use accounts::service::AccountsService;
use accounts::{DEFAULT_READER_MAX_CONN, DEFAULT_TIMEOUT_SECONDS, DEFAULT_WRITER_MAX_CONN};
use accounts_proto::accounts_v1::{FILE_DESCRIPTOR_SET, accounts_server};
use common::{database, shutdown};
use std::env;
use tonic::codegen::tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // setup database
    let database_config = database::Config {
        reader_url: env::var("READER_DATABASE_URL").expect("READER_DATABASE_URL must be set"),
        reader_max_connections: DEFAULT_READER_MAX_CONN,
        writer_url: env::var("WRITER_DATABASE_URL").expect("WRITER_DATABASE_URL must be set"),
        writer_max_connections: DEFAULT_WRITER_MAX_CONN,
        timeout_in_secs: DEFAULT_TIMEOUT_SECONDS,
    };
    let db = database::Database::new(&database_config)
        .await
        .expect("failed to create database");

    // setup repo layer
    let repo = repo::PgAccountRepository::new(db);

    // setup service
    let service = AccountsService::new(repo);

    // add reflection
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build_v1()?;

    // setup grpc server
    let port: u32 = env::var("PORT").unwrap_or("8000".to_string()).parse()?;
    let address = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("Failed to create TCP listener ❌");
    Server::builder()
        .add_service(reflection_service)
        // add accounts service to accounts server
        .add_service(accounts_server::AccountsServer::new(service))
        .serve_with_incoming_shutdown(
            TcpListenerStream::new(listener),
            shutdown::shutdown_signal(),
        )
        .await?;

    Ok(())
}
