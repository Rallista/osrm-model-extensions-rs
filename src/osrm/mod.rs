use osrm_openapi_models::models::{Annotation, RouteStep};

pub mod extensions;
pub mod utilities;

pub use extensions::{RouteStepExt, StepManeuverExt};

#[derive(Clone, Default, Debug, PartialEq)]
pub struct RouteStepBundle {
    pub step: RouteStep,
    pub annotation: Option<Box<Annotation>>,
    pub start_index: usize,
    pub end_index: usize,
}
