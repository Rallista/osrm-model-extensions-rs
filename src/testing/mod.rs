pub mod fixtures;

use crate::osrm::{RouteStepBundle, utilities::get_step_bundles};
pub use fixtures::load_route;

pub fn load_route_steps(
    file_path: &str,
    route_index: usize,
    step_index: usize,
    polyline_precision: u32,
) -> (
    RouteStepBundle,
    Option<RouteStepBundle>,
    Option<RouteStepBundle>,
) {
    let route = load_route(file_path, route_index);
    let steps = get_step_bundles(&route, polyline_precision)
        .expect("Failed to get step bundles")
        .into_iter()
        .collect::<Vec<RouteStepBundle>>();

    // Get the current step and optionally the next step
    let current_step = steps[step_index].clone();
    let next_step = steps.get(step_index + 1).cloned();
    let step_after_next = steps.get(step_index + 2).cloned();

    (current_step, next_step, step_after_next)
}
