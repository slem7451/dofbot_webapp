use serde::{Deserialize, Serialize};

///Структура для преобразования запроса в `/servo`
/// * `angle` - Угол в градусах `[-180;180]`
/// * `servo` - Номер servo
#[derive(Deserialize)]
pub struct Servo {
    pub angle: String,
    pub servo: String,
}

///Структура для преобразования запроса в `/pose`
/// * `pose` - ID позы
/// * `servo6` - Значение servo 6 (клешни). Нужно для того, чтобы сохранить данное значение у робота
#[derive(Deserialize)]
pub struct Pose {
    pub pose: String,
    pub servo6: String,
}

///Структура для преобразования ответа на запрос
/// * `status` - Статус запроса (ошибка, успех и т.д.)
/// * `response` - Тело ответа
#[derive(Serialize)]
pub struct AjaxResult {
    pub status: String,
    pub response: String,
}
