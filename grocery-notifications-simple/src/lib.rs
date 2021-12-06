use fluvio_smartmodule::{smartmodule, Record, RecordData, Result};
use serde::{Deserialize, Serialize};

/// Events that may take place in an online grocery service
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum GroceryEvent {
    AccountCreated {
        account_id: String,
        username: String,
        preferred_name: String,
        phone_number: String,
    },
    OrderReady {
        account_id: String,
        sms_number: String,
        sms_name: String,
    },
}

#[smartmodule(filter_map)]
fn filter_map(record: &Record) -> Result<Option<(Option<RecordData>, RecordData)>> {
    let event: GroceryEvent = match serde_json::from_slice(record.value.as_ref()) {
        Ok(event) => event,
        Err(_) => return Ok(None), // Skip if we fail to parse JSON
    };

    let sms_message = match event {
        GroceryEvent::OrderReady {
            sms_name,
            sms_number,
            ..
        } => serde_json::json!({
            "number": sms_number,
            "message": format!(
                "Hello {}, your groceries have been collected and are ready to pick up!",
                sms_name
            ),
        }),
        _ => return Ok(None),
    };

    let message_json = serde_json::to_string(&sms_message)?;
    Ok(Some((record.key.clone(), message_json.into())))
}
