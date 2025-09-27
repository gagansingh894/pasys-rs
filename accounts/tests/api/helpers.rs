use accounts::repo;
use accounts::service::AccountsService;
use accounts_proto::accounts_v1::accounts_client::AccountsClient;
use accounts_proto::accounts_v1::accounts_server;
use common::database;
use std::env;
use std::time::Duration;
use tonic::transport::server::Router;
use tonic::transport::{Channel, Server};

pub async fn accounts_grpc_test_server() -> Router {
    // setup database
    let database_config = database::Config {
        reader_url: env::var("READER_DATABASE_URL")
            .unwrap_or("postgres://user:user123@localhost:5433/accounts".to_string()),
        reader_max_connections: 1,
        writer_url: env::var("WRITER_DATABASE_URL")
            .unwrap_or("postgres://user:user123@localhost:5433/accounts".to_string()),
        writer_max_connections: 1,
        timeout_in_secs: 5,
    };
    let db = database::Database::new(&database_config)
        .await
        .expect("failed to create database");

    // setup repo layer
    let repo = repo::PgAccountRepository::new(db);

    // setup service
    let service = AccountsService::new(repo);

    Server::builder().add_service(accounts_server::AccountsServer::new(service))
}

pub async fn grpc_client_stub(addr: String) -> AccountsClient<Channel> {
    let channel = Channel::builder(format!("http://{}", addr).parse().unwrap())
        .timeout(Duration::from_secs(2))
        .connect()
        .await
        .unwrap();

    AccountsClient::new(channel)
}
