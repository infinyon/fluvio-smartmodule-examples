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
    AddToCart {
        item_id: String,
        item_name: String,
    },
    AddBilling {
        account_id: String,
        card_holder: String,
        card_number: String,
        expiration: String,
        security_code: String,
    },
    Checkout {
        account_id: String,
    },
    OrderBegun {
        account_id: String,
        sms_number: String,
        sms_name: String,
    },
    ItemStatus {
        account_id: String,
        item_name: String,
        status: String,
        sms_number: String,
        sms_name: String,
    },
    OrderReady {
        account_id: String,
        sms_number: String,
        sms_name: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct SmsMessage {
    number: String,
    message: String,
}

#[smartmodule(filter_map)]
fn filter_map(record: &Record) -> Result<Option<(Option<RecordData>, RecordData)>> {
    let event: GroceryEvent = match serde_json::from_slice(record.value.as_ref()) {
        Ok(event) => event,
        Err(_) => return Ok(None), // Skip if we fail to parse JSON
    };

    let sms_message = match event {
        GroceryEvent::OrderBegun {
            sms_name,
            sms_number,
            ..
        } => SmsMessage {
            number: sms_number,
            message: format!("Hello {}, your groceries are being collected!", sms_name),
        },
        GroceryEvent::ItemStatus {
            sms_name,
            sms_number,
            item_name,
            status,
            ..
        } => SmsMessage {
            number: sms_number,
            message: format!(
                "Hello {}, we have an update on your {}: {}",
                sms_name, item_name, status
            ),
        },
        GroceryEvent::OrderReady {
            sms_name,
            sms_number,
            ..
        } => SmsMessage {
            number: sms_number,
            message: format!(
                "Hello {}, your groceries have been collected and are ready to pick up!",
                sms_name
            ),
        },
        _ => return Ok(None),
    };

    let message_json = serde_json::to_string(&sms_message)?;
    Ok(Some((record.key.clone(), message_json.into())))
}
