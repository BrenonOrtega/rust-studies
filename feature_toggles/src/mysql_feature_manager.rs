use std::sync::Arc;

use mysql::prelude::Queryable;

use crate::{feature_toggles::{FeatureToggle, FeatureState}, 
    feature_manager::FeatureManager};

struct MySqlFeatureManager {
    features: Vec<FeatureToggle>
}

impl MySqlFeatureManager {
    fn new(features: Vec<FeatureToggle>) -> Self {
        Self {
            features
        }
    }
}

impl FeatureManager for MySqlFeatureManager {
    fn resolve(&self, feature_name: &str) -> Option<Box<dyn FeatureState>> {
        let feature = self.features
                .iter()
                .find(|feature| feature.name() == feature_name)
                .cloned();
        
        
    }
}

pub enum FeatureStatuses {
    HasAny(Arc<dyn FeatureManager>),
    FailedInitialization,
    Empty
}

//#[cfg(feature = "mysql_feature_manager")]
pub fn use_mysql_feature_manager(connection_string: &str) -> FeatureStatuses {
    use mysql::Pool;

    let pool = Pool::new(connection_string).unwrap();
    let mut conn_result = pool.get_conn();
    
    if let Err(err) = conn_result {
        println!("{:?}", err);
        return FeatureStatuses::FailedInitialization;
    }

    let conn = conn_result.unwrap();

    let result: Result<Vec<_>, mysql::Error> = conn.exec(
        "CREATE TABLE IF NOT EXISTS feature_toggles (
                name VARCHAR(100) PRIMARY KEY NOT NULL,
                state BOOLEAN NOT NULL);", vec![]);

    let result: Result<Vec<FeatureToggle>, mysql::Error> 
        = conn.query_map("SELECT name, state FROM feature_toggles;", |(name, state)| FeatureToggle::new(name, state));

    let result = match result {
        Ok(features) => {
            if features.len() > 0 {
                let my_sql_feature_manager: Arc<dyn FeatureManager> 
                    = Arc::new(MySqlFeatureManager::new(Arc::new(features)));
                FeatureStatuses::HasAny(my_sql_feature_manager)
            }
            else {
                FeatureStatuses::Empty
            }
        },
        Err(e) => {
            FeatureStatuses::FailedInitialization
        }
    };

    result
}