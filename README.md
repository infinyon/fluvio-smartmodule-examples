# Fluvio SmartModule Examples

This repository contains complete examples use-cases of SmartModules.
To learn more about SmartModules, visit [the docs on fluvio.io][1].

[1]: https://fluvio.io/docs/smartmodules/overview

| Example | SmartModule | Blog/Guide |
| --- | --- | --- |
| [Log Level](./log-level/src/lib.rs)| [filter](https://www.fluvio.io/docs/smartstreams/filter/) | [Blog: Write a WASM-based filter for application logs](https://www.infinyon.com/blog/2021/06/smartstream-filters/)|
| [Regex scrubbing](./regex-scrubbing/src/lib.rs) | [map](https://www.fluvio.io/docs/smartstreams/map/) | [Blog: Transforming streaming data in real-time with WebAssembly](https://www.infinyon.com/blog/2021/08/smartstream-map-use-cases/)
| [GitHub Stars](./github-stars/src/lib.rs) | [map](https://www.fluvio.io/docs/smartstreams/map/) | [Guide: How to use SmartModules with the HTTP Smart Connector](https://fluvio.io/connectors/examples/github) |
| [Summing Integers](./summing-integers/src/lib.rs) | [aggregate](https://www.fluvio.io/docs/smartstreams/aggregate/) | [Blog: Aggregate streaming data in real-time with WebAssembly](https://www.infinyon.com/blog/2021/08/smartstream-aggregates/) |
| [Reddit-pagination](./reddit-pagination/src/lib.rs) | [array-map](https://www.fluvio.io/docs/smartstreams/array-map/) | [Blog: Streaming the Reddit API using Fluvio's WASM ArrayMap](https://www.infinyon.com/blog/2021/10/smartstream-array-map-reddit/) |
| [Grocery Notifications](./grocery-notifications/src/lib.rs) | filter-map | [Blog: Using Fluvio FilterMap to apply focus to real-time data](https://www.infinyon.com/blog/2021/11/filter-map/) |
| [Json-to-Yaml](./json-to-yaml/src/lib.rs) | [map](https://www.fluvio.io/docs/smartstreams/map/) | |
| [Incremental Average](./incremental-average/src/lib.rs) | [aggregate](https://www.fluvio.io/docs/smartstreams/aggregate/) | |
| [Json Array Expansion](./json-array-expansion) | array | |

## Types of SmartModules

Below are quick introductions to the various types of SmartModules and how to use them.

### Filters

Filters are SmartModules that allow you to choose whether each record in the stream
should be kept (by returning `true`) or discarded (by returning `false`).
To make a filter, write your SmartModule using `#[smartmodule(filter)]` on your
top-level function, like this.

```rust
use fluvio_smartmodule::{smartmodule, Record, Result};

#[smartmodule(filter)]
pub fn filter(record: &Record) -> Result<bool> {
    let string = std::str::from_utf8(record.value.as_ref())?;
    Ok(string.contains('a'))
}
```

This filter will keep only records whose contents contain the letter `a`.

### Maps

Mapping functions use `#[smartmodule(map)]`, and allow you to transform each input
record to a new output record.

```rust
use fluvio_smartmodule::{smartmodule, Record, RecordData, Result};

#[smartmodule(map)]
pub fn map(record: &Record) -> Result<(Option<RecordData>, RecordData)> {
    let key = record.key.clone();

    let string = std::str::from_utf8(record.value.as_ref())?;
    let int = string.parse::<i32>()?;
    let value = (int * 2).to_string();

    Ok((key, value.into()))
}
```

This SmartModule will read each input Record as an integer (`i32`), then multiply it by 2.

### Aggregates

Aggregate functions are a way to combine the data from many input records.
Each time the aggregate function is called, it receives an "accumulated" value
as well as the value of the current record in the stream, and is expected to
combine the accumulator with the value to produce a new accumulator. This new
accumulator value will be passed to the next invocation of `aggregate` with
the next record value. The resulting stream of values is the output accumulator
from each step.

```rust
use fluvio_smartmodule::{smartmodule, Result, Record, RecordData};

#[smartmodule(aggregate)]
pub fn aggregate(accumulator: RecordData, current: &Record) -> Result<RecordData> {
    let mut acc = String::from_utf8(accumulator.as_ref().to_vec())?;
    let next = std::str::from_utf8(current.value.as_ref())?;
    acc.push_str(next);
    Ok(acc.into())
}
```

This SmartModule reads each record as a string and appends it to the accumulator string.

### ArrayMaps

ArrayMap functions are used to take one input record and create zero to many output records.
This can be used to chop up input records that logically represent more than one data point
and turn them into independent records. Below is an example where we take JSON arrays and
convert them into a stream of the inner JSON objects.

```rust
use fluvio_smartmodule::{smartmodule, Result, Record, RecordData};

#[smartmodule(array_map)]
pub fn array_map(record: &Record) -> Result<Vec<(Option<RecordData>, RecordData)>> {
    // Read the input record as a JSON array
    let array = serde_json::from_slice::<Vec<serde_json::Value>>(record.value.as_ref())?;
    
    // Convert each individual value from the array into its own JSON string
    let strings = array
        .into_iter()
        .map(|value| serde_json::to_string(&value))
        .collect::<core::result::Result<Vec<String>, _>>()?;
        
    // Return a list of records to be flattened into the output stream
    let kvs = strings
        .into_iter()
        .map(|s| (None, RecordData::from(s)))
        .collect::<Vec<_>>();
    Ok(kvs)
}
```

# License

This project is licensed under the [Apache license](LICENSE-APACHE).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Fluvio by you, shall be licensed as Apache, without any additional
terms or conditions.
