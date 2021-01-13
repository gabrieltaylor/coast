CREATE TABLE users (
   id uuid NOT NULL,
   PRIMARY KEY (id),
   name TEXT NOT NULL,
   inserted_at timestamptz NOT NULL,
   updated_at timestamptz NOT NULL
);
