use crate::error::{Error, ErrorKind};

#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct QPIWS {

    pub inverter_fault: bool,
    pub bus_over_fault: bool,
    pub bus_under_fault: bool,
    pub bus_soft_fail_fault: bool,
    pub line_fail_warning: bool,
    pub opv_short_warning: bool,
    pub inverter_voltage_too_low_fault: bool,
    pub inverter_voltage_too_high_fault: bool,
    pub over_temperature: bool,
    pub fan_locked: bool,
    pub battery_voltage_high: bool,
    pub battery_low_alarm_warning: bool,
    pub battery_under_shutdown_warning: bool,
    pub overload: bool,
    pub eeprom_fault_warning: bool,
    pub inverter_over_current_fault: bool,
    pub inverter_soft_fail_fault: bool,
    pub self_test_fail_fault: bool,
    pub op_dc_voltage_over_fault: bool,
    pub bat_open_fault: bool,
    pub current_sensor_fail_fault: bool,
    pub battery_short_fault: bool,
    pub power_limit_warning: bool,
    pub pv_voltage_high_warning: bool,
    pub mppt_overload_fault_1: bool,
    pub mppt_overload_warning_1: bool,
    pub battery_too_low_to_charge: bool,

//There are more, but nore sure what they are yet
}


impl QPIWS {
    pub fn new_from_string(input_data_string: &str) -> Result<Self, Error> {
        //(00000000000000000100000000000000E
        if input_data_string.len() < 1 {
            error!("problems parsing QPIWS data, not enough elements: {}", input_data_string);
            return Err(Error::new(ErrorKind::ParseError, format!("problems parsing QPIWS data, not enough elements: {}", input_data_string)));
        }

        let data_string = &input_data_string[1..];
        let mut parts: Vec<&str> = data_string.split("").collect();
        parts.remove(0);

        if parts.is_empty() {
            error!("Unable to parse QPIWS String: {}", input_data_string);
            return Err(Error::new(ErrorKind::ParseError, format!("Error parsing the QPIWS String: {}", input_data_string)));
        }

        if parts.len() < 30 {
            error!("problems parsing QPIWS data, not enough elements: {}", input_data_string);
            return Err(Error::new(ErrorKind::ParseError, format!("problems parsing QPIWS data, not enough elements: {}", input_data_string)));
        }

        trace!("QPIWS Parting parts: {:?}", parts);

        Ok(QPIWS {
            inverter_fault: match parts[1] { "1" => true, _ => false },
            bus_over_fault: match parts[2] { "1" => true, _ => false },
            bus_under_fault: match parts[3] { "1" => true, _ => false },
            bus_soft_fail_fault: match parts[4] { "1" => true, _ => false },
            line_fail_warning: match parts[5] { "1" => true, _ => false },
            opv_short_warning: match parts[6] { "1" => true, _ => false },
            inverter_voltage_too_low_fault: match parts[7] { "1" => true, _ => false },
            inverter_voltage_too_high_fault: match parts[8] { "1" => true, _ => false },
            over_temperature: match parts[9] { "1" => true, _ => false },
            fan_locked: match parts[10] { "1" => true, _ => false },
            battery_voltage_high: match parts[11] { "1" => true, _ => false },
            battery_low_alarm_warning: match parts[12] { "1" => true, _ => false },
            battery_under_shutdown_warning: match parts[14] { "1" => true, _ => false },
            overload: match parts[16] { "1" => true, _ => false },
            eeprom_fault_warning: match parts[17] { "1" => true, _ => false },
            inverter_over_current_fault: match parts[18] { "1" => true, _ => false },
            inverter_soft_fail_fault: match parts[19] { "1" => true, _ => false },
            self_test_fail_fault: match parts[20] { "1" => true, _ => false },
            op_dc_voltage_over_fault: match parts[21] { "1" => true, _ => false },
            bat_open_fault: match parts[22] { "1" => true, _ => false },
            current_sensor_fail_fault: match parts[23] { "1" => true, _ => false },
            battery_short_fault: match parts[24] { "1" => true, _ => false },
            power_limit_warning: match parts[25] { "1" => true, _ => false },
            pv_voltage_high_warning: match parts[26] { "1" => true, _ => false },
            mppt_overload_fault_1: match parts[27] { "1" => true, _ => false },
            mppt_overload_warning_1: match parts[28] { "1" => true, _ => false },
            battery_too_low_to_charge: match parts[29] { "1" => true, _ => false },
        })
    }
}




#[cfg(test)]
mod tests {
    use std::sync::{Once};
    use crate::TEST_LOG_FILE_CONFIG;
    use crate::data::types::qpiws::QPIWS;

    static SETUP: Once = Once::new();

    fn setup() {
        SETUP.call_once(|| {
            if let Err(e) = log4rs::init_file(TEST_LOG_FILE_CONFIG, Default::default()) {
                warn!("Test Logger already initialised: {}", e);
            }
        });
    }


    #[test]
    fn test_parser() {
        setup();
        match QPIWS::new_from_string("(00000000000000000100000000000000E") {
            Ok(o) => {
                assert!(true);
            },
            Err(e) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_parser_too_short() {
        setup();
        if let Ok(o) = QPIWS::new_from_string("(000000000000000") {
            assert!(false);
        }
    }
    #[test]
    fn test_parser_empty() {
        setup();
        if let Ok(o) = QPIWS::new_from_string("") {
            assert!(false);
        }
    }
}