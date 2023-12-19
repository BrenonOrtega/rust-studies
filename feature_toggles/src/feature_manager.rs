
use std::sync::Arc;

use crate::feature_toggles::FeatureState;

pub trait FeatureManager {
    fn resolve(&self, feature_name: &str) -> Option<Arc<dyn FeatureState>>;
}