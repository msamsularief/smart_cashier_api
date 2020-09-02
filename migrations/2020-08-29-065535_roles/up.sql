CREATE TABLE roles (
    id SERIAL PRIMARY KEY NOT NULL,
    role_level SMALLINT NOT NULL DEFAULT 1,
    role_name VARCHAR NOT NULL
);
-- Insert default roles
INSERT INTO roles (role_level, role_name)
VALUES (1, 'Owner'),
    (2, 'Supervisor'),
    (3, 'Operator');