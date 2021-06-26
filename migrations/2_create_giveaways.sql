CREATE TABLE IF NOT EXISTS giveaways(
  id           BIGSERIAL PRIMARY KEY,
  user_id      BIGSERIAL NOT NULL REFERENCES users(id),
  name         TEXT NOT NULL,
  location     TEXT NOT NULL,
  description  TEXT,
  updated      TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE TABLE IF NOT EXISTS item_tags(
  id           BIGSERIAL PRIMARY KEY,
  tag          TEXT NOT NULL
);

/* e.g. furniture, toys, food, services, appliance, clothing, electronics, sports, */

CREATE TABLE IF NOT EXISTS item_pictures(
  id BIGSERIAL PRIMARY KEY,
  giveaway_id BIGSERIAL NOT NULL REFERENCES giveaways(id),
  bytes BYTEA NOT NULL,
  description TEXT
);
