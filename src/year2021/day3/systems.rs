pub enum SystemType {
    Oxygen,
    CO2,
}

pub fn system_idx(system_type: &SystemType) -> usize {
    match system_type {
        SystemType::Oxygen => 0,
        SystemType::CO2 => 1,
    }
}
