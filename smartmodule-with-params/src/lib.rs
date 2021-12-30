use fluvio_smartmodule::{smartmodule, Record, RecordData, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum UserEvent {
    Login(UserEventMetadata),
    Logout(UserEventMetadata),
    Register(UserEventMetadata),
    ActionA(UserEventMetadata),
    ActionB(UserEventMetadata),
    HomePage(UserEventMetadata),
    PlayDemo(UserEventMetadata),
}

#[derive(Deserialize)]
pub struct UserEventMetadata {
    pub account_id: String,
    pub timestamp: i64,
    pub user_client: String,
}

impl UserEventMetadata {
    fn convert(self, params: &SmartModuleOpt) -> UserEventMetadataOutput {
        let account_id = if params.show_account_id {
            Some(self.account_id)
        } else {
            None
        };

        let timestamp = if params.show_timestamp {
            Some(self.timestamp)
        } else {
            None
        };

        let user_client = if params.show_user_client {
            Some(self.user_client)
        } else {
            None
        };
        UserEventMetadataOutput {
            account_id,
            timestamp,
            user_client,
        }
    }
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum UserEventOutput {
    Login(UserEventMetadataOutput),
    Logout(UserEventMetadataOutput),
    Register(UserEventMetadataOutput),
    ActionA(UserEventMetadataOutput),
    ActionB(UserEventMetadataOutput),
    HomePage(UserEventMetadataOutput),
    PlayDemo(UserEventMetadataOutput),
}

#[derive(Serialize)]
pub struct UserEventMetadataOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_client: Option<String>,
}

impl UserEvent {
    fn convert(self, params: &SmartModuleOpt) -> UserEventOutput {
        match self {
            UserEvent::Login(metadata) => UserEventOutput::Login(metadata.convert(params)),
            UserEvent::Logout(metadata) => UserEventOutput::Logout(metadata.convert(params)),
            UserEvent::Register(metadata) => UserEventOutput::Register(metadata.convert(params)),
            UserEvent::ActionA(metadata) => UserEventOutput::ActionA(metadata.convert(params)),
            UserEvent::ActionB(metadata) => UserEventOutput::ActionB(metadata.convert(params)),
            UserEvent::HomePage(metadata) => UserEventOutput::HomePage(metadata.convert(params)),
            UserEvent::PlayDemo(metadata) => UserEventOutput::PlayDemo(metadata.convert(params)),
        }
    }
}

#[smartmodule(map, params)]
pub fn map(record: &Record, params: &SmartModuleOpt) -> Result<(Option<RecordData>, RecordData)> {
    let event: UserEvent = serde_json::from_slice(record.value.as_ref())?;
    let output = event.convert(params);
    let value = serde_json::to_string(&output)?;
    Ok((record.key.clone(), value.into()))
}

#[derive(fluvio_smartmodule::SmartOpt)]
pub struct SmartModuleOpt {
    show_account_id: bool,
    show_timestamp: bool,
    show_user_client: bool,
}

impl Default for SmartModuleOpt {
    fn default() -> Self {
        Self {
            show_account_id: true,
            show_timestamp: true,
            show_user_client: true,
        }
    }
}
