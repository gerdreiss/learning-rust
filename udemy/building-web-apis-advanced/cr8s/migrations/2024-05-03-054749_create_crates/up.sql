CREATE TABLE crates (
   id SERIAL PRIMARY KEY,
   rustacean_id INTEGER NOT NULL REFERENCES rustaceans(id),
   code VARCHAR(64) NOT NULL,
   name VARCHAR(128) NOT NULL,
   version VARCHAR(64) NOT NULL,
   description TEXT,
   created_at TIMESTAMP NOT NULL DEFAULT NOW()
)