-- Add migration script here
create table if not exists users
(
    id         serial primary key,
    name       varchar(100)        not null,
    surname    varchar(100)        not null,
    patronymic varchar(100),
    email      varchar(100) unique not null,
    password   varchar(100)        not null
);