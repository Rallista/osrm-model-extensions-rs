use rust_i18n::t;

use osrm_openapi_models::models::VoiceInstruction;

use crate::{
    distance::{Distance, Unit, spoken_distance::SpokenDistance},
    instructions::voice_announcements::VoiceAnnouncements,
    osrm::{RouteStepBundle, StepManeuverExt},
};

use super::{utilities::step_maneuver_name, voice_announcements::AnnounceAt};

pub struct VoiceInstructionFactory {
    current: RouteStepBundle,
    next: RouteStepBundle,
    step_after_next: Option<RouteStepBundle>,
    metric: bool,
    announcements: VoiceAnnouncements,
}

impl VoiceInstructionFactory {
    pub fn new(
        current: RouteStepBundle,
        next: RouteStepBundle,
        step_after_next: Option<RouteStepBundle>,
        metric: bool,
        polyline_precision: u32,
    ) -> Self {
        VoiceInstructionFactory {
            current: current.clone(),
            next: next.clone(),
            step_after_next: step_after_next.clone(),
            metric,
            // TODO: This may need to consider step_after_next
            announcements: VoiceAnnouncements::new(current, Some(next), metric, polyline_precision),
        }
    }

    pub fn build(&self) -> Vec<VoiceInstruction> {
        self.announcements
            .build()
            .iter()
            .flat_map(|announcement| self.generate(*announcement))
            .collect()
    }

    fn generate(&self, announce_at: AnnounceAt) -> Option<VoiceInstruction> {
        self.announcement(announce_at)
            .map(|announcement| match announce_at {
                AnnounceAt::Depart(d) => VoiceInstruction {
                    distance_along_geometry: self.meters(d),
                    announcement,
                    ssml_announcement: self.ssml_announcement(),
                },
                AnnounceAt::Continue(d) => VoiceInstruction {
                    distance_along_geometry: self.meters(d),
                    announcement,
                    ssml_announcement: self.ssml_announcement(),
                },
                AnnounceAt::PreApproach(d) => VoiceInstruction {
                    distance_along_geometry: self.meters(d),
                    announcement,
                    ssml_announcement: self.ssml_announcement(),
                },
                AnnounceAt::Approach(d) => VoiceInstruction {
                    distance_along_geometry: self.meters(d),
                    announcement,
                    ssml_announcement: self.ssml_announcement(),
                },
                AnnounceAt::Maneuver(d) => VoiceInstruction {
                    distance_along_geometry: self.meters(d),
                    announcement,
                    ssml_announcement: self.ssml_announcement(),
                },
                AnnounceAt::ManeuverAndThen(d) => VoiceInstruction {
                    distance_along_geometry: self.meters(d),
                    announcement,
                    ssml_announcement: self.ssml_announcement(),
                },
            })
    }

    fn announcement(&self, announce_at: AnnounceAt) -> Option<String> {
        let current_instruction = self
            .current
            .step
            .maneuver
            .as_ref()
            .and_then(|m| m.instruction_string().ok().flatten());

        self.next
            .step
            .maneuver
            .as_ref()
            .and_then(|m| m.instruction_string().ok().flatten())
            // Join the next step's instruction with the current step's street name (for continue).
            .map(|instruction| (instruction, step_maneuver_name(self.current.step.clone())))
            .map(|(instruction, name)| match announce_at {
                AnnounceAt::Depart(..) => current_instruction.unwrap_or(t!("depart").to_string()),
                AnnounceAt::Continue(d) => t!(
                    "Continue on %{name} for %{distance}",
                    name = name,
                    distance = self.spoken_distance(d)
                )
                .to_string(),
                AnnounceAt::PreApproach(d) => t!(
                    "In %{distance}, %{instruction}",
                    distance = self.spoken_distance(d),
                    instruction = instruction.to_lowercase()
                )
                .to_string(),
                AnnounceAt::Approach(d) => t!(
                    "In %{distance}, %{instruction}",
                    distance = self.spoken_distance(d),
                    instruction = instruction.to_lowercase()
                )
                .to_string(),
                AnnounceAt::Maneuver(..) => instruction,
                AnnounceAt::ManeuverAndThen(..) => self
                    .step_after_next
                    .as_ref()
                    .and_then(|b| b.step.maneuver.as_ref())
                    .and_then(|m| m.instruction_string().ok().flatten())
                    .map(|n| {
                        t!(
                            "%{instruction} Then %{next}",
                            instruction = instruction,
                            next = n
                        )
                        .to_string()
                    })
                    .unwrap_or_else(|| instruction),
            })
    }

    fn ssml_announcement(&self) -> Option<String> {
        // TODO: SSML Integration?
        None
    }

    fn meters(&self, distance: Distance) -> f64 {
        distance.to(Unit::Meters).value()
    }

    fn spoken_distance(&self, distance: Distance) -> String {
        let converted = if self.metric {
            distance.to(Unit::Kilometers)
        } else {
            distance.to(Unit::Miles)
        };
        SpokenDistance::from_distance(converted).spoken()
    }
}

#[cfg(test)]
mod tests {
    use crate::{POLYLINE_PRECISION, testing::load_route_steps};

    use super::*;
    use insta::assert_json_snapshot;

    fn build_instructions(
        current: RouteStepBundle,
        next: RouteStepBundle,
        step_after_next: Option<RouteStepBundle>,
        metric: bool,
    ) -> Vec<VoiceInstruction> {
        let voice_instructions = VoiceInstructionFactory::new(
            current,
            next,
            step_after_next,
            metric,
            POLYLINE_PRECISION,
        );
        voice_instructions.build()
    }

    #[test]
    fn test_depart() {
        let (current, next, after) =
            load_route_steps("./fixtures/valhalla-short.json", 0, 0, POLYLINE_PRECISION);
        let instruction = build_instructions(current, next.unwrap(), after, true);
        assert_json_snapshot!(instruction);
    }

    #[test]
    fn test_basic_step() {
        let (current, next, after) =
            load_route_steps("./fixtures/valhalla-short.json", 0, 1, POLYLINE_PRECISION);
        let instruction = build_instructions(current, next.unwrap(), after, true);
        assert_json_snapshot!(instruction);
    }

    #[test]
    fn test_long_step_metric() {
        let (current, next, after) =
            load_route_steps("./fixtures/valhalla-short.json", 0, 2, POLYLINE_PRECISION);
        let metric = build_instructions(current, next.unwrap(), after, true);
        assert_json_snapshot!(metric);
    }

    #[test]
    fn test_long_step_imperial() {
        let (current, next, after) =
            load_route_steps("./fixtures/valhalla-short.json", 0, 2, POLYLINE_PRECISION);
        let imperial = build_instructions(current, next.unwrap(), after, false);
        assert_json_snapshot!(imperial);
    }

    #[test]
    fn test_and_then_step_metric() {
        let (current, next, after) =
            load_route_steps("./fixtures/valhalla-alt.json", 0, 3, POLYLINE_PRECISION);
        let metric = build_instructions(current, next.unwrap(), after, true);
        assert_json_snapshot!(metric);
    }

    #[test]
    fn test_and_then_step_imperial() {
        let (current, next, after) =
            load_route_steps("./fixtures/valhalla-alt.json", 0, 3, POLYLINE_PRECISION);
        let imperial = build_instructions(current, next.unwrap(), after, false);
        assert_json_snapshot!(imperial);
    }
}
