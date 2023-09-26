# just_buy
Окружение запускается через docker compose в папке .build
В папке .build находится .env.example из которого надо скопировать все переменные окружения в .env и заполнить данными
 - Переменная DB_URL должна быть - postgres://{user}:{password}@db:{host}/{database}

В папке collection находится документация для постмана выгруженная из сваггера
В проект включена swagger-документация, которая доступна по /swagger-ui/
В базе данных есть заготовленные пользователи:
- email: admin@shop.ru | password: password
- email: user@shop.ru | password: password

Доступные пути:
- api/products
- api/login
- api/signup
- api/logout
- api/order
- api/carts
- / - secret
