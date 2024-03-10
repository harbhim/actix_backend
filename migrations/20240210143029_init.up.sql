CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS public.users (
  id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4 ()),
  first_name varchar(255) NULL,
  last_name varchar(255) NULL,
  email varchar(255) UNIQUE NOT NULL,
  password varchar(2000) NOT NULL,
  created_at TIMESTAMP
  WITH
    TIME ZONE DEFAULT NOW (),
    updated_at TIMESTAMP
  WITH
    TIME ZONE DEFAULT NOW ()
);
