use diesel::{deserialize::Queryable, Selectable, prelude::Insertable};
use uuid::Uuid;

//#[derive(Queryable,Selectable,Insertable)]
//#[derive(Debug)]
struct Transaction {
    pub id: Uuid, 
    pub from: String,
    pub to: String,
    pub value: f32,
}

impl Transaction {
    fn new() -> Transaction {
        Transaction {
            id: Uuid::new_v4(),
            from: "".to_string(),
            to: "".to_string(),
            value: 0.0,
        }
    }
}