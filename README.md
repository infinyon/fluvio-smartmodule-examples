# Fluvio SmartModule Examples

This repository contains complete examples use-cases of SmartModules.
To learn more about SmartModules, visit [the docs on fluvio.io][1]

Examples in this repo:

- [Log Level (filter)](./log-level/src/lib.rs)
  - [Blog: Write a WASM-based filter for application logs][2]
- [Regex scrubbing (map)](./regex-scrubbing/src/lib.rs)
  - [Blog: Transforming streaming data in real-time with WebAssembly][3]
- [GitHub Stars (map)](./github-stars/src/lib.rs)
  - [Guide: How to use SmartModules with the HTTP Smart Connector][4]
- [Json-to-Yaml (map)](./json-to-yaml/src/lib.rs)
- [Reddit-pagination (array-map)](./reddit-pagination/src/lib.rs)
  - [Blog: Streaming the Reddit API using Fluvio's WASM ArrayMap](https://www.infinyon.com/blog/2021/10/smartstream-array-map-reddit/)

[1]: https://fluvio.io/docs/smartmodules/overview
[2]: https://www.infinyon.com/blog/2021/06/smartstream-filters/
[3]: https://www.infinyon.com/blog/2021/08/smartstream-map-use-cases/
[4]: https://fluvio.io/connectors/examples/github
