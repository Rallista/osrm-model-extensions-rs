use crate::instructions::utilities::step_maneuver_name;
use osrm_openapi_models::models::{
    BannerContent, BannerContentComponentsInner, BannerInstruction, ManeuverType, RouteStep,
};
use rust_i18n::t;

#[derive(Debug)]
pub struct BannerInstructionsFactory {
    next_step: Option<RouteStep>,
    step: Option<RouteStep>,
}

impl BannerInstructionsFactory {
    pub fn new(next_step: Option<RouteStep>, step: Option<RouteStep>) -> Self {
        BannerInstructionsFactory { next_step, step }
    }

    pub fn build(&self) -> Vec<BannerInstruction> {
        vec![self.generate()]
    }

    fn generate(&self) -> BannerInstruction {
        self.next_step
            .as_ref()
            .map(|step| {
                let length = self
                    .step
                    .clone()
                    .map_or(0.0, |step| step.distance.unwrap_or(0.0));
                let maneuver = step.maneuver.as_ref().unwrap();
                let name = step_maneuver_name(step.clone());

                let component = BannerContentComponentsInner {
                    r#type: Some("text".to_string()),
                    text: Some(name.clone()),
                };

                let primary = BannerContent {
                    text: name,
                    r#type: maneuver.r#type,
                    modifier: maneuver.modifier,
                    components: Some(vec![component]),
                };

                BannerInstruction {
                    distance_along_geometry: length,
                    primary: Box::new(primary),
                    secondary: None,
                }
            })
            .unwrap_or(self.arrival())
    }

    fn arrival(&self) -> BannerInstruction {
        let text = t!("arrive").to_string();

        let component = BannerContentComponentsInner {
            r#type: Some("text".to_string()),
            text: Some(text.clone()),
        };

        let primary = BannerContent {
            text,
            r#type: Some(ManeuverType::Arrive),
            modifier: None,
            components: Some(vec![component]),
        };

        BannerInstruction {
            distance_along_geometry: 0.0,
            primary: Box::new(primary),
            secondary: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_json_snapshot;
    use osrm_openapi_models::models::{ManeuverModifier, ManeuverType, StepManeuver};

    #[test]
    fn test_banner_with_next_step() {
        let next_step = RouteStep {
            distance: Some(100.0),
            maneuver: Some(Box::new(StepManeuver {
                r#type: Some(ManeuverType::Turn),
                modifier: Some(ManeuverModifier::Right),
                ..Default::default()
            })),
            name: Some("Main Street".to_string()),
            ..Default::default()
        };

        let current_step = RouteStep {
            distance: Some(50.0),
            ..Default::default()
        };

        let factory = BannerInstructionsFactory::new(Some(next_step), Some(current_step));
        assert_json_snapshot!(factory.build());
    }

    #[test]
    fn test_banner_arrival() {
        let factory = BannerInstructionsFactory::new(None, None);
        assert_json_snapshot!(factory.build());
    }

    #[test]
    fn test_banner_no_current_step_distance() {
        let next_step = RouteStep {
            distance: Some(100.0),
            maneuver: Some(Box::new(StepManeuver {
                r#type: Some(ManeuverType::Merge),
                modifier: Some(ManeuverModifier::SlightLeft),
                ..Default::default()
            })),
            name: Some("Main Street".to_string()),
            ..Default::default()
        };

        let current_step = RouteStep {
            distance: None,
            ..Default::default()
        };

        let factory = BannerInstructionsFactory::new(Some(next_step), Some(current_step));
        assert_json_snapshot!(factory.build());
    }
}
