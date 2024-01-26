use serialport::{available_ports, SerialPort, SerialPortInfo};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub type PortMap = Arc<Mutex<HashMap<String, Box<dyn SerialPort>>>>;

pub fn list_ports() -> Result<Vec<String>, String> {
    match available_ports() {
        Ok(ports) => Ok(ports.into_iter().map(|p| p.port_name).collect()),
        Err(e) => Err(e.to_string()),
    }
}

pub fn connect_port(port_name: &str, port_map: &PortMap) -> Result<String, String> {
    if is_arduino(port_name) {
        let serial_port = serialport::new(port_name, 9600)
            .open()
            .map_err(|e| e.to_string())?;
        port_map
            .lock()
            .unwrap()
            .insert(port_name.to_string(), serial_port);
        Ok(format!("Connected to Arduino on {}", port_name))
    } else {
        Err("The device is not a valid Arduino".to_string())
    }
}

fn is_arduino(port_name: &str) -> bool {
    println!("Checking if {} is an Arduino", port_name);
    let timeout = Duration::from_millis(1000);
    println!("Checking if {} is an Arduino", port_name);
    let mut port = match serialport::new(port_name, 9600).timeout(timeout).open() {
        Ok(p) => p,
        Err(_) => return false,
    };
    println!("Checking if {} is an Arduino", port_name);

    // Send "PING" command and check for "PONG" response
    let ping_command = "PING\n";
    println!("Checking if {} is an Arduino", port_name);
    if port.write_all(ping_command.as_bytes()).is_err() {
        return false;
    }
    println!("Checking if {} is an Arduino", port_name);
    std::thread::sleep(Duration::from_millis(100));
    println!("Checking if {} is an Arduino", port_name);

    let mut buffer: Vec<u8> = vec![0; 32];
    match port.read(buffer.as_mut_slice()) {
        Ok(_) => {
            println!("Checking if {} is an Arduino", port_name);
            let response = String::from_utf8_lossy(&buffer);
            response.contains("PONG")
        }
        Err(_) => {
            println!("failed");
            false
        }
    }
}

pub fn send_data(port_name: &str, data: &[u8], port_map: &PortMap) -> Result<(), String> {
    let mut map = port_map.lock().unwrap();
    match map.get_mut(port_name) {
        Some(port) => match port.write(data) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        },
        None => Err("Port not connected".to_string()),
    }
}

pub fn disconnect_all_ports(port_map: &PortMap) -> Result<String, String> {
    let mut map = port_map
        .lock()
        .map_err(|e| format!("Lock error: {}", e.to_string()))?;
    map.clear();
    Ok("All serial connections have been disconnected".to_string())
}
