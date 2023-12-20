use std::sync::Arc;

use feature_toggles::{feature_toggles::{FeatureToggle, FeatureState}, feature_manager::FeatureManager};
use mysql::prelude::Queryable;

struct Teardown { conn_str: String }

impl Drop for Teardown {
    fn drop(&mut self) {
        use mysql::Pool;
        let pool = Pool::new(self.conn_str.as_str()).unwrap();
        let mut conn = pool.get_conn().unwrap();
    
        conn.query_drop("DROP TABLE features_management.feature_toggles;").unwrap();
        println!("Tearing down.");
    }
}

#[test]
#[cfg(feature = "mysql")]
fn should_create_table_correctly() {
    use feature_toggles::mysql_feature_manager::{use_mysql_feature_manager, FeatureStatuses};
    let conn_str = "mysql://root:password@localhost:3307/db";
    let _teardown = Teardown { conn_str: conn_str.to_string() };
    let feat = Vec::new();
    let feature_statuses = use_mysql_feature_manager(conn_str, feat);
    
    match feature_statuses {
        FeatureStatuses::Empty(_) => assert!(true, "Database does not have any feature flag"),
        _ => assert!(false, "Test failed")
    }
}

#[test]
#[cfg(feature = "mysql")]
fn shouldnt_panic_if_called_more_than_once() {
    use feature_toggles::mysql_feature_manager::{use_mysql_feature_manager, FeatureStatuses};

    let conn_str = "mysql://root:password@localhost:3307/db";
    let _teardown = Teardown { conn_str: conn_str.to_string() };

    let feat = Vec::new();
    let _feature_statuses = use_mysql_feature_manager(conn_str, feat.clone());
    let feature_statuses =  use_mysql_feature_manager(conn_str, feat);

    match feature_statuses {
        FeatureStatuses::FailedInitialization => assert!(false, "Panicked for being called 2 times."),
        _ => assert!(true, "Test Passed")
    }
}

#[test]
#[cfg(feature = "mysql")]
fn loading_feature_toggles_should_work() {
    use feature_toggles::{mysql_feature_manager::{use_mysql_feature_manager, FeatureStatuses}, feature_toggles::FeatureToggle};

    let conn_str = "mysql://root:password@localhost:3307/db";
    let _teardown = Teardown { conn_str: conn_str.to_string() };

    let features = vec![
        FeatureToggle::new("TOGGLE_TEST_FEATURE".to_string(), true),
        FeatureToggle::new("OTHER_TEST_FEATURE".to_string(), false),
        FeatureToggle::new("YET_OTHER_TEST_FEATURE".to_string(), true),
    ];
    
    let feature_statuses = use_mysql_feature_manager(conn_str, features.clone());
    
    match feature_statuses {
        FeatureStatuses::HasAny(manager) 
            => assert_features_are_loaded(manager, features),
        _ => assert!(false, "TEST FAILED WHEN BUILDING FEATURE MANAGER")
    }
}

fn assert_features_are_loaded(feature_manager: Arc<dyn FeatureManager>, features: Vec<FeatureToggle>) {
    features
        .iter()
        .for_each(|feature| {
            match feature_manager.resolve(&feature.name) {
                Some(actual_feature) => 
                    assert_eq!(feature.enabled(), actual_feature.enabled(), "Newly loaded feature should represent saved feature."),
                _ => assert!(false, "Features are not being inserted correctly in the DB.")
            }
        });
}