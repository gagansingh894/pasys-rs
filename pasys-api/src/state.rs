use accounts_proto::accounts_v1::accounts_client::AccountsClient;
use tonic::transport::Channel;

pub struct Config {
    pub accounts_host: String,
}

pub struct AppState {
    pub accounts_client: AccountsClient<Channel>,
}

impl AppState {
    pub async fn new(config: Config) -> Self {
        let accounts_client = AccountsClient::connect(config.accounts_host.clone())
            .await
            .expect("failed to connect to accounts service");
        Self { accounts_client }
    }
}
