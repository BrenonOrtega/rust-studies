use std::sync::Arc;
use mysql::prelude::Queryable;
use mysql::PooledConn;
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
}

#[cfg(feature = "mysql")]
pub enum FeatureStatuses {
    HasAny(Arc<dyn FeatureManager>),
    Empty(Arc<dyn FeatureManager>),
    FailedInitialization,
}

#[cfg(feature = "mysql")]
pub fn use_mysql_feature_manager(connection_string: &str, features: Vec<FeatureToggle>) -> FeatureStatuses {
    use mysql::Pool;

    let pool = Pool::new(connection_string).unwrap();
    let conn_result = pool.get_conn();
    
    if let Err(err) = conn_result {
        println!("{:?}", err);
        return FeatureStatuses::FailedInitialization;
    }

    let mut conn = conn_result.unwrap();

    conn.query_drop(r#"
                CREATE DATABASE IF NOT EXISTS features_management;
                CREATE TABLE IF NOT EXISTS features_management.feature_toggles (
                    name VARCHAR(100) PRIMARY KEY NOT NULL,
                    state TINYINT NOT NULL);"#).unwrap();

    if let Err(e) = insert_features(&mut conn, features) {
        println!("{:?}", e);
        return FeatureStatuses::FailedInitialization;   
    }

    load_feature_manager(conn)
}

#[cfg(feature = "mysql")]
fn insert_features(conn: &mut PooledConn, features: Vec<FeatureToggle>) -> Result<(), mysql::Error> {
    if features.len() == 0 {
        return Ok(());
    }

    let insert = "INSERT INTO features_management.feature_toggles (name, state) VALUES";
    let statements = features.iter()
        .map(|feature| format!("(\'{}\', {})", feature.name, feature.state))
        .reduce(|initial, next| format!("{}, {}", initial, next))
        .unwrap();
    
    let sql = format!("{} {}", insert, statements);

    conn.query_drop(sql)
}

fn load_feature_manager(mut conn: mysql::PooledConn) -> FeatureStatuses {
    let result: Result<Vec<FeatureToggle>, mysql::Error> 
        = conn.query_map("SELECT name, state FROM features_management.feature_toggles;",
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
