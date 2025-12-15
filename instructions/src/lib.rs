osrm_extensions_i18n::init!();

pub mod distance;
pub mod geo;
pub mod instructions;
pub mod osrm;
pub mod testing;

#[allow(dead_code)] // Used in tests.
const POLYLINE_PRECISION: u32 = 6;
