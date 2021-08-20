CREATE TABLE IF NOT EXISTS auth_client (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  password VARCHAR NOT NULL,
  CONSTRAINT unique_name_auth_client UNIQUE(name)
);
