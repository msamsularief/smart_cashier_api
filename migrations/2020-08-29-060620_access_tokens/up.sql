CREATE TABLE access_tokens (
    id BIGSERIAL PRIMARY KEY NOT NULL,
    account_id BIGINT REFERENCES accounts (id) ON DELETE CASCADE,
    token VARCHAR(100) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
-- Unique column dari tabel access_tokens
CREATE INDEX idx_access_tokens_account_id ON access_tokens ((account_id));