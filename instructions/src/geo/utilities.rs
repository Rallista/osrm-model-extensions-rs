use geo::{Coord, Distance, Haversine};
use polyline::decode_polyline;

pub fn get_coordinates_from_geometry(
    geometry: &str,
    polyline_precision: u32,
) -> Option<Vec<Coord>> {
    decode_polyline(geometry, polyline_precision)
        .ok()?
        .into_iter()
        .collect::<Vec<_>>()
        .into()
}

pub fn get_coordinate_index(
    geometry: &str,
    polyline_precision: u32,
    distance: f64,
) -> Option<usize> {
    let line_string = match decode_polyline(geometry, polyline_precision) {
        Ok(coords) => coords,
        Err(_) => return None,
    };

    let points: Vec<_> = line_string.points().collect();

    if points.len() < 2 {
        return None;
    }

    // Return first point for distance 0
    if distance == 0.0 {
        return Some(0);
    }

    let mut accumulated_distance = 0.0;

    // Iterate through points calculating cumulative distance
    for i in 1..points.len() {
        let current_point = &points[i];
        let prev_point = &points[i - 1];
        let segment_distance = Haversine::distance(*prev_point, *current_point);

        let next_accumulated = accumulated_distance + segment_distance;

        // Find midpoint between accumulated and next_accumulated
        let segment_midpoint = accumulated_distance + (segment_distance / 2.0);

        // If target distance is closer to previous point
        if distance < segment_midpoint {
            return Some(i - 1);
        }
        // If target distance is closer to current point
        else if distance <= next_accumulated {
            return Some(i);
        }

        accumulated_distance = next_accumulated;
    }

    // Return Last if distance is beyond total length
    Some(points.len() - 1)
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;

    use super::*;

    #[test]
    fn test_coordinates_from_geometry() {
        let geometry = "epxkF|`miVfAfA~@~@";
        let coords = get_coordinates_from_geometry(geometry, 5);
        assert_snapshot!(format!("{:?}", coords));
    }

    #[test]
    fn test_get_coordinate_index() {
        // A simple straight line with 3 points, each ~50m apart
        let geometry = "epxkF|`miVfAfA~@~@"; // Example encoded polyline

        // Distances -
        // Accumulated: 50.75198497595465
        // Accumulated: 95.86494183726555

        // Test distances
        assert_eq!(
            get_coordinate_index(geometry, 5, 0.0),
            Some(0),
            "Should find first point at 0m"
        );
        assert_eq!(
            get_coordinate_index(geometry, 5, 25.0),
            Some(0),
            "Should round to first point at 25"
        );
        assert_eq!(
            get_coordinate_index(geometry, 5, 25.4),
            Some(1),
            "Should find first point at 25.4m"
        );
        assert_eq!(
            get_coordinate_index(geometry, 5, 73.0),
            Some(1),
            "Should round first point at 73m"
        );
        assert_eq!(
            get_coordinate_index(geometry, 5, 74.0),
            Some(2),
            "Should round second point at 74m"
        );
        assert_eq!(
            get_coordinate_index(geometry, 5, 95.0),
            Some(2),
            "Should find second point at 95m"
        );
        assert_eq!(
            get_coordinate_index(geometry, 5, 100.0),
            Some(2),
            "Should return last for distance beyond geometry"
        );
    }

    #[test]
    fn test_get_coordinate_index_invalid_input() {
        // Test invalid geometry string
        assert_eq!(
            get_coordinate_index("invalid", 5, 100.0),
            None,
            "Should return None for invalid geometry"
        );

        // Test empty geometry
        assert_eq!(
            get_coordinate_index("", 5, 100.0),
            None,
            "Should return None for empty geometry"
        );
    }

    #[test]
    fn test_get_coordinate_index_single_point() {
        // Test geometry with single point
        let single_point = "_p~iF~ps|U"; // Example single point
        assert_eq!(
            get_coordinate_index(single_point, 5, 100.0),
            None,
            "Should return None for single point geometry"
        );
    }
}
