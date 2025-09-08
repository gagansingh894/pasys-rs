CREATE TABLE accounts (
                          id UUID PRIMARY KEY DEFAULT gen_random_uuid(),      -- Unique account ID
                          name TEXT NOT NULL,                                 -- Account name
                          account_type TEXT NOT NULL,                         -- 'CUSTOMER', 'MERCHANT', 'SYSTEM', etc.
                          account_status TEXT NOT NULL DEFAULT 'ACTIVE',     -- 'ACTIVE', 'FROZEN', 'CLOSED', etc.
                          created_by UUID NOT NULL,                           -- ID of the creator (user/system)
                          created_at TIMESTAMPTZ NOT NULL DEFAULT now(),      -- Timestamp of creation
                          updated_at TIMESTAMPTZ NOT NULL DEFAULT now()       -- Timestamp of last update
);