mod structs;

use std::collections::HashMap;
use axum::{
    extract::Multipart, 
    response::Html, 
    Json};
use std::fs::File;
use std::io::Read;
use pyo3::{prelude::*, types::{PyModule, PyTuple}};
use structs::*;

/// Рендер HTML страницы
/// * `file_name` - Название HTML-файла (только название, без пути!)
/// * `params` - Значения, которые необходимо вставить в HTML из Rust
/// 
/// Указание параметров:
/// 
/// `("title", some_title_name)` - после этого в соответсвующем HTML нужно указать плейсхолдер вида `{{title}}`
fn render(file_name: &'static str, mut params: HashMap<&'static str, &'static str>) -> String {
    let title = *params.get(&"title").unwrap_or(&file_name); //Получаем название вкладки. Если ее нет в params, то названием будет являться название файла
    params.remove("title");

    let mut file = File::open(format!("src/views/{}.html", file_name)).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap(); //Весь контент файла записываем в строку

    for (k, v) in &params {
        content = content.replace(format!("{{{{{}}}}}", k).as_str(), v) //Заменяем плейсхолдеры на значения
    }

    return render_layout(content, title);
}


/// Вставка контента HTML внутрь шаблонного HTML (например, для отображения header и footer без их дублирования в каждом файле)
/// * `content` - Контент HTML страницы
/// * `title` - Название вкладки
fn render_layout<'a>(content: String, title: &'static str) -> String {
    let mut file = File::open("src/views/layout.html").unwrap();
    let mut layout_content = String::new();
    file.read_to_string(&mut layout_content).unwrap();

    layout_content = layout_content.replace("{{title}}", title);
    layout_content = layout_content.replace("{{content}}", content.as_str());

    layout_content
}

///Слушатель для `/index`
/// 
/// Метод `GET`
pub async fn handle_index() -> Html<String> {
    let title = "Настройка позиции";

    Html(render("index", HashMap::from([
        ("title", title),
        ("hello", "Hello World"),
    ])))
}

///Слушатель для `/contacts`
/// 
/// Метод `GET`
pub async fn handle_contacts() -> Html<String> {
    let title = "Контакты";

    Html(render("contacts", HashMap::from([
        ("title", title),
    ])))
}

///Слушатель для `/servo`
/// 
/// Метод `POST`
/// 
/// Метод предназначен для задания угла отдельного servo робота
/// * `payload` - Запрос
pub async fn handle_servo(Json(payload): Json<Servo>) -> Json<AjaxResult> {
    pyo3::prepare_freethreaded_python();
    
    let angle = payload.angle.parse::<i32>().unwrap();
    let servo = payload.servo.parse::<i32>().unwrap();
    let code = include_str!("py/control.py");

    Python::with_gil(|py| {
        let args = PyTuple::new_bound(py, &[servo, angle]); //Второй парметр - передача значений в Python-функцию
        let py_fun: Py<PyAny> = PyModule::from_code_bound(py, code, "", "").unwrap().getattr("control").unwrap().into();
        py_fun.call1(py, args).unwrap();
    });

    Json(AjaxResult {
        status: "ok".to_string(),
        response: format!("Servo {}, angle: {} success", servo, angle)
    })
}

///Слушатель для `/pose`
/// 
/// Метод `POST`
/// 
/// Метод предназначен для задания готовой позиции робота
/// * `payload` - Запрос
pub async fn handle_pose(Json(payload): Json<Pose>) -> Json<AjaxResult> {
    pyo3::prepare_freethreaded_python();
    
    let pose = payload.pose.parse::<i32>().unwrap();
    let servo6 = payload.servo6.parse::<i32>().unwrap();
    let code = include_str!("py/control_pose.py");
    let mut res = String::new();

    Python::with_gil(|py| {
        let args = PyTuple::new_bound(py, &[pose, servo6]); //Второй парметр - передача значений в Python-функцию
        let py_fun: Py<PyAny> = PyModule::from_code_bound(py, code, "", "").unwrap().getattr("control_pose").unwrap().into();
        let py_res = py_fun.call1(py, args).unwrap(); //Получаем результат, вернувшийся из Python-функции
        res = format!("{py_res}");
    });

    Json(AjaxResult {
        status: "ok".to_string(),
        response: res
    })
}

///Слушатель для `/trajectory`
/// 
/// Метод `POST`
/// 
/// Метод предназначен для прохождения траектории из заданного файла
/// * `multipart` - Отправленная форма запроса
pub async fn handle_trajectory(mut multipart: Multipart) -> Json<AjaxResult> {
    pyo3::prepare_freethreaded_python();

    let file = multipart.next_field().await.unwrap().unwrap();
    let data_file = file.bytes().await.unwrap();

    let servo6 = multipart.next_field().await.unwrap().unwrap();
    let data_sevor6 = servo6.bytes().await.unwrap();

    let code = include_str!("py/trajectory.py");
    let mut res = String::new();

    Python::with_gil(|py| {
        let args = PyTuple::new_bound(py, &[String::from_utf8(data_file.to_vec()).unwrap(), String::from_utf8(data_sevor6.to_vec()).unwrap()]); //Второй парметр - передача значений в Python-функцию
        let py_fun: Py<PyAny> = PyModule::from_code_bound(py, code, "", "").unwrap().getattr("trajectory").unwrap().into();
        let py_res = py_fun.call1(py, args).unwrap(); //Получаем результат, вернувшийся из Python-функции
        res = format!("{py_res}");
    });

    Json(AjaxResult {
        status: "ok".to_string(),
        response: res
    })
}