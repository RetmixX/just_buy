-- Add migration script here
insert into users(name, surname, patronymic, email, password, role)
VALUES ('Retmix', 'S', 'P', 'retmix@gmail.com', 'password', 'ROLE_ADMIN'),
       ('Test', 'Admin', 'Test', 'admin@shop.ru', 'password', 'ROLE_ADMIN'),
       ('Test', 'User', 'Test', 'user@shop.ru', 'password', 'ROLE_CLIENT');