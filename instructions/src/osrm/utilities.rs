

use osrm_openapi_models::models::{Annotation, Route};

use crate::geo::utilities::get_coordinates_from_geometry;

use super::{RouteStepBundle, RouteStepExt};

pub(crate) fn get_step_bundles(
    route: &Route,
    polyline_precision: u32,
) -> Option<Vec<RouteStepBundle>> {
    let annotations = route
        .clone()
        .legs?
        .iter()
        .flat_map(|l| l.annotation.clone())
        .next();

    let mut current_index = 0;

    let steps = route
        .legs
        .as_ref()?
        .iter()
        .flat_map(|l| l.steps.clone())
        .flatten()
        .map(|step| {
            let coord_len = step
                .geometry_string()
                .ok()
                .flatten()
                .and_then(|g| get_coordinates_from_geometry(&g, polyline_precision))
                .map(|c| c.len())
                .unwrap_or(0);
            let annotation_len = coord_len - 1; // annotations are between coordinates

            let start_index = current_index;
            let end_index = current_index + annotation_len - 1;
            current_index = end_index + 1;

            let annotation_slice =
                get_annotation_slice(annotations.clone(), start_index, end_index);

            RouteStepBundle {
                step,
                annotation: annotation_slice,
                start_index,
                end_index,
            }
        })
        .collect();
    Some(steps)
}

pub(crate) fn get_annotation_slice(
    annotations: Option<Box<Annotation>>,
    start_index: usize,
    end_index: usize,
) -> Option<Box<Annotation>> {
    if start_index >= end_index {
        return None;
    }

    annotations.map(|ann| {
        Box::new(Annotation {
            distance: ann.distance.map(|v| v[start_index..=end_index].to_vec()),
            duration: ann.duration.map(|v| v[start_index..=end_index].to_vec()),
            datasources: ann.datasources.map(|v| v[start_index..=end_index].to_vec()),
            nodes: ann.nodes.map(|v| v[start_index..=end_index].to_vec()),
            weight: ann.weight.map(|v| v[start_index..=end_index].to_vec()),
            speed: ann.speed.map(|v| v[start_index..=end_index].to_vec()),
            maxspeed: ann.maxspeed.map(|v| v[start_index..=end_index].to_vec()),
            metadata: ann.metadata.clone(),
        })
    })
}

#[cfg(test)]
mod tests {
    use crate::POLYLINE_PRECISION;

    use super::*;
    use insta::assert_debug_snapshot;
    use crate::testing::load_route;

    #[test]
    fn test_get_step_bundle_index_ranges() {
        let route = load_route("../fixtures/valhalla-short.json", 0);
        let step_bundles = get_step_bundles(&route, POLYLINE_PRECISION).unwrap();

        let index_ranges = step_bundles
            .iter()
            .map(|sb| (sb.start_index, sb.end_index))
            .collect::<Vec<(usize, usize)>>();

        assert_debug_snapshot!(index_ranges);
    }

    #[test]
    fn test_get_step_bundle_annotations() {
        let route = load_route("../fixtures/valhalla-short.json", 0);
        let step_bundles = get_step_bundles(&route, POLYLINE_PRECISION).unwrap();

        let annotations = step_bundles
            .iter()
            .map(|sb| sb.annotation.clone())
            .collect::<Vec<_>>();

        assert_debug_snapshot!(annotations);
    }
}
