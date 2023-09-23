-- Add migration script here
alter table users
    add column token text;