use banner_instruction::BannerInstructionsFactory;
use osrm_openapi_models::models::Route;
use voice_instructions::VoiceInstructionFactory;

use crate::osrm::utilities::get_step_bundles;

pub mod banner_instruction;
pub mod speed_class;
pub mod utilities;
pub mod voice_announcements;
pub mod voice_instructions;

pub struct InstructionsFactory {
    polyline_precision: u32,
    metric: bool,
}

impl InstructionsFactory {
    pub fn new(polyline_precision: u32, metric: bool) -> Self {
        InstructionsFactory {
            polyline_precision,
            metric,
        }
    }

    pub fn apply(&self, mut route: Route) -> Option<Route> {
        let step_bundles = get_step_bundles(&route, self.polyline_precision)?;
        let legs = route.legs.as_mut()?;

        // Process each leg's steps
        for leg in legs.iter_mut() {
            // Access steps through the leg's steps field
            if let Some(ref mut steps) = leg.steps {
                for (index, step) in steps.iter_mut().enumerate() {
                    let current = step_bundles.get(index)?;
                    let next = step_bundles.get(index + 1);
                    let step_after_next = step_bundles.get(index + 2);

                    // Generate banner instructions
                    let banner_factory = BannerInstructionsFactory::new(
                        next.map(|b| b.step.clone()),
                        Some(current.step.clone()),
                    );
                    step.banner_instructions = Some(banner_factory.build());

                    // Generate voice instructions if we have a next step
                    if let Some(next_bundle) = next {
                        let voice_factory = VoiceInstructionFactory::new(
                            current.clone(),
                            next_bundle.clone(),
                            step_after_next.cloned(),
                            self.metric,
                            self.polyline_precision,
                        );
                        step.voice_instructions = Some(voice_factory.build());
                    }
                }
            }
        }

        Some(route)
    }
}

#[cfg(test)]
mod tests {
    use crate::POLYLINE_PRECISION;

    use super::*;
    use insta::assert_json_snapshot;
    use crate::testing::load_route;

    #[test]
    fn test_instructions_factory() {
        let route = load_route("../fixtures/valhalla-short.json", 0);
        let factory = InstructionsFactory::new(POLYLINE_PRECISION, true);
        let route_with_instructions = factory.apply(route).unwrap();
        assert_json_snapshot!(route_with_instructions);
    }

    #[test]
    fn test_instructions_factory_imperial() {
        let route = load_route("../fixtures/valhalla-short.json", 0);
        let factory = InstructionsFactory::new(POLYLINE_PRECISION, false);
        let route_with_instructions = factory.apply(route).unwrap();
        assert_json_snapshot!(route_with_instructions);
    }
}
