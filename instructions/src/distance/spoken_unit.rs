use osrm_extensions_i18n::t;

pub trait SpokenUnit {
    fn get_form(count: f64) -> String;
}

#[derive(Debug, Clone)]
pub struct SpokenMile;

impl SpokenUnit for SpokenMile {
    fn get_form(count: f64) -> String {
        if count <= 1.0 {
            t!("units.singular.mile").to_string()
        } else {
            t!("units.plural.miles").to_string()
        }
    }
}

#[derive(Debug, Clone)]
pub struct SpokenFoot;

impl SpokenUnit for SpokenFoot {
    fn get_form(count: f64) -> String {
        if count == 1.0 {
            t!("units.singular.foot").to_string()
        } else {
            t!("units.plural.feet").to_string()
        }
    }
}

#[derive(Debug, Clone)]
pub struct SpokenKilometer;

impl SpokenUnit for SpokenKilometer {
    fn get_form(count: f64) -> String {
        if count == 1.0 {
            t!("units.singular.kilometer").to_string()
        } else {
            t!("units.plural.kilometers").to_string()
        }
    }
}

#[derive(Debug, Clone)]
pub struct SpokenMeter;

impl SpokenUnit for SpokenMeter {
    fn get_form(count: f64) -> String {
        if count == 1.0 {
            t!("units.singular.meter").to_string()
        } else {
            t!("units.plural.meters").to_string()
        }
    }
}
