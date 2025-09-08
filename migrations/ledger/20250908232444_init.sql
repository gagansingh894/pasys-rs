CREATE TABLE accounts (
                          id UUID PRIMARY KEY,            -- Account ID (from Accounts service)
                          account_type TEXT NOT NULL      -- 'CUSTOMER', 'MERCHANT', 'SYSTEM'
);

CREATE TABLE transactions (
                              id UUID PRIMARY KEY,
                              client_id UUID NOT NULL,
                              debit_account_id UUID NOT NULL REFERENCES accounts(id),
                              credit_account_id UUID NOT NULL REFERENCES accounts(id),
                              amount_minor BIGINT NOT NULL,
                              currency CHAR(3) NOT NULL,
                              idempotency_key TEXT NOT NULL UNIQUE,
                              status TEXT NOT NULL DEFAULT 'INIT',
                              request_timestamp TIMESTAMP WITH TIME ZONE,
                              created_at TIMESTAMP WITH TIME ZONE,
                              updated_at TIMESTAMP WITH TIME ZONE
);

CREATE TABLE ledger_entries (
                                id UUID PRIMARY KEY,
                                transaction_id UUID NOT NULL REFERENCES transactions(id) ON DELETE CASCADE,
                                account_id UUID NOT NULL REFERENCES accounts(id),
                                entry_type TEXT NOT NULL,
                                amount_minor BIGINT NOT NULL,
                                currency CHAR(3) NOT NULL,
                                created_at TIMESTAMP WITH TIME ZONE
);

CREATE TABLE transaction_events (
                                    id UUID PRIMARY KEY,
                                    transaction_id UUID NOT NULL REFERENCES transactions(id),
                                    event_type TEXT NOT NULL,
                                    payload JSONB,
                                    processed BOOLEAN DEFAULT FALSE,
                                    created_at TIMESTAMP WITH TIME ZONE,
                                    processed_at TIMESTAMP WITH TIME ZONE
);

-- Indexes
CREATE INDEX idx_transactions_client ON transactions(client_id);
CREATE INDEX idx_ledger_entries_account ON ledger_entries(account_id);
CREATE INDEX idx_transaction_events_transaction ON transaction_events(transaction_id);-- Add migration script here
