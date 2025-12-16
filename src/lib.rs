pub mod distance;
pub mod geo;
pub mod instructions;
pub mod osrm;
pub mod testing;

rust_i18n::i18n!("locales");

#[allow(dead_code)] // Used in tests.
const POLYLINE_PRECISION: u32 = 6;
