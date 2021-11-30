use fluvio_smartmodule::{smartmodule, Result};
use fluvio_smartmodule::extract::*;
use fluvio_model_github::GhRepo;
use serde_json::{json, Value as JsonValue};

#[smartmodule(map)]
pub fn map(record: Value<Json<GhRepo>>) -> Result<Value<Json<JsonValue>>> {
    let repo = record.inner();

    Ok(Value(Json(json!({
        "stars": repo.stargazers_count,
        "forks": repo.forks_count,
    }))))
}
