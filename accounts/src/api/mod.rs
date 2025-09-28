mod parsers;

use crate::api::parsers::{parse_account_to_proto, parse_to_domain_account_type};
use crate::repo::AccountRepository;
use crate::service::AccountsService;

use accounts_proto::accounts_v1;

use async_trait::async_trait;
use tracing::instrument;

#[async_trait]
impl<R> accounts_v1::accounts_server::Accounts for AccountsService<R>
where
    R: AccountRepository,
{
    async fn health_check(
        &self,
        _request: tonic::Request<()>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        Ok(tonic::Response::new(()))
    }

    #[instrument(skip(self, request))]
    async fn create_account(
        &self,
        request: tonic::Request<accounts_v1::CreateAccountRequest>,
    ) -> Result<tonic::Response<accounts_v1::CreateAccountResponse>, tonic::Status> {
        let request = request.into_inner();

        // parse account_type to domain
        let account_type = match parse_to_domain_account_type(request.r#type) {
            Ok(t) => t,
            Err(e) => {
                tracing::error!("failed to parse_to_domain_account_type: {}", e);
                return Err(tonic::Status::invalid_argument(e.to_string()));
            }
        };

        // call account service to create account
        let account = match self
            .create_account(request.name, account_type, request.created_by)
            .await
        {
            Ok(account) => parse_account_to_proto(account),
            Err(e) => {
                tracing::error!("failed to create account: {}", e);
                return Err(tonic::Status::new(
                    tonic::Code::Internal,
                    format!("failed to create account: {}", e),
                ));
            }
        };

        Ok(tonic::Response::new(accounts_v1::CreateAccountResponse {
            account: Some(account),
        }))
    }

    #[instrument(skip(self, _request))]
    async fn get_accounts(
        &self,
        _request: tonic::Request<accounts_v1::GetAccountsRequest>,
    ) -> Result<tonic::Response<accounts_v1::GetAccountsResponse>, tonic::Status> {
        todo!()
    }

    #[instrument(skip(self, request))]
    async fn get_account(
        &self,
        request: tonic::Request<accounts_v1::GetAccountRequest>,
    ) -> Result<tonic::Response<accounts_v1::GetAccountResponse>, tonic::Status> {
        let request = request.into_inner();
        let account = match self.get_account_by_id(request.account_id.as_str()).await {
            Ok(account) => parse_account_to_proto(account),
            Err(e) => {
                tracing::error!(
                    "failed to get account {}: {}",
                    request.account_id.as_str(),
                    e
                );
                return Err(tonic::Status::new(
                    tonic::Code::Internal,
                    format!("failed to get account: {}", e),
                ));
            }
        };

        Ok(tonic::Response::new(accounts_v1::GetAccountResponse {
            account: Some(account),
        }))
    }
}
