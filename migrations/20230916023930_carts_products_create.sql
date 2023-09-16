-- Add migration script here
create table if not exists carts_products
(
    id         serial primary key,
    cart_id    serial not null,
    product_id serial not null,
    foreign key (product_id) references products (id),
    foreign key (cart_id) references carts (id)
);