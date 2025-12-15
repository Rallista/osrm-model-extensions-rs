use osrm_openapi_models::models::ManeuverType;

use crate::{
    distance::{Distance, Unit},
    osrm::{RouteStepBundle, RouteStepExt},
};

use super::{speed_class::SpeedClass, utilities::speed_at_distance};

/// Step length in miles or kilometers required for certain announcements.
const CONTINUE_MINIMUM_DISTANCE: f64 = 1.0;
const CONTINUE_PCT_SHORT_STEP: f64 = 0.9;
const CONTINUE_PCT: f64 = 0.98;

const PRE_APPROACH_MINIMUM_DISTANCE: f64 = 2.0;
const PRE_APPROACH_LONG_THRESHOLD: f64 = 5.0; // A long step is 5 mi/km +
const PRE_APPROACH_LONG_AT: f64 = 2.0; // Long steps the announcement happens at 2 mi/km
const PRE_APPROACH_SHORT_AT: f64 = 1.0; // Short steps the announcement happens at 1 mi/km

const APPROACH_MINIMUM_DISTANCE: f64 = 1.0;
const APPROACH_DISTANCE_SLOW: f64 = 0.25;
const APPROACH_DISTANCE: f64 = 0.5;

const THEN_MINIMUM_DISTANCE: f64 = 0.1;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnnouncementType {
    Depart,
    Continue,
    PreApproach,
    Approach,
    Maneuver,
    ManeuverAndThen,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnnounceAt {
    /// An announcement that is made immediately at the start
    /// of the route. "Depart"
    Depart(Distance),

    /// Announce continue at a specified distance. This typically happens
    /// right after the start of a long manuever.
    Continue(Distance),

    /// In "2 miles (or kilometers), complete a maneuver"
    PreApproach(Distance),

    /// In "0.5 (or 0.25) miles (or kilometers), complete a maneuver"
    Approach(Distance),

    /// The announcement right before the maneuver is made.
    Maneuver(Distance),

    /// The announcement right before the maneuver is made with an
    /// additional "Then" instruction for a very short next step.
    ManeuverAndThen(Distance),
}

pub struct VoiceAnnouncements {
    current: RouteStepBundle,
    next: Option<RouteStepBundle>,
    metric: bool,
    polyline_precision: u32,
}

impl VoiceAnnouncements {
    pub(crate) fn new(
        current: RouteStepBundle,
        next: Option<RouteStepBundle>,
        metric: bool,
        polyline_precision: u32,
    ) -> Self {
        VoiceAnnouncements {
            current,
            next,
            metric,
            polyline_precision,
        }
    }

    /// Build the voice annoucenements for a step.
    pub fn build(&self) -> Vec<AnnounceAt> {
        let announcements = vec![
            self.create_announcement(AnnouncementType::Depart),
            self.create_announcement(AnnouncementType::Continue),
            self.create_announcement(AnnouncementType::PreApproach),
            self.create_announcement(AnnouncementType::Approach),
            self.create_announcement(AnnouncementType::Maneuver),
            self.create_announcement(AnnouncementType::ManeuverAndThen),
        ];
        announcements
            .into_iter()
            .flatten()
            .filter(|a| self.should_announce(*a))
            .collect()
    }

    /// Calculates the distance at which a specific announcement should be made.
    /// This does not consider whether the announcement should be made at all.
    fn create_announcement(&self, announcement: AnnouncementType) -> Option<AnnounceAt> {
        let step_distance = Distance::from_meters(self.current.step.distance.unwrap_or(0.0));

        match announcement {
            // Depart is a notice at the beginning of the route.
            // It occurs immediately for a depart maneuver.
            AnnouncementType::Depart => Some(AnnounceAt::Depart(
                self.pct_of_distance(step_distance, CONTINUE_PCT),
            )),

            // Continue is a notice at the beginning of a longer step.
            AnnouncementType::Continue => {
                if step_distance < self.distance(CONTINUE_MINIMUM_DISTANCE) {
                    None
                } else {
                    let ideal_distance = self.pct_of_distance(step_distance, CONTINUE_PCT);
                    // If the step is short and slow, we want to announce a bit earlier.
                    if step_distance < self.distance(0.5)
                        && self.speed_class(ideal_distance) == Some(SpeedClass::Slow)
                    {
                        Some(AnnounceAt::Continue(
                            self.pct_of_distance(step_distance, CONTINUE_PCT_SHORT_STEP),
                        ))
                    } else {
                        Some(AnnounceAt::Continue(ideal_distance))
                    }
                }
            }

            // PreApproach is a fixed notice 1 to 2 km/mi before the maneuver for longer steps.
            // This is used as an attention wake up after a long stretch of road with no maneuvers.
            AnnouncementType::PreApproach => match step_distance {
                d if d < self.distance(PRE_APPROACH_MINIMUM_DISTANCE) => None,
                d if d > self.distance(PRE_APPROACH_LONG_THRESHOLD) => {
                    Some(AnnounceAt::PreApproach(self.distance(PRE_APPROACH_LONG_AT)))
                }
                _ => Some(AnnounceAt::PreApproach(
                    self.distance(PRE_APPROACH_SHORT_AT),
                )),
            },

            // Approach is an essential notice before the maneuver on larger higher speed roads.
            // This is particularly useful at focusing the user's attention for an upcoming exit,
            // off ramp, or fork.
            AnnouncementType::Approach => {
                if step_distance < self.distance(APPROACH_MINIMUM_DISTANCE) {
                    None
                } else {
                    match self.speed_class(self.pct_of_distance(step_distance, 0.95)) {
                        Some(SpeedClass::Slow) => {
                            Some(AnnounceAt::Approach(self.distance(APPROACH_DISTANCE_SLOW)))
                        }
                        _ => Some(AnnounceAt::Approach(self.distance(APPROACH_DISTANCE))),
                    }
                }
            }

            // The maneuver instruction occurs right as the user approaches the maneuver.
            // The trade off is between the user's speed and the distance to the maneuver.
            // This seems to work well in practice, but is occasionally a bit tight for
            // things like forks and exits where you missed an Approach announcement.
            AnnouncementType::Maneuver => Some(AnnounceAt::Maneuver(
                self.get_maneuver_distance(step_distance),
            )),

            // Then only occurs if the next step is very short.
            AnnouncementType::ManeuverAndThen => Some(AnnounceAt::ManeuverAndThen(
                self.get_maneuver_distance(step_distance),
            )),
        }
    }

    // MARK: Helpers for creating announcements

    /// Determines whether a given announcement should be made based on the step and the next step.
    /// Any step can say every announcement. This just allows us to filter only those that are
    /// needed for the current step.
    fn should_announce(&self, announce_at: AnnounceAt) -> bool {
        let make_distance = if self.metric {
            Distance::from_kilometers
        } else {
            Distance::from_miles
        };
        let length = Distance::from_meters(self.current.step.distance.unwrap_or(0.0));
        let next_length = self
            .next
            .as_ref()
            .and_then(|n| n.step.distance.map(Distance::from_meters));

        let is_depart = matches!(
            self.current.step.maneuver.as_ref().and_then(|m| m.r#type),
            Some(ManeuverType::Depart)
        );

        let needs_then =
            next_length.is_some_and(|length| length < make_distance(THEN_MINIMUM_DISTANCE));

        match announce_at {
            AnnounceAt::Depart(..) => is_depart,
            AnnounceAt::Continue(..) => length >= make_distance(CONTINUE_MINIMUM_DISTANCE),
            AnnounceAt::PreApproach(..) => length >= make_distance(PRE_APPROACH_MINIMUM_DISTANCE),
            AnnounceAt::Approach(..) => length >= make_distance(APPROACH_MINIMUM_DISTANCE),
            AnnounceAt::Maneuver(..) => !needs_then,
            AnnounceAt::ManeuverAndThen(..) => needs_then,
        }
    }

    fn distance(&self, constant_dist: f64) -> Distance {
        if self.metric {
            Distance::from_kilometers(constant_dist)
        } else {
            Distance::from_miles(constant_dist)
        }
    }

    fn pct_of_distance(&self, distance: Distance, pct: f64) -> Distance {
        let new_distance = distance.value() * pct;
        Distance::new(new_distance, distance.unit())
    }

    fn speed_class(&self, distance: Distance) -> Option<SpeedClass> {
        // This method is not ideal, but it can give us some indication of how suddenly the user will
        // approach the maneuver. We can absolutely replace this with a more accurate method in the
        // future.
        let meters = distance.to(Unit::Meters).value();

        self.current
            .step
            .geometry_string()
            .ok()
            .flatten()
            .and_then(|geometry| {
                self.current.annotation.as_ref().map(|annotations| {
                    let mps = speed_at_distance(
                        geometry,
                        annotations.as_ref().clone(),
                        meters,
                        self.polyline_precision,
                    );
                    SpeedClass::from_meters_per_second(mps)
                })
            })
            .flatten()
    }

    fn get_maneuver_distance(&self, step_distance: Distance) -> Distance {
        match self.speed_class(self.pct_of_distance(step_distance, 0.95)) {
            Some(SpeedClass::Fast) => Distance::from_meters(150.0).min(step_distance),
            Some(SpeedClass::Medium) => Distance::from_meters(100.0).min(step_distance),
            _ => Distance::from_meters(70.0).min(step_distance),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{testing::load_route_steps, POLYLINE_PRECISION};

    use super::*;
    use insta::assert_debug_snapshot;

    fn build_announcements(
        current: RouteStepBundle,
        next: Option<RouteStepBundle>,
    ) -> Vec<AnnounceAt> {
        let voice_announcements = VoiceAnnouncements {
            current,
            next,
            metric: true,
            polyline_precision: 5,
        };

        voice_announcements.build()
    }

    #[test]
    fn test_depart_announcement() {
        let (current, next, _) =
            load_route_steps("../fixtures/valhalla-short.json", 0, 0, POLYLINE_PRECISION);
        let announcements = build_announcements(current, next);
        assert_debug_snapshot!(announcements);
    }

    #[test]
    fn test_basic_step() {
        let (current, next, _) =
            load_route_steps("../fixtures/valhalla-short.json", 0, 1, POLYLINE_PRECISION);
        let announcements = build_announcements(current, next);
        assert_debug_snapshot!(announcements);
    }

    #[test]
    fn test_long_step() {
        let (current, next, _) =
            load_route_steps("../fixtures/valhalla-short.json", 0, 2, POLYLINE_PRECISION);
        let announcements = build_announcements(current, next);
        assert_debug_snapshot!(announcements);
    }

    #[test]
    fn test_and_then_step() {
        let (current, next, _) =
            load_route_steps("../fixtures/valhalla-alt.json", 0, 3, POLYLINE_PRECISION);
        let announcements = build_announcements(current, next);
        assert_debug_snapshot!(announcements);
    }
}
