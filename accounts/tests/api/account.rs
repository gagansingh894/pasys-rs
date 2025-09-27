use crate::helpers;
use accounts_proto::accounts_v1::{CreateAccountRequest, GetAccountRequest};
use tokio::net::TcpListener;
use tonic::codegen::tokio_stream::wrappers::TcpListenerStream;

#[tokio::test]
async fn successfully_calls_the_create_account_rpc_and_retrieve_account_using_get_account_rpc() {
    // arrange
    let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let test_server = helpers::accounts_grpc_test_server().await;

    tokio::spawn(async move {
        test_server
            .serve_with_incoming(TcpListenerStream::new(listener))
            .await
            .unwrap()
    });

    let mut client = helpers::grpc_client_stub(addr.to_string()).await;

    // act
    let response = client
        .create_account(CreateAccountRequest {
            name: "test".to_string(),
            r#type: 1,
            created_by: "test".to_string(),
        })
        .await;

    // assert
    assert!(response.is_ok());
    let account = response.unwrap().into_inner().account.unwrap();

    let get_account_response = client
        .get_account(GetAccountRequest {
            account_id: account.id.clone(),
        })
        .await;
    assert!(get_account_response.is_ok());
    let get_account = get_account_response.unwrap().into_inner().account.unwrap();
    assert_eq!(get_account.name, account.name);
    assert_eq!(get_account.r#type, account.r#type);
    assert_eq!(get_account.id, account.id);
}
