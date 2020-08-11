# Broadsign Real-Time Pop server sample
**This is an unofficial server example**. It is discouraged to use it as-is for production uses.
It comes with **no warranty**.

## About Real-Time Pop server implementation
This is an implementation of version Broadsign Control 13.2's real-time pop protocol:
https://docs.broadsign.com/broadsign-control/13-2/real-time-pop-api.html

## Installing Rust
Follow the steps indicated on the official page:
https://www.rust-lang.org/tools/install

## Using VS Code
1. Install `rust-analyzer` and  `Rust` extensions.
2. Recommended: Change `Rust` extension settings to use `rust-analyzer` instead of `rls` (`ctrl + ,`).
3. Run unit tests using `cargo test`.
4. Run using `cargo run`. Use `cargo run --release` for optimizations.

## Log levels
You may set the log level by prepending `RUST_LOG=<level>`, where level corresponds to:
* `debug`
* `error`
* `warn`
* `info` (default)
* `trace`
```
RUST_LOG=trace cargo run
```
_Note that logs messages are not currently enabled in the tests by lack of time and will._

## See proof of plays
They are stored in the SQLite `pops.db` file in the working directory. For the tests, a `test.db` fle is used, instead.

## Using Insomnia
An Insomnia file is available (`api_insomnia.json`). You may import it to help you debug or comprehend how to use this real-time pop server implementation.

### Request Json format
```
{
   "api_key": "{{ api_key }}",
   "player_id": 12345,
	"pop": [
  	{
      "display_unit_id": 4456,
      "frame_id": 4457,
      "n_screens": 1,
      "ad_copy_id": 5001,
      "campaign_id": 5002,
      "schedule_id": 5003,
      "impressions": 2,
      "interactions": 0,
      "end_time": "2016-05-31T10:14:50.200",
      "duration": 5000,
      "ext1": "bmb",
      "ext2": "3451",
      "extra_data": ""
    },
    {
      "display_unit_id": 3456,
      "frame_id": 3457,
      "n_screens": 1,
      "ad_copy_id": 7001,
      "campaign_id": 7002,
      "schedule_id": 7003,
      "impressions": 4,
      "interactions": 1,
      "end_time": "2016-05-31T10:14:55.200",
      "duration": 5000,
      "ext1": "",
      "ext2": "",
      "extra_data": ""
     }
	]
}
```

## License
MIT Licensed. See LICENSE.md
