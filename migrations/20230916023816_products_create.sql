-- Add migration script here
create table if not exists products
(
    id          serial primary key,
    name        varchar(100) not null,
    description text,
    price       int          not null
);