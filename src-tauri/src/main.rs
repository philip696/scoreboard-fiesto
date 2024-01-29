// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use serialport::SerialPort;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager, State};
use tokio::time::{sleep, Duration};

mod rabbitmq;
mod serial;
mod utils;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct AppState {
    rabbitmq_host: String,
    rabbitmq_username: String,
    rabbitmq_password: String,
    event_id: String,
    field_id: String,
    scorer_url: String,
    dark_statistic_url: String,
    light_statistic_url: String,
    man_of_the_match_url: String,
    top_player_url: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct ConfigState {
    file_path: String,
}

#[tauri::command]
fn list_serial_ports() -> Result<Vec<String>, String> {
    serial::list_ports()
}

#[tauri::command]
fn connect_serial_port(
    port_name: String,
    // port_map: State<serial::PortMap>,
) -> Result<String, String> {
    match serial::connect_port(&port_name) {
        Ok(_) => Ok(format!("Successfully connected to {}", port_name)),
        Err(e) => Err(e),
    }
}

#[tauri::command]
fn disconnect_serial_port(port_map: State<serial::PortMap>) -> Result<String, String> {
    serial::disconnect_all_ports(&port_map)
        .map(|_| "Successfully disconnected from all ports".to_string())
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn trigger_alarm(port_name: String, duration: String) -> Result<String, String> {
    let duration: u64 = match duration.parse::<u64>() {
        Ok(num) => num,
        Err(e) => return Err(e.to_string()),
    };

    match serial::trigger_serial_alarm(&port_name, duration).await {
        Ok(_) => Ok("Successfully triggered alarm".to_string()),
        Err(e) => Err(e.to_string()),
    }
}

// async fn produce_to_rabbitmq(
//     host: String,
//     username: String,
//     password: String,
//     routing_key: String,
//     message: String,
// ) -> Result<(), String> {
//     let amqp_uri = format!("amqp://{}:{}@{}:5672", username, password, host); //state.lock().unwrap().rabbitmq_host.to_string(); // Replace with your URI

//     // Connect to the AMQP server
//     let conn = Connection::connect(&amqp_uri, ConnectionProperties::default())
//         .await
//         .expect("Failed to connect to AMQP server");

//     let channel = conn
//         .create_channel()
//         .await
//         .expect("Failed to create a channel");

//     channel
//         .basic_publish(
//             "amq.topic",  // Exchange
//             &routing_key, // Routing key (queue name)
//             BasicPublishOptions::default(),
//             &message.as_bytes().to_vec(),
//             BasicProperties::default(),
//         )
//         .await
//         .map_err(|e| e.to_string())
//         .map_err(|e| e.to_string())
//         .unwrap();

//     Ok(())
// }

#[tauri::command]
fn update_time(time: String, state: tauri::State<'_, Arc<Mutex<AppState>>>) -> Result<(), String> {
    let state = state.lock().unwrap();
    let host = state.rabbitmq_host.clone();
    let username = state.rabbitmq_username.clone();
    let password = state.rabbitmq_password.clone();
    let routing_key = format!("sportkit.basket.{}.{}.time", state.event_id, state.field_id);

    tokio::spawn(async move {
        let _ = rabbitmq::produce_to_rabbitmq(host, username, password, routing_key, time).await;
    });

    Ok(())
}

#[tauri::command]
fn update_quarter(
    quarter: String,
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
) -> Result<(), String> {
    let state = state.lock().unwrap();
    let host = state.rabbitmq_host.clone();
    let username = state.rabbitmq_username.clone();
    let password = state.rabbitmq_password.clone();
    let routing_key = format!(
        "sportkit.basket.{}.{}.quarter",
        state.event_id, state.field_id
    );

    tokio::spawn(async move {
        let _ = rabbitmq::produce_to_rabbitmq(host, username, password, routing_key, quarter).await;
    });

    Ok(())
}

// Tauri command to update the state
#[tauri::command]
fn save_config(
    rabbitmq_host: String,
    rabbitmq_username: String,
    rabbitmq_password: String,
    event_id: String,
    field_id: String,
    scorer_url: String,
    dark_statistic_url: String,
    light_statistic_url: String,
    man_of_the_match_url: String,
    top_player_url: String,
    app_state: tauri::State<'_, Arc<Mutex<AppState>>>,
    config_state: tauri::State<'_, Arc<Mutex<ConfigState>>>,
) -> Result<(), String> {
    let mut app_state = app_state.lock().unwrap();
    app_state.rabbitmq_host = rabbitmq_host;
    app_state.rabbitmq_username = rabbitmq_username;
    app_state.rabbitmq_password = rabbitmq_password;
    app_state.event_id = event_id;
    app_state.field_id = field_id;
    app_state.scorer_url = scorer_url;
    app_state.dark_statistic_url = dark_statistic_url;
    app_state.light_statistic_url = light_statistic_url;
    app_state.man_of_the_match_url = man_of_the_match_url;
    app_state.top_player_url = top_player_url;
    let relative_path = config_state.lock().unwrap().file_path.to_string();
    let file_path = utils::get_absolute_path_in_home(&relative_path).unwrap();

    let _ = utils::ensure_file_exists(&file_path);

    match serde_json::to_string(&*app_state) {
        Ok(serialized) => match std::fs::write(file_path, &serialized) {
            Ok(_) => {
                return Ok(());
            }
            Err(e) => {
                let err_msg = format!("Failed to write config file: {}", e);
                println!("{}", &err_msg);
                return Err(err_msg);
            }
        },
        Err(e) => {
            let err_msg = format!("Failed to serialize state: {}", e);
            println!("{}", &err_msg);
            return Err(err_msg);
        }
    };
    // Optionally, log or do something after updating the state
}

#[tauri::command]
fn open_config(app: AppHandle) -> Result<(), String> {
    let config_window = app.get_window("configurationpage").unwrap();
    config_window.show().unwrap();
    Ok(())
}

#[tauri::command]
fn end_config(app: AppHandle) -> Result<(), String> {
    let config_window = app.get_window("configurationpage").unwrap();
    let main_window = app.get_window("indexpage").unwrap();
    let controller_window = app.get_window("controllerpage").unwrap();
    config_window.hide().unwrap();
    main_window.show().unwrap();
    controller_window.show().unwrap();
    Ok(())
}

#[tauri::command]
fn get_config(state: tauri::State<'_, Arc<Mutex<AppState>>>) -> Result<AppState, String> {
    let state = state.lock().map_err(|e| e.to_string())?;
    Ok(state.clone())
}

#[tauri::command]
fn close_all_processes() {
    std::process::exit(0);
}

#[tokio::main]
async fn main() {
    let config_state = ConfigState {
        file_path: "BasketScoreboard/settings.json".to_string(),
    };

    let file_path = utils::get_absolute_path_in_home(&config_state.file_path).unwrap();

    let app_state = match utils::read_file(&file_path) {
        Ok(contents) => match serde_json::from_str::<AppState>(&contents) {
            Ok(data) => {
                println!("Deserialized data: {:?}", data);
                data
            }
            Err(e) => {
                println!("Error parsing JSON: {}", e);
                AppState {
                    rabbitmq_host: "initial_host".to_string(),
                    rabbitmq_username: "initial_username".to_string(),
                    rabbitmq_password: "initial_password".to_string(),
                    event_id: "initial_event_id".to_string(),
                    field_id: "initial_field_id".to_string(),
                    scorer_url: "initial_scorer_url".to_string(),
                    dark_statistic_url: "initial_dark_statistic_url".to_string(),
                    light_statistic_url: "initial_light_statistic_url".to_string(),
                    man_of_the_match_url: "initial_man_of_the_match_url".to_string(),
                    top_player_url: "initial_top_player_url".to_string(),
                }
            }
        },
        Err(e) => {
            println!("Error reading file: {}", e);
            AppState {
                rabbitmq_host: "initial_host".to_string(),
                rabbitmq_username: "initial_username".to_string(),
                rabbitmq_password: "initial_password".to_string(),
                event_id: "initial_event_id".to_string(),
                field_id: "initial_field_id".to_string(),
                scorer_url: "initial_scorer_url".to_string(),
                dark_statistic_url: "initial_dark_statistic_url".to_string(),
                light_statistic_url: "initial_light_statistic_url".to_string(),
                man_of_the_match_url: "initial_man_of_the_match_url".to_string(),
                top_player_url: "initial_top_player_url".to_string(),
            }
        }
    };

    tauri::Builder::default()
        .manage({
            let state = Arc::new(Mutex::new(app_state));
            state
        })
        .manage({
            let state = Arc::new(Mutex::new(config_state));
            state
        })
        .manage(Arc::new(Mutex::new(
            HashMap::<String, Box<dyn SerialPort>>::new(),
        )))
        .invoke_handler(tauri::generate_handler![
            update_time,
            update_quarter,
            list_serial_ports,
            connect_serial_port,
            disconnect_serial_port,
            trigger_alarm,
            save_config,
            get_config,
            open_config,
            end_config,
            close_all_processes
        ])
        .setup(|app| {
            let splashscreen_window = app.get_window("splashscreen").unwrap();
            // let controller_window = app.get_window("controllerpage").unwrap();
            // let main_windows = app.get_window("indexpage").unwrap();
            let configuration_windows = app.get_window("configurationpage").unwrap();

            tauri::async_runtime::spawn(async move {
                sleep(Duration::from_secs(1)).await;

                splashscreen_window.close().unwrap();
                configuration_windows.show().unwrap();
                // main_windows.show().unwrap();
                // controller_window.show().unwrap();
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
