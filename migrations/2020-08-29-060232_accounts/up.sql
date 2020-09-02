CREATE TABLE accounts (
    id BIGSERIAL PRIMARY KEY NOT NULL,
    full_name VARCHAR NOT NULL,
    phone_num VARCHAR(15) NOT NULL,
    email VARCHAR NOT NULL,
    role TEXT NOT NULL,
    active BOOLEAN NOT NULL,
    registration_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
-- Unique column dari tabel accounts
CREATE UNIQUE INDEX idx_account_phone_num ON accounts ((lower(phone_num)));
CREATE UNIQUE INDEX idx_account_email ON accounts ((lower(email)));
-- Kumpulan beberapa password akun yang sudah ter-hash
CREATE TABLE account_passhash (
    id BIGSERIAL PRIMARY KEY NOT NULL,
    account_id BIGSERIAL REFERENCES accounts (id) ON DELETE CASCADE,
    passhash VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);