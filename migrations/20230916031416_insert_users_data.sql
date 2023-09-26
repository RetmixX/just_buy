-- Add migration script here
insert into users(name, surname, patronymic, email, password, role)
VALUES ('Retmix', 'S', 'P', 'retmix@gmail.com', '$2b$10$Tzg3lMgFt8Ipt76TMb845u4qANWWttgMfSWuRbToYDpQYaL9Q27d6', 'ROLE_ADMIN'),
       ('Test', 'Admin', 'Test', 'admin@shop.ru', '$2b$10$Tzg3lMgFt8Ipt76TMb845u4qANWWttgMfSWuRbToYDpQYaL9Q27d6', 'ROLE_ADMIN'),
       ('Test', 'User', 'Test', 'user@shop.ru', '$2b$10$Tzg3lMgFt8Ipt76TMb845u4qANWWttgMfSWuRbToYDpQYaL9Q27d6', 'ROLE_CLIENT');