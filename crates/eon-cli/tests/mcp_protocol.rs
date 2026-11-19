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
