use super::from_str;
use std::time::Instant;

pub(super) trait ChangeLogTrait {
    fn to_sql(&self, table: &str) -> String;
}

pub(super) struct ChangeLog  {
    pub(super) id: i64,
    pub(super) entity_type: String,
    pub(super) old_value: String, 
    pub(super) new_value: String,
    pub(super) timestamp: String,
    pub(super) sent_by: String,
    pub(super) service: String
}

impl ChangeLog {
    pub(super) fn new(entity_type: &str, old_value: &str, new_value: &str,
        sent_by: &str, service: &str) -> ChangeLog {
        let timestamp = format!("{:?}", Instant::now());
        ChangeLog { 
            id: 0,
            entity_type: from_str(entity_type), 
            old_value: from_str(old_value), 
            new_value: from_str(new_value), 
            timestamp: from_str(&timestamp.as_str()), 
            sent_by: from_str(sent_by),
            service: from_str(service) 
        }
    }
}

impl ChangeLogTrait for ChangeLog {
    fn to_sql(&self, table: &str) -> String {
        let res = format!("INSERT INTO {} (entity_type, old_value, new_value, timestamp, sent_by, service) VALUES ({}, {}, {}, {}, {}, {}", table, self.entity_type, self.old_value, self.new_value, 
            self.timestamp, self.sent_by, self.service);
    
        return res;
    }
}
