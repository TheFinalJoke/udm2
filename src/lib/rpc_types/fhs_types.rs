tonic::include_proto!("fhs_types");

impl RegulatorType {
    pub fn get_possible_values() -> Vec<&'static str> {
        [
            RegulatorType::Pump.as_str_name(),
            RegulatorType::Tap.as_str_name(),
            RegulatorType::Valve.as_str_name(),
        ]
        .to_vec()
    }
}
