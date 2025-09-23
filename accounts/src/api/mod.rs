use crate::domain::account::{Status, Type};
use crate::repo::AccountRepository;
use crate::service::AccountsService;
use accounts_proto::accounts_v1::accounts_server::Accounts;
use accounts_proto::accounts_v1::{
    Account, AccountType, CreateAccountRequest, CreateAccountResponse, GetAccountsRequest,
    GetAccountsResponse,
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
        Ok(tonic::Response::new(()))
    }

    async fn create_account(
        &self,
        request: tonic::Request<CreateAccountRequest>,
    ) -> Result<tonic::Response<CreateAccountResponse>, tonic::Status> {
        let request = request.into_inner();

        // parse account_type to domain
        let account_type = match request.r#type {
            0 => {
                return Err(tonic::Status::invalid_argument(format!(
                    "{} is not a valid account type",
                    request.r#type
                )))
            }
            1 => Type::Customer,
            2 => Type::Merchant,
            3 => Type::System,
            _ => return Err(tonic::Status::invalid_argument("invalid account type")),
        };

        // call account service to create account
        let account = match self
            .create_account(request.name, account_type, request.created_by)
            .await
        {
            Ok(account) => Account {
                id: account.id.to_string(),
                name: account.name,
                r#type: match account.account_type {
                    Type::Customer => AccountType::Customer as i32,
                    Type::Merchant => AccountType::Merchant as i32,
                    Type::System => AccountType::System as i32,
                },
                status: match account.account_status {
                    Status::Active => Status::Active as i32,
                    Status::Frozen => Status::Active as i32,
                    Status::Closed => Status::Active as i32,
                },
                created_by: account.created_by,
                created_at: None, // todo: fix parsing of time to proto time
                updated_at: None, // todo: fix parsing of time to proto time
            },
            Err(e) => {
                return Err(tonic::Status::new(
                    tonic::Code::Internal,
                    format!("failed to create account: {}", e),
                ));
            }
        };

        Ok(tonic::Response::new(CreateAccountResponse {
            account: Some(account),
        }))
    }

    async fn get_accounts(
        &self,
        request: tonic::Request<GetAccountsRequest>,
    ) -> Result<tonic::Response<GetAccountsResponse>, tonic::Status> {
        todo!()
    }
}
