use fluvio_smartmodule::{smartmodule, Record, RecordData, Result};
use serde_json::{json, Value as JsonValue};

#[smartmodule(map)]
fn shaper(record: &Record) -> Result<(Option<RecordData>, RecordData)> {
    let repo: JsonValue = serde_json::from_slice(record.value.as_ref())?;

    let shaped = json!({
        "stars": repo["stargazers_count"],
        "forks": repo["forks_count"],
    });

    let output = serde_json::to_vec(&shaped)?;
    Ok((record.key.clone(), RecordData::from(output)))
}
