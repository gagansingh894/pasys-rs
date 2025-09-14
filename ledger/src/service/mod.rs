use crate::repo::{LedgerRepository, PgLedgerRepository};

pub struct LedgerService<R>
where
    R: LedgerRepository,
{
    repo: R,
}

impl<R> LedgerService<R>
where
    R: LedgerRepository,
{
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}
