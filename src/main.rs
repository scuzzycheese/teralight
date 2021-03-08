#![feature(proc_macro_hygiene, decl_macro)]
#![feature(with_options)]

#[macro_use] extern crate log;
extern crate log4rs;

#[macro_use] extern crate lazy_static;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket;

use std::time::Duration;
use serialport::{SerialPortSettings, DataBits, FlowControl, Parity, StopBits, SerialPort};
use crc16::State as CRCState;
use std::io::{Read, Write};
use crate::error::*;
use crate::data::holder::Holder;
use crate::data::types::qpigs::QPIGS;
use std::thread;

use rocket_contrib::json::{Json};
use rocket_contrib::serve::StaticFiles;
use crate::data::types::qpiws::QPIWS;
use std::fs::{File, OpenOptions};


#[cfg(target_os="macos")]
const TTY_DEV: &str = "/dev/tty.usbmodem141101";
#[cfg(target_os="macos")]
const RAW_HID: &str = "/dev/hidraw0";
#[cfg(target_os="linux")]
const TTY_DEV: &str = "/dev/ttyACM0";
#[cfg(target_os="linux")]
const RAW_HID: &str = "/dev/hidraw0";





#[allow(dead_code)]
static TEST_LOG_FILE_CONFIG: &'static str = "log4rs_tests.yaml";
static LOG_FILE_CONFIG: &'static str = "log4rs.yaml";


pub mod error;
pub mod data;



#[get("/status", rank = 20)]
fn status() -> Option<Json<QPIGS>> {
    match Holder::get_qpigs() {
        Some(qpigs) => Some(Json(qpigs)),
        None => None
    }
}

#[get("/errors_and_warnings", rank = 21)]
fn errors_and_warnings() -> Option<Json<QPIWS>> {
    match Holder::get_qpiws() {
        Some(qpiws) => Some(Json(qpiws)),
        None => None
    }
}


fn main() {
    log4rs::init_file(LOG_FILE_CONFIG, Default::default()).unwrap();

    //Start with a blank version of the structure
    Holder::set_qpigs(QPIGS {
        ac_input_voltage: 0.0,
        ac_input_frequency: 0.0,
        ac_output_voltage: 0.0,
        ac_output_frequency: 0.0,
        ac_output_va: 0,
        ac_output_watts: 0,
        load_percent: 0,
        bus_voltage: 0,
        battery_voltage: 0.0,
        battery_charging_current: 0,
        battery_capacity_percent: 0,
        inverter_heatsink_temp: 0,
        pv_input_current: 0,
        pv_input_voltage: 0.0,
        battery_voltage_from_scc: 0.0,
        battery_discharge_current: 0,
        pv_charging_power: 0
    });


    //TODO: Maybe each of these needs a ::default method
    Holder::set_qpiws(QPIWS {
        inverter_fault: false,
        bus_over_fault: false,
        bus_under_fault: false,
        bus_soft_fail_fault: false,
        line_fail_warning: false,
        opv_short_warning: false,
        inverter_voltage_too_low_fault: false,
        inverter_voltage_too_high_fault: false,
        over_temperature: false,
        fan_locked: false,
        battery_voltage_high: false,
        battery_low_alarm_warning: false,
        battery_under_shutdown_warning: false,
        overload: false,
        eeprom_fault_warning: false,
        inverter_over_current_fault: false,
        inverter_soft_fail_fault: false,
        self_test_fail_fault: false,
        op_dc_voltage_over_fault: false,
        bat_open_fault: false,
        current_sensor_fail_fault: false,
        battery_short_fault: false,
        power_limit_warning: false,
        pv_voltage_high_warning: false,
        mppt_overload_fault_1: false,
        mppt_overload_warning_1: false,
        battery_too_low_to_charge: false
    });

    thread::spawn(move || {
        loop {
            let command = build_command("QPIGS");
            match fetch_command_data_usb(RAW_HID, command) {
                Ok(data) => {
                    if let Err(e) = update_qpigs(&data) {
                        error!("Something went wrong updating the QPIGS information: {}", e);
                    }
                },
                Err(e) => {
                    error!("Error fetching data from the inverter: {}", e);
                }
            }

            let command = build_command("QPIWS");
            match fetch_command_data_usb(RAW_HID, command) {
                Ok(data) => {
                    if let Err(e) = update_qpiws(&data) {
                        error!("Something went wrong updating the QPIWS information: {}", e);
                    }
                },
                Err(e) => {
                    error!("Error fetching data from the inverter: {}", e);
                }
            }

            thread::sleep(Duration::from_secs(2));
        }
    });

    rocket::ignite()
        .mount("/", StaticFiles::from("/home/pi/web").rank(30))
        .mount("/api", routes![status, errors_and_warnings]).launch();
}



fn update_qpigs(data: &[u8]) -> Result<(), Error> {
    match QPIGS::new_from_string(&String::from_utf8_lossy(data)) {
        Ok(qp) => {
            trace!("QPIGS: {:?}", &qp);
            Holder::set_qpigs(qp);
            Ok(())
        },
        Err(e) => {
            error!("Error marshalling response to structure: {}", e);
            Err(Error::from(e))
        }
    }
}

fn update_qpiws(data: &[u8]) -> Result<(), Error> {
    match QPIWS::new_from_string(&String::from_utf8_lossy(data)) {
        Ok(qw) => {
            trace!("QPIWS: {:?}", &qw);
            Holder::set_qpiws(qw);
            Ok(())
        },
        Err(e) => {
            error!("Error marshalling response to structure: {}", e);
            Err(Error::from(e))
        }
    }
}


// fn setup_port(port_device: &str) -> Option<Box<dyn SerialPort>> {
//     match serialport::open_with_settings(port_device, &SerialPortSettings {
//         baud_rate: 2400,
//         data_bits: DataBits::Eight,
//         flow_control: FlowControl::Hardware,
//         parity: Parity::None,
//         stop_bits: StopBits::One,
//         timeout: Duration::from_millis(1000)
//     }) {
//         Ok(sp) => Some(sp),
//         Err(e) => {
//             error!("Unable to open serial port: {}", e);
//             None
//         }
//     }
// }




fn fetch_command_data_usb(usb_file_name: &str, command: Vec<u8>) -> Result<[u8; 1000], Error> {
    // let mut usb_file = File::open(usb_file_name)?;
    let mut usb_file = File::with_options().write(true).read(true).open(usb_file_name)?;
    write_command_usb(&mut usb_file, command)?;
    read_result_usb(&mut usb_file)
}
fn write_command_usb(usb_file: &mut File, command: Vec<u8>) -> Result<usize, Error> {
    usb_file.write_all(command.as_slice())?;
    debug!("Bytes writen to use device.");
    Ok(command.len())
}


//TODO: But a CRC check on the response
fn read_result_usb(usb_file: &mut File) -> Result<[u8; 1000], Error> {
    let mut buf: [u8; 1000] = [0; 1000];
    let mut buf_slice = &mut buf[0..];

    let mut counter: usize = 0;

    // while buf_slice[0] as char != '\r' {
    while !buf_slice.contains( &('\r' as u8)) {
        buf_slice = &mut buf[counter..];
        let bytes_read = match usb_file.read(buf_slice) {
            Ok(br) => { counter += br; br },
            Err(e) => {
                error!("failed to read bytes from serial port: {}", e);
                return Err(Error::from(e));
            }
        };
        trace!("Bytes read: {}", bytes_read);
        trace!("Byte read: {}", buf_slice[0] as char);
    }
    Ok(buf)
}

// fn write_command_serial(port: &mut Box<dyn SerialPort>, command: Vec<u8>) -> Result<usize, Error> {
//     let bytes_written = port.write(command.as_slice())?;
//     debug!("Bytes writen to serial port: {}", bytes_written);
//     Ok(bytes_written)
// }

//TODO: But a CRC check on the response
// fn read_result_serial(port: &mut Box<dyn SerialPort>) -> Result<[u8; 1000], Error> {
//
//     let mut buf: [u8; 1000] = [0; 1000];
//     let mut buf_slice = &mut buf[0..];
//
//     let mut counter: usize = 0;
//
//     while buf_slice[0] as char != '\r' {
//         buf_slice = &mut buf[counter..];
//         let bytes_read = match port.read(buf_slice) {
//             Ok(br) => { counter += br; br },
//             Err(e) => {
//                 error!("failed to read bytes from serial port: {}", e);
//                 return Err(Error::from(e));
//             }
//         };
//         trace!("Bytes read: {}", bytes_read);
//         trace!("Byte read: {}", buf_slice[0] as char);
//     }
//
//     Ok(buf)
// }


fn build_command(command: &str) -> Vec<u8> {
    let mut command: Vec<u8> = String::from(command).into_bytes();
    let crc_16 = CRCState::<crc16::XMODEM>::calculate(command.as_slice());

    let crc = unsafe {
        std::mem::transmute::<u16, [u8; 2]>(crc_16)
    };

    command.push(crc[1]);
    command.push(crc[0]);
    command.push(0x0d as u8);

    debug!("Command Built: {:?}", command);

    command
}
