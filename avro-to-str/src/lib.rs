use apache_avro::{from_value, GenericSingleObjectReader, Schema};
use fluvio_smartmodule::{smartmodule, Record, RecordData, Result};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

lazy_static! {
    static ref READER: GenericSingleObjectReader = GenericSingleObjectReader::new(
        Schema::parse_str(
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
        .expect("valid avro schema")
    )
    .expect("valid reader");
}

#[smartmodule(map)]
pub fn map(record: &Record) -> Result<(Option<RecordData>, RecordData)> {
    let mut cursor = Cursor::new(record.value().as_ref());
    let value = READER.read_value(&mut cursor)?;
    let fact: CatFact = from_value(&value)?;

    let encoded = serde_json::to_string(&fact)?;

    Ok((None, encoded.into()))
}

#[derive(Debug, Serialize, Deserialize)]
struct CatFact {
    fact: String,
    length: u32,
}
