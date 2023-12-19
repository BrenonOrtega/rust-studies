use mysql::prelude::Queryable;

struct Teardown { conn_str: String }

impl Drop for Teardown {
    fn drop(&mut self) {
        use mysql::Pool;
        let pool = Pool::new(self.conn_str.as_str()).unwrap();
        let mut conn = pool.get_conn().unwrap();
    
        conn.query_drop("DROP TABLE feature_toggles;").unwrap();
        println!("Tearing down after each test...");
    }
}

#[test]
#[cfg(feature = "mysql")]
fn should_create_table_correctly() {
    use feature_toggles::mysql_feature_manager::{use_mysql_feature_manager, FeatureStatuses};
    let conn_str = "mysql://user:password@localhost:3307/test";
    let _teardown = Teardown { conn_str: conn_str.to_string() };
    let option = use_mysql_feature_manager(conn_str);
    
    match option {
        FeatureStatuses::Empty(_) => assert!(true, "Database does not have any feature flag"),
        _ => assert!(false, "Test failed")
    }
}

#[test]
#[cfg(feature = "mysql")]
fn shouldnt_panic_if_called_more_than_once() {
    use feature_toggles::mysql_feature_manager::{use_mysql_feature_manager, FeatureStatuses};

    let conn_str = "mysql://user:password@localhost:3307/test";
    let _teardown = Teardown { conn_str: conn_str.to_string() };

    let option = use_mysql_feature_manager(conn_str);
    let option =  use_mysql_feature_manager(conn_str);

    match option {
        FeatureStatuses::FailedInitialization => assert!(false, "Panicked for being called 2 times."),
        _ => assert!(true, "Test Passed")
    }
    
}

#[test]
#[cfg(feature = "mysql")]
fn loading_feature_toggles_should_work() {
    use feature_toggles::{mysql_feature_manager::{use_mysql_feature_manager, FeatureStatuses}, feature_manager};

    let conn_str = "mysql://user:password@localhost:3307/test";
    let _teardown = Teardown { conn_str: conn_str.to_string() };

    let option = use_mysql_feature_manager(conn_str);

    let mut feature_manager;
    match option {
        FeatureStatuses::Empty(manager) => feature_manager = manager,
        _ => panic!()
    }

    //feature_manager.load()

}
