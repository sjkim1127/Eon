//! End-to-end checks of the stdio MCP server (JSON-RPC 2.0).

use serde_json::Value;
use std::io::Write;
use std::process::{Command, Stdio};

/// Sends one JSON-RPC message per line and returns the parsed responses in order.
fn exchange(requests: &[&str]) -> Vec<Value> {
    let mut child = Command::new(env!("CARGO_BIN_EXE_eon"))
        .arg("mcp")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn eon mcp");
    {
        let stdin = child.stdin.as_mut().unwrap();
        for r in requests {
            writeln!(stdin, "{r}").unwrap();
        }
    }
    let out = child.wait_with_output().unwrap();
    String::from_utf8(out.stdout)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).expect("each response line is JSON"))
        .collect()
}

fn call(args: &str) -> String {
    format!(
        r#"{{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{{"name":"eon_analyze","arguments":{args}}}}}"#
    )
}

#[test]
fn ping_returns_an_empty_result() {
    let r = exchange(&[r#"{"jsonrpc":"2.0","id":7,"method":"ping"}"#]);
    assert_eq!(r.len(), 1);
    assert_eq!(r[0]["id"], 7);
    assert_eq!(r[0]["result"], serde_json::json!({}));
}

#[test]
fn notifications_get_no_reply() {
    let r = exchange(&[
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/cancelled","params":{}}"#,
        r#"{"jsonrpc":"2.0","id":1,"method":"ping"}"#,
    ]);
    assert_eq!(r.len(), 1, "only the ping request may be answered");
}

#[test]
fn tool_failures_are_reported_in_the_result_not_as_protocol_errors() {
    let r = exchange(&[
        &call(r#"{"birth":"2023-02-30 12:00"}"#),
        &call(r#"{}"#),
        &call(r#"{"birth":"1990-05-15 10:30","format":"bogus"}"#),
    ]);
    assert_eq!(r.len(), 3);
    for resp in &r {
        assert!(
            resp.get("error").is_none(),
            "unexpected protocol error: {resp}"
        );
        assert_eq!(resp["result"]["isError"], true);
        assert!(resp["result"]["content"][0]["text"].as_str().unwrap().len() > 10);
    }
}

#[test]
fn unknown_tool_is_a_protocol_error() {
    let r = exchange(&[
        r#"{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"nope","arguments":{}}}"#,
    ]);
    assert_eq!(r[0]["error"]["code"], -32602);
}

#[test]
fn lunar_leap_and_negative_coordinates_work() {
    let r = exchange(&[
        &call(r#"{"birth":"2023-02-01","lunar":true,"leap":true,"format":"markdown"}"#),
        &call(
            r#"{"birth":"1990-05-15 10:30","lat":-33.87,"lon":151.2,"timezone":"Australia/Sydney"}"#,
        ),
    ]);
    assert_eq!(r.len(), 2);
    let md = r[0]["result"]["content"][0]["text"].as_str().unwrap();
    assert!(
        md.starts_with("# Eon:"),
        "markdown format must use the report renderer"
    );
    assert!(r[0]["result"].get("isError").is_none());
    let agent = r[1]["result"]["content"][0]["text"].as_str().unwrap();
    assert!(agent.contains("Lat: -33") || agent.contains("-33.87"));
}

#[test]
fn schema_advertises_lunar_and_leap() {
    let r = exchange(&[r#"{"jsonrpc":"2.0","id":1,"method":"tools/list"}"#]);
    let props = &r[0]["result"]["tools"][0]["inputSchema"]["properties"];
    assert!(props.get("lunar").is_some());
    assert!(props.get("leap").is_some());
}

fn tool_call(name: &str, args: &str) -> String {
    format!(
        r#"{{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{{"name":"{name}","arguments":{args}}}}}"#
    )
}

fn text_of(resp: &Value) -> &str {
    resp["result"]["content"][0]["text"].as_str().unwrap_or("")
}

#[test]
fn tools_list_exposes_every_engine_and_the_two_person_tools() {
    let r = exchange(&[r#"{"jsonrpc":"2.0","id":1,"method":"tools/list"}"#]);
    let tools = r[0]["result"]["tools"].as_array().unwrap();
    let names: Vec<&str> = tools.iter().map(|t| t["name"].as_str().unwrap()).collect();

    for expected in [
        "eon_analyze",
        "eon_saju",
        "eon_saju_luck",
        "eon_report",
        "eon_vedic",
        "eon_western",
        "eon_zwds",
        "eon_human_design",
        "eon_human_design_penta",
        "eon_qimen",
        "eon_numerology",
        "eon_iching",
        "eon_compatibility",
    ] {
        assert!(
            names.contains(&expected),
            "missing tool {expected}: {names:?}"
        );
    }
    for tool in tools {
        assert_eq!(tool["inputSchema"]["type"], "object", "{}", tool["name"]);
        assert!(tool["description"].as_str().unwrap().len() > 20);
    }
    let compat = tools
        .iter()
        .find(|t| t["name"] == "eon_compatibility")
        .unwrap();
    let required = compat["inputSchema"]["required"].as_array().unwrap();
    assert!(required.iter().any(|v| v == "partner_birth"));
}

#[test]
fn engine_tools_return_text_summaries_and_json_on_request() {
    let r = exchange(&[
        &tool_call("eon_saju", r#"{"birth":"2004-11-27 22:00","gender":"m"}"#),
        &tool_call(
            "eon_saju",
            r#"{"birth":"2004-11-27 22:00","format":"json"}"#,
        ),
    ]);
    assert!(
        text_of(&r[0]).contains("Four Pillars"),
        "{}",
        text_of(&r[0])
    );
    assert!(r[0]["result"]["isError"].is_null());
    let json: Value = serde_json::from_str(text_of(&r[1])).expect("json format is valid JSON");
    assert!(json["report"]["pillars"].is_object());
}

#[test]
fn compatibility_tool_runs_a_single_system_and_all() {
    let r = exchange(&[
        &tool_call(
            "eon_compatibility",
            r#"{"birth":"2004-11-27 22:00","partner_birth":"1999-03-08 14:20","system":"vedic"}"#,
        ),
        &tool_call(
            "eon_compatibility",
            r#"{"birth":"2004-11-27 22:00","partner_birth":"1999-03-08 14:20"}"#,
        ),
    ]);
    assert!(text_of(&r[0]).contains("/ 36"), "{}", text_of(&r[0]));
    let all = text_of(&r[1]);
    for section in ["Saju", "Vedic", "Western", "Human Design"] {
        assert!(all.contains(section), "missing {section} in:\n{all}");
    }
}

#[test]
fn compatibility_tool_supports_zwds_and_zwds_tool_reports_the_new_data() {
    let r = exchange(&[
        &tool_call(
            "eon_compatibility",
            r#"{"birth":"2004-11-27 22:00","partner_birth":"1999-03-08 14:20","system":"zwds","format":"json"}"#,
        ),
        &tool_call(
            "eon_zwds",
            r#"{"birth":"2004-11-27 22:00","gender":"m","target_year":2026,"format":"json"}"#,
        ),
    ]);
    let compat: Value = serde_json::from_str(text_of(&r[0])).expect("json");
    assert!(
        compat["zwds"]["result"]["harmonyScore"].is_number()
            || compat["zwds"]["result"]["harmony_score"].is_number(),
        "{compat}"
    );
    let zwds: Value = serde_json::from_str(text_of(&r[1])).expect("json");
    assert!(
        zwds["minorLimit"].is_object() || zwds["minor_limit"].is_object(),
        "{zwds}"
    );
    let palaces = zwds["chart"]["palaces"].as_array().unwrap();
    assert!(palaces.iter().all(|p| p["twelve_gods"].is_object()));
}

#[test]
fn compatibility_skips_time_dependent_systems_without_a_birth_time() {
    let r = exchange(&[&tool_call(
        "eon_compatibility",
        r#"{"birth":"2004-11-27","partner_birth":"1999-03-08"}"#,
    )]);
    let text = text_of(&r[0]);
    assert!(r[0]["result"]["isError"].is_null(), "{text}");
    assert!(text.contains("skipped"), "{text}");
    assert!(
        text.contains("Saju"),
        "saju does not need the hour:\n{text}"
    );
}

#[test]
fn numerology_tool_supports_system_target_date_and_compatibility() {
    let r = exchange(&[
        &tool_call(
            "eon_numerology",
            r#"{"birth":"1994-04-04","name":"John Doe","system":"chaldean","target_date":"2026-11-20"}"#,
        ),
        &tool_call(
            "eon_numerology",
            r#"{"birth":"1990-05-28","target_date":"2026-11-20","format":"json"}"#,
        ),
        &tool_call(
            "eon_numerology",
            r#"{"birth":"1990-05-28","system":"bogus"}"#,
        ),
        &tool_call(
            "eon_compatibility",
            r#"{"birth":"1990-05-15","name":"Dave","partner_birth":"1994-04-04","partner_name":"John Doe","system":"numerology"}"#,
        ),
    ]);
    let text = text_of(&r[0]);
    // Hand-computed: Chaldean JOHN DOE = 34 -> 7; life path 13/4 karmic debt; PY 9, PM 2, PD 4.
    assert!(text.contains("== Chaldean Numerology =="), "{text}");
    assert!(text.contains("Expression Number: 7"), "{text}");
    assert!(text.contains("13/4 in lifePath"), "{text}");
    assert!(text.contains("Personal Month Vibration: 2"), "{text}");
    assert!(text.contains("Personal Day Vibration: 4"), "{text}");

    let json: Value = serde_json::from_str(text_of(&r[1])).unwrap();
    assert_eq!(json["result"]["personalYear"], 7);
    assert_eq!(json["result"]["personalMonth"], 9);
    assert_eq!(json["result"]["personalDay"], 2);
    assert_eq!(json["result"]["system"], "pythagorean");

    assert_eq!(r[2]["result"]["isError"], true);
    assert!(text_of(&r[2]).contains("pythagorean or chaldean"));

    let compat = text_of(&r[3]);
    assert!(compat.contains("Numerology (heuristic)"), "{compat}");
    assert!(compat.contains("Total: 57.0 / 100"), "{compat}");
}

#[test]
fn engine_tool_argument_errors_are_tool_errors() {
    let r = exchange(&[
        &tool_call(
            "eon_western",
            r#"{"birth":"2004-11-27 22:00","house_system":"Bogus"}"#,
        ),
        &tool_call("eon_compatibility", r#"{"birth":"2004-11-27 22:00"}"#),
        &tool_call(
            "eon_report",
            r#"{"birth":"2004-11-27 22:00","theme":"fame"}"#,
        ),
        &tool_call(
            "eon_human_design_penta",
            r#"{"members":["1990-05-15 10:30"]}"#,
        ),
        &tool_call(
            "eon_human_design",
            r#"{"birth":"2004-11-27 22:00","return":"solar"}"#,
        ),
        &tool_call("eon_saju", r#"{"birth":"2004-11-27 22:00","format":"xml"}"#),
    ]);
    assert_eq!(r.len(), 6);
    for resp in &r {
        assert!(resp.get("error").is_none(), "protocol error: {resp}");
        assert_eq!(resp["result"]["isError"], true, "{resp}");
    }
    assert!(text_of(&r[0]).contains("house system"));
    assert!(text_of(&r[1]).contains("partner_birth"));
    assert!(text_of(&r[4]).contains("--year"), "{}", text_of(&r[4]));
}

#[test]
fn western_progression_and_hd_overlays_work_through_mcp() {
    let r = exchange(&[
        &tool_call(
            "eon_western",
            r#"{"birth":"2004-11-27 22:00","progress_to":"2027-03-01"}"#,
        ),
        &tool_call(
            "eon_human_design",
            r#"{"birth":"2004-11-27 22:00","transit_at":"2027-03-01 10:00"}"#,
        ),
        &tool_call(
            "eon_human_design",
            r#"{"birth":"2004-11-27 22:00","return":"solar","year":2027}"#,
        ),
        &tool_call(
            "eon_human_design_penta",
            r#"{"members":["1990-05-15 10:30","1988-01-02 08:00@Busan","1995-09-30 21:15"]}"#,
        ),
    ]);
    assert!(
        text_of(&r[0]).contains("Secondary Progression"),
        "{}",
        text_of(&r[0])
    );
    assert!(text_of(&r[1]).contains("Transit"), "{}", text_of(&r[1]));
    assert!(
        text_of(&r[2]).contains("Solar Return"),
        "{}",
        text_of(&r[2])
    );
    assert!(
        text_of(&r[3]).contains("Penta (3 members)"),
        "{}",
        text_of(&r[3])
    );
}

#[test]
fn human_design_exposes_circuitry_and_lunar_authority_through_mcp() {
    let r = exchange(&[
        &tool_call(
            "eon_human_design",
            r#"{"birth":"1990-05-15 10:30","format":"text"}"#,
        ),
        &tool_call(
            "eon_human_design",
            r#"{"birth":"2000-02-23 16:00","tz":"UTC","format":"text"}"#,
        ),
        &tool_call(
            "eon_human_design",
            r#"{"birth":"1990-05-15 10:30","format":"json"}"#,
        ),
    ]);
    let natal = text_of(&r[0]);
    assert!(natal.contains("Circuit Groups:"), "{natal}");
    assert!(natal.contains("Sensing"), "{natal}");
    let reflector = text_of(&r[1]);
    assert!(reflector.contains("Authority: Lunar"), "{reflector}");
    let json = text_of(&r[2]);
    assert!(json.contains("\"circuitry\""), "{json}");
    assert!(json.contains("\"authority_ko\""), "{json}");
}

#[test]
fn western_timing_techniques_work_through_mcp() {
    let r = exchange(&[
        &tool_call(
            "eon_western",
            r#"{"birth":"2004-11-27 22:00","transit_at":"2026-11-20"}"#,
        ),
        &tool_call(
            "eon_western",
            r#"{"birth":"2004-11-27 22:00","solar_return":2027,"return_city":"New York"}"#,
        ),
        &tool_call(
            "eon_western",
            r#"{"birth":"2004-11-27 22:00","lunar_return_after":"2026-11-20"}"#,
        ),
        &tool_call(
            "eon_western",
            r#"{"birth":"2004-11-27 22:00","solar_arc_to":"2027-03-01"}"#,
        ),
        &tool_call(
            "eon_western",
            r#"{"birth":"2004-11-27 22:00","profection_at":"2026-11-20"}"#,
        ),
    ]);
    assert!(text_of(&r[0]).contains("Transits at"), "{}", text_of(&r[0]));
    assert!(
        text_of(&r[1]).contains("Solar Return 2027"),
        "{}",
        text_of(&r[1])
    );
    assert!(
        text_of(&r[2]).contains("Lunar Return after"),
        "{}",
        text_of(&r[2])
    );
    assert!(text_of(&r[3]).contains("Solar Arc"), "{}", text_of(&r[3]));
    assert!(
        text_of(&r[4]).contains("Lord of the year"),
        "{}",
        text_of(&r[4])
    );
}

#[test]
fn western_techniques_are_mutually_exclusive_and_validated_through_mcp() {
    let r = exchange(&[
        &tool_call(
            "eon_western",
            r#"{"birth":"2004-11-27 22:00","transit_at":"2026-11-20","solar_return":2027}"#,
        ),
        &tool_call(
            "eon_western",
            r#"{"birth":"2004-11-27 22:00","solar_return":2004}"#,
        ),
        &tool_call(
            "eon_western",
            r#"{"birth":"2004-11-27 22:00","profection_at":"1990-01-01"}"#,
        ),
        &tool_call(
            "eon_western",
            r#"{"birth":"2004-11-27 22:00","solar_return":2027,"return_city":"Atlantis"}"#,
        ),
    ]);
    for resp in &r {
        assert!(resp.get("error").is_none(), "protocol error: {resp}");
        assert_eq!(resp["result"]["isError"], true, "{resp}");
    }
    assert!(text_of(&r[0]).contains("only one"), "{}", text_of(&r[0]));
}
