ALTER TABLE accounts
    ALTER COLUMN created_by TYPE TEXT USING created_by::TEXT;
