mod structs;

use std::collections::HashMap;
use axum::{response::Html, Json};
use std::fs::File;
use std::io::Read;
use pyo3::{prelude::*, types::{PyModule, PyTuple}};
use structs::*;

fn render(file_name: &'static str, mut params: HashMap<&'static str, &'static str>) -> String {
    let title = *params.get(&"title").unwrap_or(&file_name);
    params.remove("title");

    let mut file = File::open(format!("src/views/{}.html", file_name)).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    for (k, v) in &params {
        content = content.replace(format!("{{{{{}}}}}", k).as_str(), v)
    }

    return render_layout(content, title);
}

fn render_layout<'a>(content: String, title: &'static str) -> String {
    let mut file = File::open("src/views/layout.html").unwrap();
    let mut layout_content = String::new();
    file.read_to_string(&mut layout_content).unwrap();

    layout_content = layout_content.replace("{{title}}", title);
    layout_content = layout_content.replace("{{content}}", content.as_str());

    layout_content
}

pub async fn handle_index() -> Html<String> {
    let title = "Настройка позиции";

    Html(render("index", HashMap::from([
        ("title", title),
        ("hello", "Hello World"),
    ])))
}

pub async fn handle_contacts() -> Html<String> {
    let title = "Контакты";

    Html(render("contacts", HashMap::from([
        ("title", title),
    ])))
}

pub async fn handle_servo1(Json(payload): Json<Servo>) -> Json<AjaxResult> {
    pyo3::prepare_freethreaded_python();
    
    let angle = payload.angle.parse::<f64>().unwrap();
    let code = include_str!("test.py");

    Python::with_gil(|py| {
        let args = PyTuple::new_bound(py, &[angle]);
        let py_fun: Py<PyAny> = PyModule::from_code_bound(py, code, "", "").unwrap().getattr("test").unwrap().into();
        py_fun.call1(py, args).unwrap();
    });

    Json(AjaxResult {
        status: "ok".to_string(),
        response: payload.angle
    })
}

pub async fn handle_servo2(Json(payload): Json<Servo>) -> Json<AjaxResult> {
    pyo3::prepare_freethreaded_python();
    
    let angle = payload.angle.parse::<f64>().unwrap();
    let code = include_str!("test.py");

    Python::with_gil(|py| {
        let args = PyTuple::new_bound(py, &[angle]);
        let py_fun: Py<PyAny> = PyModule::from_code_bound(py, code, "", "").unwrap().getattr("test").unwrap().into();
        py_fun.call1(py, args).unwrap();
    });

    Json(AjaxResult {
        status: "ok".to_string(),
        response: payload.angle
    })
}

pub async fn handle_servo3(Json(payload): Json<Servo>) -> Json<AjaxResult> {
    pyo3::prepare_freethreaded_python();
    
    let angle = payload.angle.parse::<f64>().unwrap();
    let code = include_str!("test.py");

    Python::with_gil(|py| {
        let args = PyTuple::new_bound(py, &[angle]);
        let py_fun: Py<PyAny> = PyModule::from_code_bound(py, code, "", "").unwrap().getattr("test").unwrap().into();
        py_fun.call1(py, args).unwrap();
    });

    Json(AjaxResult {
        status: "ok".to_string(),
        response: payload.angle
    })
}

pub async fn handle_servo4(Json(payload): Json<Servo>) -> Json<AjaxResult> {
    pyo3::prepare_freethreaded_python();
    
    let angle = payload.angle.parse::<f64>().unwrap();
    let code = include_str!("test.py");

    Python::with_gil(|py| {
        let args = PyTuple::new_bound(py, &[angle]);
        let py_fun: Py<PyAny> = PyModule::from_code_bound(py, code, "", "").unwrap().getattr("test").unwrap().into();
        py_fun.call1(py, args).unwrap();
    });

    Json(AjaxResult {
        status: "ok".to_string(),
        response: payload.angle
    })
}

pub async fn handle_servo5(Json(payload): Json<Servo>) -> Json<AjaxResult> {
    pyo3::prepare_freethreaded_python();
    
    let angle = payload.angle.parse::<f64>().unwrap();
    let code = include_str!("test.py");

    Python::with_gil(|py| {
        let args = PyTuple::new_bound(py, &[angle]);
        let py_fun: Py<PyAny> = PyModule::from_code_bound(py, code, "", "").unwrap().getattr("test").unwrap().into();
        py_fun.call1(py, args).unwrap();
    });

    Json(AjaxResult {
        status: "ok".to_string(),
        response: payload.angle
    })
}

pub async fn handle_servo6(Json(payload): Json<Servo>) -> Json<AjaxResult> {
    pyo3::prepare_freethreaded_python();
    
    let angle = payload.angle.parse::<f64>().unwrap();
    let code = include_str!("test.py");

    Python::with_gil(|py| {
        let args = PyTuple::new_bound(py, &[angle]);
        let py_fun: Py<PyAny> = PyModule::from_code_bound(py, code, "", "").unwrap().getattr("test").unwrap().into();
        py_fun.call1(py, args).unwrap();
    });

    Json(AjaxResult {
        status: "ok".to_string(),
        response: payload.angle
    })
}