
#[test]
#[cfg(feature = "mysql")]
fn should_create_table_correctly() {
    use feature_toggles::mysql_feature_manager::{use_mysql_feature_manager, FeatureStatuses};

    let conn_string = "mysql://user:password@localhost:3307/db";
    let option = use_mysql_feature_manager(conn_string);

    assert_eq!(option, FeatureStatuses::Empty);
}