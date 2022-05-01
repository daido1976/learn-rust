CREATE TABLE subscriptions(
   id bigserial,
   email TEXT NOT NULL UNIQUE,
   name TEXT NOT NULL,
   subscribed_at timestamptz NOT NULL,
   PRIMARY KEY (id)
);
