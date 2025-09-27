use crate::helpers;
use tokio::net::TcpListener;
use tonic::codegen::tokio_stream::wrappers::TcpListenerStream;

#[tokio::test]
async fn successfully_calls_the_health_check_rpc() {
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
    let response = client.health_check(()).await;

    // assert
    assert!(response.is_ok());
}
