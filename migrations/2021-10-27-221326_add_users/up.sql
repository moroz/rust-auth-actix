-- Your SQL goes here
create table users (
  id serial primary key,
  first_name text not null,
  last_name text not null,
  email text not null,
  created_at timestamp not null default (now() at time zone 'utc')
);
