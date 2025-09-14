use crate::repo::AccountRepository;
use crate::service::AccountsService;
use accounts_proto::accounts_v1::accounts_server::Accounts;
use accounts_proto::accounts_v1::{
    CreateAccountRequest, CreateAccountResponse, GetAccountsRequest, GetAccountsResponse,
};
use async_trait::async_trait;

#[async_trait]
impl<R> Accounts for AccountsService<R>
where
    R: AccountRepository,
{
    async fn health_check(
        &self,
        request: tonic::Request<()>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn create_account(
        &self,
        request: tonic::Request<CreateAccountRequest>,
    ) -> Result<tonic::Response<CreateAccountResponse>, tonic::Status> {
        todo!()
    }

    async fn get_accounts(
        &self,
        request: tonic::Request<GetAccountsRequest>,
    ) -> Result<tonic::Response<GetAccountsResponse>, tonic::Status> {
        todo!()
    }
}
