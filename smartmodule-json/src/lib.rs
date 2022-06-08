use fluvio_smartmodule::{smartmodule, Record, RecordData, Result};
use serde::{Deserialize, Serialize};

#[smartmodule(map)]
pub fn mean_visibility_map(record: &Record) -> Result<(Option<RecordData>, RecordData)> {
    // Deserialize input from JSON record using serde_json
    let input = serde_json::from_slice::<InputHKMeanVisibility>(record.value.as_ref())?;

    // transform input into output struct using From trait
    let output = OutputHKMeanVisibility::from(input);

    // Serialize output into JSON using serde_json
    let serialized_output = serde_json::to_vec(&output)?;

    Ok((None, RecordData::from(serialized_output)))
}

#[derive(Deserialize)]
struct InputHKMeanVisibility {
    #[allow(dead_code)]
    fields: Vec<String>,
    data: Vec<Vec<String>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct OutputHKMeanVisibility {
    datetime: String,
    stations_mean_visibility: Vec<StationMeanVisibility>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct StationMeanVisibility {
    station_name: String,
    mean_visibility: String,
}

impl From<InputHKMeanVisibility> for OutputHKMeanVisibility {
    fn from(input: InputHKMeanVisibility) -> Self {
        let datetime = input.data[0][0].to_owned();
        let stations_mean_visibility = input
            .data
            .into_iter()
            .map(|data| StationMeanVisibility {
                station_name: data[1].to_owned(),
                mean_visibility: data[2].to_owned(),
            })
            .collect();
        Self {
            datetime,
            stations_mean_visibility,
        }
    }
}
