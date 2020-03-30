-- Your SQL goes here
create table users (
  id serial primary key,
  email varchar not null unique,
  password varchar not null,
  created_at timestamp not null default now(),
  updated_at timestamp not null default now(),
  deleted_at timestamp
);
