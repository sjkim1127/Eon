use std::collections::HashMap;

pub struct ExaltationDetriment {
    pub gate: u8,
    pub line: u8,
    pub exaltation_planet: Option<&'static str>,
    pub detriment_planet: Option<&'static str>,
}

pub fn get_incarnation_cross_name(sun_gate: u8, angle: &str) -> String {
    match (sun_gate, angle) {
        // Example mapping for a few gates. Users can fill the rest later.
        (1, "Right Angle") => "Right Angle Cross of the Sphinx 4".to_string(),
        (1, "Juxtaposition") => "Juxtaposition Cross of Self-Expression".to_string(),
        (1, "Left Angle") => "Left Angle Cross of Defiance 2".to_string(),
        (2, "Right Angle") => "Right Angle Cross of the Sphinx 2".to_string(),
        (2, "Juxtaposition") => "Juxtaposition Cross of Driver".to_string(),
        (2, "Left Angle") => "Left Angle Cross of Defiance 1".to_string(),
        _ => format!("{} Cross of Gate {}", angle, sun_gate),
    }
}

pub fn get_line_fixation(gate: u8, line: u8) -> ExaltationDetriment {
    match (gate, line) {
        (1, 1) => ExaltationDetriment { gate, line, exaltation_planet: Some("Sun"), detriment_planet: Some("Earth") },
        (1, 2) => ExaltationDetriment { gate, line, exaltation_planet: Some("Venus"), detriment_planet: Some("Mars") },
        _ => ExaltationDetriment { gate, line, exaltation_planet: None, detriment_planet: None },
    }
}
