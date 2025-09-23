-- Add migration script here
CREATE TYPE account_type AS ENUM ('customer', 'merchant', 'system');
CREATE TYPE account_status AS ENUM ('active', 'frozen', 'closed');

-- safe to do since no data in database yet
ALTER TABLE accounts ALTER COLUMN account_status DROP DEFAULT;


ALTER TABLE accounts
        ALTER COLUMN account_type TYPE account_type USING LOWER(account_type)::account_type,
        ALTER COLUMN account_status TYPE account_status USING LOWER(account_status)::account_status;

ALTER TABLE accounts
        ALTER COLUMN account_status SET DEFAULT 'active';
