extern crate serde_derive;

use chrono::naive::NaiveDateTime;
use serde_derive::{Deserialize, Serialize};

/* A proof of play (pop) entry from the request. Some fields were aliased
 * to be more explicit.
 */
#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct RealTimePopEntry {
    pub display_unit_id: u64,
    pub frame_id: u64,
    #[serde(rename = "n_screens")]
    pub active_screens_count: u32,
    pub ad_copy_id: u64,
    pub campaign_id: u64,
    pub schedule_id: u64,
    pub impressions: u32,
    pub interactions: u32,
    // Since the Broadsign Player sends the timestamp in its local time, we cannot safely
    // deduce a time zone, so we're using NaiveDateTime. If your players are guaranteed to
    // be on the same timezone as the server, you may use:
    //
    //    chrono::DateTime<chrono::Local> (DateTime<Local>)
    pub end_time: NaiveDateTime,
    #[serde(rename = "duration")]
    pub duration_ms: u32,
    #[serde(rename = "ext1")]
    pub service_name: String,
    #[serde(rename = "ext2")]
    pub service_value: String,
    // `extra_data` field has been added in 13.2; if you have players from both 13.0, 13.1 and
    // 13.2, you may want to make it optional, otherwise the parsing _will_ fail.
    //
    //    extra_data: Option<serde_json::Value>
    //
    // You may also want to strongly type it (create a struct and derive serde).
    pub extra_data: Option<serde_json::Value>,
}

/* Request sent by Broadsign Player, as defined on Broadsign's website:
 * https://docs.broadsign.com/broadsign-control/13-2/real-time-pop-api.html
 */
#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct RealTimePopRequest {
    pub api_key: String,
    pub player_id: u64,
    #[serde(rename = "pop")]
    pub pops: Vec<RealTimePopEntry>,
}

/*
 * Test section
 */
#[cfg(test)]
mod protocol_serialization_tests {
    use super::*;
    use chrono::NaiveDate;

    fn assert_valid_request_content(deserialized: RealTimePopRequest) {
        assert_eq!(deserialized.api_key, "some_secure_api_key");
        assert_eq!(deserialized.player_id, 12345);
        assert_eq!(deserialized.pops.len(), 2);

        // Validate first pop
        assert_eq!(deserialized.pops[0].display_unit_id, 4456);
        assert_eq!(deserialized.pops[0].frame_id, 4457);
        assert_eq!(deserialized.pops[0].active_screens_count, 1);
        assert_eq!(deserialized.pops[0].ad_copy_id, 5001);
        assert_eq!(deserialized.pops[0].campaign_id, 5002);
        assert_eq!(deserialized.pops[0].schedule_id, 5003);
        assert_eq!(deserialized.pops[0].impressions, 2);
        assert_eq!(deserialized.pops[0].interactions, 0);
        assert_eq!(
            deserialized.pops[0].end_time,
            NaiveDate::from_ymd(2016, 5, 31).and_hms_milli(10, 14, 50, 200)
        );
        assert_eq!(deserialized.pops[0].duration_ms, 5000);
        assert_eq!(deserialized.pops[0].service_name, "bmb");
        assert_eq!(deserialized.pops[0].service_value, "3451");
        assert_eq!(deserialized.pops[0].extra_data.is_some(), true);
        assert_eq!(deserialized.pops[0].extra_data.as_ref().unwrap(), "");

        // Validate second pop
        assert_eq!(deserialized.pops[1].display_unit_id, 3456);
        assert_eq!(deserialized.pops[1].frame_id, 3457);
        assert_eq!(deserialized.pops[1].active_screens_count, 1);
        assert_eq!(deserialized.pops[1].ad_copy_id, 7001);
        assert_eq!(deserialized.pops[1].campaign_id, 7002);
        assert_eq!(deserialized.pops[1].schedule_id, 7003);
        assert_eq!(deserialized.pops[1].impressions, 4);
        assert_eq!(deserialized.pops[1].interactions, 1);
        assert_eq!(
            deserialized.pops[1].end_time,
            NaiveDate::from_ymd(2016, 5, 31).and_hms_milli(10, 14, 55, 200)
        );
        assert_eq!(deserialized.pops[1].duration_ms, 5000);
        assert_eq!(deserialized.pops[1].service_name, "");
        assert_eq!(deserialized.pops[1].service_value, "");
        assert_eq!(deserialized.pops[1].extra_data.is_some(), true);
        assert_eq!(deserialized.pops[1].extra_data.as_ref().unwrap(), "");
    }

    #[test]
    fn given_valid_non_verbose_request_deserialization_is_success() {
        let request = r#"
		    {
		        "api_key": "some_secure_api_key",
		        "player_id": 12345,
		        "pop": [
			        [4456, 4457, 1, 5001, 5002, 5003, 2, 0, "2016-05-31T10:14:50.200", 5000, "bmb", "3451", ""],
			        [3456, 3457, 1, 7001, 7002, 7003, 4, 1, "2016-05-31T10:14:55.200", 5000, "", "", ""]
		        ]
            }"#;

        let deserialized = serde_json::from_str::<RealTimePopRequest>(request).unwrap();
        assert_valid_request_content(deserialized);
    }

    #[test]
    fn given_valid_verbose_request_deserialization_is_success() {
        let request = r#"
		    {
		        "api_key": "some_secure_api_key",
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
            }"#;

        let deserialized = serde_json::from_str::<RealTimePopRequest>(request).unwrap();
        assert_valid_request_content(deserialized);
    }
}
