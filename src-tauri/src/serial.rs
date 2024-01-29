use serialport::{available_ports, SerialPort};
use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

pub type PortMap = Arc<Mutex<HashMap<String, Box<dyn SerialPort>>>>;

pub fn list_ports() -> Result<Vec<String>, String> {
    match available_ports() {
        Ok(ports) => Ok(ports.into_iter().map(|p| p.port_name).collect()),
        Err(e) => Err(e.to_string()),
    }
}

pub async fn trigger_serial_alarm(port_name: &str, duration: u64) -> Result<(), String> {
    match send_data(port_name, "on\n".to_string()) {
        Ok(_) => {}
        Err(_e) => {}
    }
    thread::sleep(Duration::from_secs(duration));
    match send_data(port_name, "off\n".to_string()) {
        Ok(_) => Ok(()),
        Err(e) => {
            // Collect errors to report them, or handle them as needed
            Err(format!("Error triggering alarm: {}", e))
        }
    }
}

pub fn connect_port(port_name: &str) -> Result<String, String> {
    if is_arduino(port_name) {
        let _serial_port = serialport::new(port_name, 9600)
            .open()
            .map_err(|e| e.to_string())?;

        Ok(format!("Connected to Arduino on {}", port_name))
    } else {
        Err("The device is not a valid Arduino".to_string())
    }
}

pub fn send_data(port_name: &str, data: String) -> Result<Box<dyn SerialPort>, String> {
    let port = match serialport::new(port_name, 9600)
        .timeout(Duration::from_millis(100))
        .open()
    {
        Ok(p) => p,
        Err(e) => Err(format!("Failed to open {}: {}", port_name, e.to_string()))?,
    };

    let (sender, receiver) = mpsc::channel();

    let mut port_clone = port.try_clone().expect("Failed to clone port");

    println!("Sent data: {}", data.clone());

    thread::spawn(move || {
        let write_result = port_clone.write_all(data.as_bytes());
        write_result.unwrap_or_else(|e| eprintln!("Write failed: {:?}", e));
        sender.send(()).unwrap();
    });

    match receiver.recv_timeout(Duration::from_millis(100)) {
        Ok(_) => Ok(port),
        Err(_) => Err("Write operation timed out".to_string()),
    }
}

pub fn disconnect_all_ports(port_map: &PortMap) -> Result<String, String> {
    let mut map = port_map
        .lock()
        .map_err(|e| format!("Lock error: {}", e.to_string()))?;
    map.clear();
    Ok("All serial connections have been disconnected".to_string())
}

fn is_arduino(port_name: &str) -> bool {
    let mut port = match send_data(port_name, "PING\n".to_string()) {
        Ok(p) => p,
        Err(_) => {
            return false;
        }
    };

    let mut serial_buf: Vec<u8> = vec![0; 32]; // Adjust buffer size as needed
    match port.read(serial_buf.as_mut_slice()) {
        Ok(bytes) => {
            if bytes > 0 {
                let received_data = String::from_utf8_lossy(&serial_buf);
                if received_data.contains("PONG\r\n") {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        }
        Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => false,
        Err(_e) => false,
    }
}
