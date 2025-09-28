use crate::config::Config;
use accounts_proto::accounts_v1::accounts_client::AccountsClient;
use tonic::transport::Channel;

pub struct AppState {
    pub accounts_client: AccountsClient<Channel>,
}

impl AppState {
    pub async fn new(config: Config) -> Self {
        let accounts_client = AccountsClient::connect(get_url(config.accounts_host.clone()))
            .await
            .expect("failed to connect to accounts service");
        Self { accounts_client }
    }
}


pub fn get_url(base_url: String) -> String {
    if base_url.starts_with("http://") || base_url.starts_with("https://") {
        return base_url;
    }
    format!("http://{}", base_url)
}