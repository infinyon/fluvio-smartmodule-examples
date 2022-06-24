use apache_avro::{GenericSingleObjectWriter, Schema};
use fluvio_smartmodule::{smartmodule, Record, RecordData, Result};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;

lazy_static! {
    static ref SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "cat_fact",
        "fields": [
            {"name": "fact", "type": "string"},
            {"name": "length", "type": "int"}
        ]
    }
    "#
    )
    .expect("valid avro schema");
}

thread_local!(static WRITER: RefCell<GenericSingleObjectWriter> = RefCell::new(GenericSingleObjectWriter::new_with_capacity(&SCHEMA, 1024).expect("valid reader")));

#[smartmodule(map)]
pub fn map(record: &Record) -> Result<(Option<RecordData>, RecordData)> {
    let fact: CatFact = serde_json::from_slice(record.value().as_ref())?;

    let mut encoded = Vec::new();

    WRITER.with(|cell| cell.borrow_mut().write(&fact, &mut encoded))?;

    Ok((None, encoded.into()))
}

#[derive(Debug, Serialize, Deserialize)]
struct CatFact {
    fact: String,
    length: u32,
}
