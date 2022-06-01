/// Reads XML data and returns a Vec of Json Strings.
/// All this information is based on the output from
/// the bike occupancy API from Transport for London.
use fluvio_smartmodule::{smartmodule, Record, RecordData, Result};
use serde::{Deserialize, Serialize};

#[smartmodule(array_map)]
pub fn array_map(record: &Record) -> Result<Vec<(Option<RecordData>, RecordData)>> {
    // Deserialize XML from record
    let array = quick_xml::de::from_slice::<ArrayOfBikePointOccupancy>(record.value.as_ref())?;

    // Create a Json string for each bike point occupancy
    let strings: Vec<String> = array
        .bike_point_occupancy
        .into_iter()
        .map(|value| serde_json::to_string(&value))
        .collect::<core::result::Result<_, _>>()?;

    // Create one record from each JSON string to send
    let kvs: Vec<(Option<RecordData>, RecordData)> = strings
        .into_iter()
        .map(|s| (None, RecordData::from(s)))
        .collect();
    Ok(kvs)
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ArrayOfBikePointOccupancy {
    bike_point_occupancy: Vec<BikePointOccupancy>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BikePointOccupancy {
    bikes_count: usize,
    e_bikes_count: usize,
    empty_docks: usize,
    id: String,
    name: String,
    standard_bikes_count: usize,
    total_docks: usize,
}
