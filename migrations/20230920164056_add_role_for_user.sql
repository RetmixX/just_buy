-- Add migration script here
alter table users
add column role varchar(30) not null default 'ROLE_CLIENT'