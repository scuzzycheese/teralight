#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate log;
extern crate log4rs;

#[macro_use] extern crate rocket;
#[macro_use]
extern crate lazy_static;

use std::time::Duration;
use serialport::{SerialPortSettings, DataBits, FlowControl, Parity, StopBits, SerialPort};
use std::ops::Add;
use crc16::*;
use std::io::Read;
use crate::error::*;
use crate::data::holder::Holder;
use rocket::response::content::Json;
use std::sync::{Arc, Mutex};
use crate::data::types::qpigs::QPIGS;



#[cfg(target_os="macos")]
const TTY_DEV: &str = "/dev/tty.usbmodem141101";
#[cfg(target_os="linux")]
const TTY_DEV: &str = "/dev/ttyACM0";


#[allow(dead_code)]
static TEST_LOG_FILE_CONFIG: &'static str = "log4rs_tests.yaml";
static LOG_FILE_CONFIG: &'static str = "log4rs.yaml";


pub mod error;
pub mod data;



// #[get("/")]
// fn index() -> &'static str {
//     "Hello, world!"
// }
//
//
// #[get("/status")]
// fn status() -> Json<QPIGS> {
//
// }

fn main() {
    log4rs::init_file(LOG_FILE_CONFIG, Default::default()).unwrap();

    let mut port = match serialport::open_with_settings(TTY_DEV, &SerialPortSettings {
        baud_rate: 2400,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::Hardware,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: Duration::from_millis(1000)
    }) {
        Ok(p) => p,
        Err(e) => {
            error!("Error, unable to open the serial port: {}", e);
            panic!("Error, unable to open the serial port: {}", e);
        }
    };



    // rocket::ignite().mount("/", routes![index]).launch();


    //Build the command to send to the inverter
    let command = build_command("QPIGS");

    //Write that command to the inverter
    write_command(&mut port, command);

    //Read the result
    let response = read_result(&mut port);

    let qpigs = QPIGS::new_from_string(&String::from_utf8_lossy(&response));
    info!("QPIGS: {:?}", qpigs);

    println!("Response: {}", String::from_utf8_lossy(&response));


}

fn write_command(port: &mut Box<dyn SerialPort>, command: Vec<u8>) -> Result<usize, Error> {
    let bytes_written = port.write(command.as_slice())?;
    debug!("Bytes writen to serial port: {}", bytes_written);
    Ok(bytes_written)
}

//TODO: But a CRC check on the response
fn read_result(port: &mut Box<dyn SerialPort>) -> [u8; 1000] {

    let mut buf: [u8; 1000] = [0; 1000];
    let mut buf_slice = &mut buf[0..];

    let mut counter: usize = 0;

    while buf_slice[0] as char != '\r' {
        buf_slice = &mut buf[counter..];
        let bytes_read = match port.read(buf_slice) {
            Ok(br) => { counter += br; br },
            Err(e) => {
                panic!("Failed to read bytes: {}", e);
            }
        };
        trace!("Bytes read: {}", bytes_read);
        trace!("Byte read: {}", buf_slice[0] as char);
    }

    buf

}


fn build_command(command: &str) -> Vec<u8> {
    let mut command: Vec<u8> = String::from(command).into_bytes();
    let crc_16 = State::<XMODEM>::calculate(command.as_slice());

    let crc = unsafe {
        std::mem::transmute::<u16, [u8; 2]>(crc_16)
    };

    command.push(crc[1]);
    command.push(crc[0]);
    command.push(0x0d as u8);

    println!("commnand: {:?}", command);

    command
}
