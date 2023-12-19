
use crate::feature_toggles::{FeatureState, FeatureToggle};

pub trait FeatureManager {
    fn resolve(&self, feature_name: &str) -> Option<Box<dyn FeatureState>>;

    fn load(&mut self, features: &mut Vec<FeatureToggle>) -> ();
}