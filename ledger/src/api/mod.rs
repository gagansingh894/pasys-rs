use crate::repo::LedgerRepository;
use crate::service::LedgerService;
use async_trait::async_trait;
use ledger_proto::ledger_v1::ledger_server::Ledger;
use ledger_proto::ledger_v1::{CreateTransactionRequest, CreateTransactionResponse};

#[async_trait]
impl<R> Ledger for LedgerService<R>
where
    R: LedgerRepository,
{
    async fn health_check(
        &self,
        request: tonic::Request<()>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn create_transaction(
        &self,
        request: tonic::Request<CreateTransactionRequest>,
    ) -> Result<tonic::Response<CreateTransactionResponse>, tonic::Status> {
        todo!()
    }
}
