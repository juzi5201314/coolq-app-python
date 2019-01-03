#![feature(specialization)]
#![feature(toowned_clone_into)]
extern crate coolq_sdk_rust;
#[macro_use]
extern crate pyo3;
#[macro_use]
extern crate lazy_static;

use coolq_sdk_rust::cqpapi;

use pyo3::{IntoPyTuple, prelude::*, types::*};
use std::{collections::HashMap, fs::{create_dir_all, read_dir}, path::Path, sync::Mutex, thread};

static APPID: &'static str = "net.systir.coolq_app_python";
static VERSION: i32 = 9;

lazy_static! {
    static ref listeners: Mutex<HashMap<String, Vec<PyObject>>> = Mutex::new(HashMap::new());
    static ref modules: Mutex<HashMap<String, HashMap<String, String>>> = Mutex::new(HashMap::new());
}


#[no_mangle]
pub extern "Rust" fn appinfo() -> (i32, String) {
    (VERSION, String::from(APPID))
}

#[no_mangle]
#[allow(unused_variables)]
pub unsafe extern "Rust" fn start() -> i32 {

    fn check_runtime_error<T>(re: Result<T, PyErr>) -> T {
        match re {
            Ok(ok) => ok,
            Err(e) => {
                let gil = Python::acquire_gil();
                let py = gil.python();
                e.print(py);
                alert::alert("python应用出现错误", "具体错误信息请查看酷q运行日志");
                panic!("python runtime error!")
            },
        }
    }

    cqpapi::ON_PRIVATE_MESSAGE = Some(Box::new(|sub_type: i32, send_time: i32, user_id: i64, msg: String, font: i32| -> i32 {
        let map = listeners.lock().unwrap();
        if map.contains_key("on_private_message") {
            for obj in map.get("on_private_message").unwrap() {
                let gil = Python::acquire_gil();
                let py = gil.python();
                check_runtime_error(obj.call1(py, (sub_type, send_time, user_id, msg.as_str(), font).into_tuple(py)));
            }
        }
        cqpapi::EVENT_IGNORE
    }));
    cqpapi::ON_ENABLE = Some(Box::new(|| -> i32 {
        let map = listeners.lock().unwrap();
        if map.contains_key("enable") {
            for obj in map.get("enable").unwrap() {
                let gil = Python::acquire_gil();
                let py = gil.python();
                check_runtime_error(obj.call1(py, ().into_tuple(py)));
            }
        }
        cqpapi::EVENT_IGNORE
    }));
    cqpapi::ON_DISABLE = Some(Box::new(|| -> i32 {
        let map = listeners.lock().unwrap();
        if map.contains_key("disable") {
            for obj in map.get("disable").unwrap() {
                let gil = Python::acquire_gil();
                let py = gil.python();
                check_runtime_error(obj.call1(py, ().into_tuple(py)));
            }
        }
        cqpapi::EVENT_IGNORE
    }));
    cqpapi::ON_EXIT = Some(Box::new(|| -> i32 {
        let map = listeners.lock().unwrap();
        if map.contains_key("exit") {
            for obj in map.get("exit").unwrap() {
                let gil = Python::acquire_gil();
                let py = gil.python();
                check_runtime_error(obj.call1(py, ().into_tuple(py)));
            }
        }
        cqpapi::EVENT_IGNORE
    }));
    cqpapi::ON_GROUP_MESSAGE = Some(Box::new(|sub_type: i32, send_time: i32, group_id: i64, user_id: i64, anonymous: String, msg: String, font: i32| -> i32 {
        let map = listeners.lock().unwrap();
        if map.contains_key("on_group_message") {
            for obj in map.get("on_group_message").unwrap() {
                let gil = Python::acquire_gil();
                let py = gil.python();
                check_runtime_error(obj.call1(py, (sub_type, send_time, group_id, user_id, anonymous.as_str(), msg.as_str(), font).into_tuple(py)));
            }
        }
        cqpapi::EVENT_IGNORE
    }));
    cqpapi::ON_DISCUSS_MESSAGE = Some(Box::new(|sub_type: i32, send_time: i32, discuss_id: i64, user_id: i64, msg: String, font: i32| -> i32 {
        let map = listeners.lock().unwrap();
        if map.contains_key("on_discuss_message") {
            for obj in map.get("on_discuss_message").unwrap() {
                let gil = Python::acquire_gil();
                let py = gil.python();
                check_runtime_error(obj.call1(py, (sub_type, send_time, discuss_id, user_id, msg.as_str(), font).into_tuple(py)));
            }
        }
        cqpapi::EVENT_IGNORE
    }));
    cqpapi::ON_GROUP_ADMIN = Some(Box::new(|sub_type: i32, send_time: i32, group_id: i64, user_id: i64| -> i32 {
        let map = listeners.lock().unwrap();
        if map.contains_key("on_group_admin") {
            for obj in map.get("on_group_admin").unwrap() {
                let gil = Python::acquire_gil();
                let py = gil.python();
                check_runtime_error(obj.call1(py, (sub_type, send_time, group_id, user_id).into_tuple(py)));
            }
        }
        cqpapi::EVENT_IGNORE
    }));
    cqpapi::ON_GROUP_MEMBER_DECREASE = Some(Box::new(|sub_type: i32, send_time: i32, group_id: i64, from_user_id: i64, user_id: i64| -> i32 {
        let map = listeners.lock().unwrap();
        if map.contains_key("on_group_member_decrease") {
            for obj in map.get("on_group_member_decrease").unwrap() {
                let gil = Python::acquire_gil();
                let py = gil.python();
                check_runtime_error(obj.call1(py, (sub_type, send_time, group_id, from_user_id, user_id).into_tuple(py)));
            }
        }
        cqpapi::EVENT_IGNORE
    }));
    cqpapi::ON_GROUP_MEMBER_INCREASE = Some(Box::new(|sub_type: i32, send_time: i32, group_id: i64, from_user_id: i64, user_id: i64| -> i32 {
        let map = listeners.lock().unwrap();
        if map.contains_key("on_group_member_increase") {
            for obj in map.get("on_group_member_increase").unwrap() {
                let gil = Python::acquire_gil();
                let py = gil.python();
                check_runtime_error(obj.call1(py, (sub_type, send_time, group_id, from_user_id, user_id).into_tuple(py)));
            }
        }
        cqpapi::EVENT_IGNORE
    }));
    cqpapi::ON_FRIEND_ADD = Some(Box::new(|sub_type: i32, send_time: i32, user_id: i64| -> i32 {
        let map = listeners.lock().unwrap();
        if map.contains_key("on_friend_add") {
            for obj in map.get("on_friend_add").unwrap() {
                let gil = Python::acquire_gil();
                let py = gil.python();
                check_runtime_error(obj.call1(py, (sub_type, send_time, user_id).into_tuple(py)));
            }
        }
        cqpapi::EVENT_IGNORE
    }));
    cqpapi::ON_ADD_FRIEND = Some(Box::new(|sub_type: i32, send_time: i32, user_id: i64, msg: String, flag: String| -> i32 {
        let map = listeners.lock().unwrap();
        if map.contains_key("on_add_friend") {
            for obj in map.get("on_add_friend").unwrap() {
                let gil = Python::acquire_gil();
                let py = gil.python();
                check_runtime_error(obj.call1(py, (sub_type, send_time, user_id, msg.as_str(), flag.as_str()).into_tuple(py)));
            }
        }
        cqpapi::EVENT_IGNORE
    }));
    cqpapi::ON_ADD_GROUP = Some(Box::new(|sub_type: i32, send_time: i32, group_id: i64, user_id: i64, msg: String, flag: String| -> i32 {
        let map = listeners.lock().unwrap();
        if map.contains_key("on_add_group") {
            for obj in map.get("on_add_group").unwrap() {
                let gil = Python::acquire_gil();
                let py = gil.python();
                check_runtime_error(obj.call1(py, (sub_type, send_time, group_id, user_id, msg.as_str(), flag.as_str()).into_tuple(py)));
            }
        }
        cqpapi::EVENT_IGNORE
    }));


    cqpapi::ON_ENABLE = Some(Box::new(|| {
        let dir = cqpapi::get_app_directory();
        create_dir_all(format!("{}/py_script", dir)).unwrap();
        let gil = Python::acquire_gil();
        let py = gil.python();
        let syspath: &PyList = py.import("sys").unwrap().get("path").unwrap().try_into().unwrap();
        syspath.insert(0, format!("{}/py_script", dir)).unwrap();
        let path = format!("{}/py_script", dir);
        let path = Path::new(path.as_str());
        for entry in read_dir(path).unwrap() {
            let p = entry.unwrap().path();
            if p.is_dir() {
                let name = p.file_name().unwrap().to_str().unwrap();
                let test = py.import(name);
                let test = match test {
                    Ok(ok) => ok,
                    Err(e) => {
                        alert::alert(&format!("{}模块加载失败", name), "请检查python环境与是否有语法错误");
                        continue
                    },
                };
                test.add_class::<ErrOut>().unwrap();
                test.add_wrapped(wrap_module!(coolq_sdk)).unwrap();
                let r = check_runtime_error(test.call1("main", ().into_tuple(py)));
                let dict: &PyDict = <&PyDict as FromPyObject>::extract(&r).unwrap();
                let mut m = modules.lock().unwrap();
                let n = name.to_string();
                let mut map: HashMap<String, String> = HashMap::new();
                for (k, v) in dict.iter() {
                    let key = <&PyString as FromPyObject>::extract(&k).unwrap();
                    let value = <&PyString as FromPyObject>::extract(&v).unwrap();
                    map.insert(key.to_string().unwrap().to_string(), value.to_string().unwrap().to_string());
                }
                m.insert(n, map);
                cqpapi::add_log(cqpapi::CQLOG_INFOSUCCESS, "info", &format!("python模块: {}加载完成", name));
            }
        }

        cqpapi::EVENT_IGNORE
    }));
    cqpapi::EVENT_IGNORE
}

#[pyclass]
struct Logger {}

#[pymethods]
impl Logger {
    #[new]
    fn __new__(obj: &PyRawObject) -> PyResult<()> {
        obj.init(|| Logger {})
    }

    fn write(&self, str: &str) -> i32 {
        if !str.trim().is_empty() {
            cqpapi::add_log(cqpapi::CQLOG_INFOSUCCESS, "info", str)
        }else {
            0
        }
    }
}

#[pyclass]
struct ErrOut {}

#[pymethods]
impl ErrOut {
    #[new]
    fn __new__(obj: &PyRawObject) -> PyResult<()> {
        obj.init(|| ErrOut {})
    }

    fn write(&self, str: &str) -> i32 {
        if !str.trim().is_empty() {
            cqpapi::add_log(cqpapi::CQLOG_ERROR, "python", str)
        }else {
            0
        }
    }
}

#[allow(dead_code)]
#[warn(unused_must_use)]
#[pymodule]
fn coolq_sdk(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfunction]
    fn send_private_msg(user_id: i64, msg: &str) -> i32 {
        cqpapi::send_private_msg(user_id, msg)
    }
    #[pyfunction]
    fn send_group_msg(group_id: i64, msg: &str) -> i32 {
        cqpapi::send_group_msg(group_id, msg)
    }
    #[pyfunction]
    fn send_discuss_msg(discussio_id: i64, msg: &str) -> i32 {
        cqpapi::send_discuss_msg(discussio_id, msg)
    }
    #[pyfunction]
    fn delete_msg(message_id: i64) -> i32 {
        cqpapi::delete_msg(message_id)
    }
    #[pyfunction]
    fn send_like(user_id: i64) -> i32 {
        cqpapi::send_like(user_id)
    }
    #[pyfunction]
    fn set_group_kick(group_id: i64, user_id: i64, refuse_rejoin: bool) -> i32 {
        cqpapi::set_group_kick(group_id, user_id, refuse_rejoin)
    }
    #[pyfunction]
    fn set_group_admin(group_id: i64, user_id: i64, become_admin: bool) -> i32 {
        cqpapi::set_group_admin(group_id, user_id, become_admin)
    }
    #[pyfunction]
    fn set_group_title(group_id: i64, user_id: i64, title: &str, time: i64) -> i32 {
        cqpapi::set_group_title(group_id, user_id, title, time)
    }
    #[pyfunction]
    fn set_group_whole_ban(group_id: i64, enable: bool) -> i32 {
        cqpapi::set_group_whole_ban(group_id, enable)
    }
    #[pyfunction]
    fn set_group_anonymous_ban(group_id: i64, anonymous_name: &str, time: i64) -> i32 {
        cqpapi::set_group_anonymous_ban(group_id, anonymous_name, time)
    }
    #[pyfunction]
    fn set_group_anonymous(group_id: i64, enable: bool) -> i32 {
        cqpapi::set_group_anonymous(group_id, enable)
    }
    #[pyfunction]
    fn set_group_card(group_id: i64, user_id: i64, nickname: &str) -> i32 {
        cqpapi::set_group_card(group_id, user_id, nickname)
    }
    #[pyfunction]
    fn set_group_leave(group_id: i64, dispose: bool) -> i32 {
        cqpapi::set_group_leave(group_id, dispose)
    }
    #[pyfunction]
    fn set_discuss_leave(discussio_id: i64) -> i32 {
        cqpapi::set_discuss_leave(discussio_id)
    }
    #[pyfunction]
    fn set_friend_add_request(flag: &str, response: i32, comment: &str) -> i32 {
        cqpapi::set_friend_add_request(flag, response, comment)
    }
    #[pyfunction]
    fn set_group_add_request_v2(flag: &str, request: i32, response: i32, reason: &str) -> i32 {
        cqpapi::set_group_add_request_v2(flag, request, response, reason)
    }
    #[pyfunction]
    fn get_group_member_info_v2(group_id: i64, user_id: i64, use_cache: bool) -> String {
        cqpapi::get_group_member_info_v2(group_id, user_id, use_cache)
    }
    #[pyfunction]
    fn get_group_member_list(group_id: i64) -> String {
        cqpapi::get_group_member_list(group_id)
    }
    #[pyfunction]
    fn get_group_list() -> String {
        cqpapi::get_group_list()
    }
    #[pyfunction]
    fn get_stranger_info(user_id: i64, use_cache: bool) -> String {
        cqpapi::get_stranger_info(user_id, use_cache)
    }
    #[pyfunction]
    fn add_log(priority: i32, tag: &str, msg: &str) -> i32 {
        cqpapi::add_log(priority, tag, msg)
    }
    #[pyfunction]
    fn get_cookies() -> String {
        cqpapi::get_cookies()
    }
    #[pyfunction]
    fn get_csrf_token() -> i32 {
        cqpapi::get_csrf_token()
    }
    #[pyfunction]
    fn get_login_qq() -> i64 {
        cqpapi::get_login_qq()
    }
    #[pyfunction]
    fn get_login_nick() -> String {
        cqpapi::get_login_nick()
    }
    #[pyfunction]
    fn get_app_directory() -> String {
        cqpapi::get_app_directory()
    }
    #[pyfunction]
    fn set_fatal(error_message: &str) -> i32 {
        cqpapi::set_fatal(error_message)
    }
    #[pyfunction]
    fn get_record(file: &str, outformat: &str) -> String {
        cqpapi::get_record(file, outformat)
    }

    #[pyfunction]
    #[allow(unused_must_use)]
    unsafe fn register(event: &str, func: PyObject) -> i32 {
        let mut map = listeners.lock().unwrap();
        if !map.contains_key(event) {
            map.insert(event.to_string(), Vec::new());
        }
        map.get_mut(&event.to_string()).unwrap().push(func);
        0
    }

    #[pyfunction]
    fn alert(title: &str, msg: &str) -> i32 {
        let title = String::from(title);
        let msg = String::from(msg);
        thread::spawn(move || {
            alert::alert(title.as_str(), msg.as_str());
        });
        0
    }

    m.add_class::<Logger>()?;
    m.add_wrapped(wrap_function!(send_private_msg))?;
    m.add_wrapped(wrap_function!(send_group_msg))?;
    m.add_wrapped(wrap_function!(send_discuss_msg))?;
    m.add_wrapped(wrap_function!(delete_msg))?;
    m.add_wrapped(wrap_function!(send_like))?;
    m.add_wrapped(wrap_function!(set_group_kick))?;
    m.add_wrapped(wrap_function!(set_group_admin))?;
    m.add_wrapped(wrap_function!(set_group_title))?;
    m.add_wrapped(wrap_function!(set_group_whole_ban))?;
    m.add_wrapped(wrap_function!(set_group_anonymous_ban))?;
    m.add_wrapped(wrap_function!(set_group_anonymous))?;
    m.add_wrapped(wrap_function!(set_group_card))?;
    m.add_wrapped(wrap_function!(set_group_leave))?;
    m.add_wrapped(wrap_function!(set_discuss_leave))?;
    m.add_wrapped(wrap_function!(set_friend_add_request))?;
    m.add_wrapped(wrap_function!(set_group_add_request_v2))?;
    m.add_wrapped(wrap_function!(get_group_member_info_v2))?;
    m.add_wrapped(wrap_function!(get_group_member_list))?;
    m.add_wrapped(wrap_function!(get_group_list))?;
    m.add_wrapped(wrap_function!(get_stranger_info))?;
    m.add_wrapped(wrap_function!(add_log))?;
    m.add_wrapped(wrap_function!(get_cookies))?;
    m.add_wrapped(wrap_function!(get_csrf_token))?;
    m.add_wrapped(wrap_function!(get_login_qq))?;
    m.add_wrapped(wrap_function!(get_login_nick))?;
    m.add_wrapped(wrap_function!(get_app_directory))?;
    m.add_wrapped(wrap_function!(set_fatal))?;
    m.add_wrapped(wrap_function!(get_record))?;
    m.add_wrapped(wrap_function!(register))?;
    m.add_wrapped(wrap_function!(alert))?;

    Ok(())
}