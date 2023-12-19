pub trait FeatureState {
    fn enabled(&self) -> bool;
    fn disabled(&self) -> bool;
    fn name(&self) -> &str;
}

#[derive(Debug, PartialEq, Eq)]
pub struct FeatureToggle {
    pub name: String,
    state: bool,
}

impl FeatureToggle {
    pub(crate) fn new(name: String, state: bool) -> Self {
        Self {
            name, 
            state
        }
    }
}

impl Default for FeatureToggle {
    fn default() -> Self {
        FeatureToggle {
            name: "DEFAULT_FEATURE".to_string(),
            state: false
        }
    }
}

impl Clone for FeatureToggle {
    fn clone(&self) -> Self {
        Self { name: self.name.clone(), state: self.state.clone() }
    }
}

impl FeatureState for FeatureToggle {
    fn enabled(&self) -> bool {
        self.state
    }

    fn disabled(&self) -> bool {
       !self.enabled()
    }

    fn name(&self) -> &str {
        &self.name
    }
}
