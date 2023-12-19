use std::sync::Arc;
use mysql::prelude::Queryable;
use crate::{feature_toggles::{FeatureToggle, FeatureState}, 
    feature_manager::FeatureManager};

#[cfg(feature = "mysql")]
struct MySqlFeatureManager {
    features: Vec<FeatureToggle>
}

#[cfg(feature = "mysql")]
impl MySqlFeatureManager {
    fn new(features: Vec<FeatureToggle>) -> Self {
        Self {
            features
        }
    }
}

#[cfg(feature = "mysql")]
impl FeatureManager for MySqlFeatureManager {
    fn resolve(&self, feature_name: &str) -> Option<Box<dyn FeatureState>> {
        let feature = self.features
                .iter()
                .find(|feature| feature.name() == feature_name)
                .cloned();

        match feature {
            Some(feature) => Some(Box::new(feature)),
            None => None,
        }
    }

    fn load(&mut self, features: &mut Vec<FeatureToggle>) -> () {
        insert_features(&features);
        if self.features.len() > 0 {
            self.features.append(features);
        }

        todo!()
    }
}

fn insert_features(features: &[FeatureToggle]) -> () {
    todo!()
}

#[cfg(feature = "mysql")]
pub enum FeatureStatuses {
    HasAny(Arc<dyn FeatureManager>),
    Empty(Arc<dyn FeatureManager>),
    FailedInitialization,
}

#[cfg(feature = "mysql")]
pub fn use_mysql_feature_manager(connection_string: &str) -> FeatureStatuses {
    use mysql::Pool;

    let pool = Pool::new(connection_string).unwrap();
    let conn_result = pool.get_conn();
    
    if let Err(err) = conn_result {
        println!("{:?}", err);
        return FeatureStatuses::FailedInitialization;
    }

    let mut conn = conn_result.unwrap();

    conn.query_drop(r#"CREATE TABLE IF NOT EXISTS feature_toggles (
                name VARCHAR(100) PRIMARY KEY NOT NULL,
                state TINYINT NOT NULL);"#).unwrap();

    let result: Result<Vec<FeatureToggle>, mysql::Error> 
        = conn.query_map("SELECT name, state FROM feature_toggles;",
            |(name, state)| FeatureToggle::new(name, state));

    let result = match result {
        Ok(features) => {
            if features.len() > 0 {
                let my_sql_feature_manager: Arc<dyn FeatureManager> = Arc::new(MySqlFeatureManager::new(features));
                FeatureStatuses::HasAny(my_sql_feature_manager)
            }
            else {
                FeatureStatuses::Empty(Arc::new(MySqlFeatureManager::new(Vec::new())))
            }
        },
        Err(e) => {
            println!("{}", e);
            FeatureStatuses::FailedInitialization
        }
    };

    result
}