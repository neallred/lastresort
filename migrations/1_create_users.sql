CREATE TABLE IF NOT EXISTS users
(
    id          BIGSERIAL PRIMARY KEY,
    username    TEXT NOT NULL,
    password    TEXT NOT NULL,
    UNIQUE(username)
);

/* user roles, e.g. pending, user, moderator, admin, owner */
CREATE TABLE IF NOT EXISTS roles
(
    id      BIGSERIAL PRIMARY KEY,
    name    TEXT,
    UNIQUE(name)
);

/*
Admin defined boundaries for giveaway groups.
Geographical examples: Glencoe, MN, Mcleod County, Downtown Minneapolis.
By interest examples: Furniture giveaways, boys newborn clothing
*/
CREATE TABLE IF NOT EXISTS groups(
  id           BIGSERIAL PRIMARY KEY,
  name         TEXT NOT NULL,
  description  TEXT,
  picture      BYTEA,
  UNIQUE(name)
);

/* If no group id, assumed to be global. */
/* If group id, then applies to that table */
CREATE TABLE IF NOT EXISTS user_roles
(
    user_id      BIGSERIAL NOT NULL REFERENCES users(id),
    role_id      BIGSERIAL NOT NULL REFERENCES roles(id),
    group_id     BIGSERIAL REFERENCES groups(id),
    UNIQUE(user_id, role_id)
);

CREATE TABLE IF NOT EXISTS tokens
(
  id           BIGSERIAL PRIMARY KEY,
  token        TEXT NOT NULL,
  user_id      BIGINT NOT NULL REFERENCES users(id) ON UPDATE CASCADE ON DELETE CASCADE,
  issued       TIMESTAMP WITH TIME ZONE,
  UNIQUE(token)
);
