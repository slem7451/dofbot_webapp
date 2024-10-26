# Веб-приложение для Dofbot
### Запуск
Без логирования в консоль:
```
cargo run
```
С логированием  в консоль:
```
RUS_LOG=info cargo run
RUS_LOG=debug cargo run
```
### Логирование
Для логирования в файл необходимо подключить соответствующий пакет:
```
use log::info; //Или use log::debug;
use log::{info, debug}; //Или все вместе
```
После этого доступны макросы **info!** и/или **debug!**
### Структура проекта
```
dofbot_webapp
│   .gitignore
│   Cargo.lock
│   Cargo.toml
│   README.md
│
└───src
    │   app.rs    #Функции для запуска сервера и всего необходимого для работы
    │   main.rs   #Точка входа, здесь запускается сервер
    │
    ├───app
    │   │   controller.rs      #Слушатели для каждого URL приложения
    │   │
    │   ├───controller
    │   │       structs.rs     #Необходимые структуры для запросов и ответов
    │   │
    │   └───py   #Python-скрипты
    │           control.py
    │           control_pose.py
    │
    ├───static   #Статические файлы. Картинки, скрипты, стили и т.д.
    │   │   favicon.ico
    │   │
    │   ├───images
    │   │   └───poses
    │   │           lay.png
    │   │           snake.png
    │   │           standart.png
    │   │           zigzag.png
    │   │
    │   └───scripts
    │           jquery-3.7.1.min.js
    │
    └───views    #HTML для страниц
            contacts.html
            index.html
            layout.html      #HTML внутри которого вставляется контент других HTML
```