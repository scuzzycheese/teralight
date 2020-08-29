use crate::error::Error;

#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct QPIGS {
    ac_input_voltage: f32,
    ac_input_frequency: f32,
    ac_output_voltage: f32,
    ac_output_frequency: f32,
    ac_output_va: usize,
    ac_output_watts: usize,
    load_percent: usize,
    bus_voltage: usize,
    battery_voltage: f32,
    battery_charging_current: usize,
    battery_capacity_percent: usize,
    inverter_heatsink_temp: usize,
    pv_input_current: usize,
    pv_input_voltage: f32,
    battery_voltage_from_scc: f32,
    battery_discharge_current: usize,
    //There are more, but nore sure what they are yet
}


impl QPIGS {
    pub fn new_from_string(input_data_string: &str) -> Result<Self, Error> {
        let data_string = &input_data_string[1..];
        let parts: Vec<&str> = data_string.split(" ").collect();

        trace!("QPIGS Parting parts: {:?}", parts);

        Ok(QPIGS {
            ac_input_voltage: parts[0].parse::<f32>()?,
            ac_input_frequency: parts[1].parse::<f32>()?,
            ac_output_voltage: parts[2].parse::<f32>()?,
            ac_output_frequency: parts[3].parse::<f32>()?,
            ac_output_va: parts[4].parse::<usize>()?,
            ac_output_watts: parts[5].parse::<usize>()?,
            load_percent: parts[6].parse::<usize>()?,
            bus_voltage: parts[7].parse::<usize>()?,
            battery_voltage: parts[8].parse::<f32>()?,
            battery_charging_current: parts[9].parse::<usize>()?,
            battery_capacity_percent: parts[10].parse::<usize>()?,
            inverter_heatsink_temp: parts[11].parse::<usize>()?,
            pv_input_current: parts[12].parse::<usize>()?,
            pv_input_voltage: parts[13].parse::<f32>()?,
            battery_voltage_from_scc: parts[14].parse::<f32>()?,
            battery_discharge_current: parts[15].parse::<usize>()?
        })
    }
}