-- Add migration script here
create table if not exists carts
(
    id      serial primary key,
    user_id serial                  not null,
    ordered boolean default (false) not null,
    foreign key (user_id) references users (id)
);