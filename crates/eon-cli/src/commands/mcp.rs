use anyhow::Result;
use clap::Args;
use eon_service::facade;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};

use crate::format::agent::format_agent_context;
use crate::format::markdown::format_markdown_report;

#[derive(Args, Debug)]
pub struct McpCommand {}

#[derive(Deserialize)]
struct JsonRpcRequest {
    #[allow(dead_code)]
    jsonrpc: Option<String>,
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

#[derive(Serialize)]
struct JsonRpcResponse {
    jsonrpc: &'static str,
    id: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<Value>,
}

/// A tool-execution failure. MCP reports these inside the tool result (`isError: true`) so the
/// model can read the message and correct its call, instead of as a protocol-level error.
fn tool_error(id: Value, message: String) -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: "2.0",
        id,
        result: Some(json!({
            "content": [{ "type": "text", "text": message }],
            "isError": true
        })),
        error: None,
    }
}

pub fn run(_cmd: McpCommand) -> Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let req: JsonRpcRequest = match serde_json::from_str(line) {
            Ok(r) => r,
            Err(e) => {
                let err_res = json!({
                    "jsonrpc": "2.0",
                    "id": null,
                    "error": { "code": -32700, "message": format!("Parse error: {}", e) }
                });
                writeln!(stdout, "{}", serde_json::to_string(&err_res)?)?;
                stdout.flush()?;
                continue;
            }
        };

        let req_id = req.id.clone().unwrap_or(Value::Null);

        match req.method.as_str() {
            "initialize" => {
                let res = JsonRpcResponse {
                    jsonrpc: "2.0",
                    id: req_id,
                    result: Some(json!({
                        "protocolVersion": "2024-11-05",
                        "capabilities": {
                            "tools": {}
                        },
                        "serverInfo": {
                            "name": "eon-destiny-engine",
                            "version": env!("CARGO_PKG_VERSION")
                        }
                    })),
                    error: None,
                };
                writeln!(stdout, "{}", serde_json::to_string(&res)?)?;
                stdout.flush()?;
            }
            "notifications/initialized" => {
                // No-op for notification
            }
            "ping" => {
                let res = JsonRpcResponse {
                    jsonrpc: "2.0",
                    id: req_id,
                    result: Some(json!({})),
                    error: None,
                };
                writeln!(stdout, "{}", serde_json::to_string(&res)?)?;
                stdout.flush()?;
            }
            "tools/list" => {
                let res = JsonRpcResponse {
                    jsonrpc: "2.0",
                    id: req_id,
                    result: Some(json!({
                        "tools": [
                            {
                                "name": "eon_analyze",
                                "description": "Execute comprehensive 8-engine destiny analysis (Saju, Vedic, Western, ZWDS, Human Design, QiMen, Numerology, Destiny Tier 7.0).",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {
                                        "birth": {
                                            "type": "string",
                                            "description": "Birth date and time as 'YYYY-MM-DD HH:MM' (e.g. '1990-05-15 10:30'). A date alone ('1990-05-15') means the birth time is unknown: the Hour pillar and time-dependent engines (Vedic, Western, Zi Wei Dou Shu, Human Design, QiMen) are then not evaluated."
                                        },
                                        "gender": {
                                            "type": "string",
                                            "enum": ["m", "f"],
                                            "description": "Gender: 'm' for male, 'f' for female",
                                            "default": "m"
                                        },
                                        "lunar": {
                                            "type": "boolean",
                                            "description": "True if 'birth' is a lunar calendar date",
                                            "default": false
                                        },
                                        "leap": {
                                            "type": "boolean",
                                            "description": "True if the lunar birth month is a leap month (only with lunar=true)",
                                            "default": false
                                        },
                                        "city": {
                                            "type": "string",
                                            "description": "Birth city or region for automatic coordinate and timezone resolution (e.g. 'Seoul', 'Busan', 'Tokyo', 'New York', 'London', 'LA')"
                                        },
                                        "lat": {
                                            "type": "number",
                                            "description": "Optional latitude (-90.0 to 90.0)"
                                        },
                                        "lon": {
                                            "type": "number",
                                            "description": "Optional longitude (-180.0 to 180.0)"
                                        },
                                        "timezone": {
                                            "type": "string",
                                            "description": "Optional IANA timezone (e.g. 'Asia/Seoul', 'America/New_York', 'Europe/London')"
                                        },
                                        "name": {
                                            "type": "string",
                                            "description": "Optional English name for Numerology (e.g. 'John Doe')"
                                        },
                                        "format": {
                                            "type": "string",
                                            "enum": ["agent", "json", "markdown"],
                                            "description": "Output format: 'agent' for a compact LLM prompt context, 'markdown' for a report with tables, 'json' for the full structured data (very large)",
                                            "default": "agent"
                                        }
                                    },
                                    "required": ["birth"]
                                }
                            }
                        ]
                    })),
                    error: None,
                };
                writeln!(stdout, "{}", serde_json::to_string(&res)?)?;
                stdout.flush()?;
            }
            "tools/call" => {
                let params = req.params.unwrap_or(Value::Null);
                let tool_name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
                let args = params.get("arguments").cloned().unwrap_or(json!({}));

                if tool_name == "eon_analyze" {
                    let birth_str = args.get("birth").and_then(|v| v.as_str()).unwrap_or("");
                    let gender_str = args.get("gender").and_then(|v| v.as_str()).unwrap_or("m");
                    let name_str = args
                        .get("name")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    let format_str = args
                        .get("format")
                        .and_then(|v| v.as_str())
                        .unwrap_or("agent");

                    let city_str = args
                        .get("city")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    let lat_val = args.get("lat").and_then(|v| v.as_f64());
                    let lon_val = args.get("lon").and_then(|v| v.as_f64());
                    let tz_str = args
                        .get("timezone")
                        .or_else(|| args.get("tz"))
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());

                    let lunar_val = args.get("lunar").and_then(|v| v.as_bool()).unwrap_or(false);
                    let leap_val = args.get("leap").and_then(|v| v.as_bool()).unwrap_or(false);

                    if !matches!(format_str, "agent" | "json" | "markdown") {
                        let res = tool_error(
                            req_id,
                            format!(
                                "Invalid format: '{}'. Expected 'agent', 'json' or 'markdown'",
                                format_str
                            ),
                        );
                        writeln!(stdout, "{}", serde_json::to_string(&res)?)?;
                        stdout.flush()?;
                        continue;
                    }

                    let birth_args = crate::commands::BirthArgs {
                        birth: birth_str.to_string(),
                        gender: gender_str.to_string(),
                        city: city_str,
                        lat: lat_val,
                        lon: lon_val,
                        tz: tz_str,
                        lunar: lunar_val,
                        leap: leap_val,
                        name: name_str,
                    };

                    match birth_args.to_analysis_input() {
                        Ok(input) => {
                            let is_male = birth_args.is_male().unwrap_or(true);
                            match facade::analyze_all_synthesized(
                                input,
                                is_male,
                                birth_args.name.clone(),
                            ) {
                                Ok(omni) => {
                                    let content_text = if format_str == "json" {
                                        serde_json::to_string_pretty(&omni)?
                                    } else if format_str == "markdown" {
                                        format_markdown_report(
                                            &omni.input,
                                            Some(&omni.saju),
                                            omni.vedic.as_ref(),
                                            omni.western.as_ref(),
                                            omni.zwds.as_ref(),
                                            omni.human_design.as_ref(),
                                            omni.qimen.as_ref(),
                                            omni.numerology.as_ref(),
                                            Some(&omni.tier),
                                            Some(&omni.handbook),
                                        )
                                    } else {
                                        format_agent_context(
                                            &omni.input,
                                            Some(&omni.saju),
                                            omni.vedic.as_ref(),
                                            omni.western.as_ref(),
                                            omni.zwds.as_ref(),
                                            omni.human_design.as_ref(),
                                            omni.qimen.as_ref(),
                                            omni.numerology.as_ref(),
                                            Some(&omni.tier),
                                            Some(&omni.handbook),
                                        )
                                    };

                                    let res = JsonRpcResponse {
                                        jsonrpc: "2.0",
                                        id: req_id,
                                        result: Some(json!({
                                            "content": [
                                                {
                                                    "type": "text",
                                                    "text": content_text
                                                }
                                            ]
                                        })),
                                        error: None,
                                    };
                                    writeln!(stdout, "{}", serde_json::to_string(&res)?)?;
                                }
                                Err(err) => {
                                    let res =
                                        tool_error(req_id, format!("Analysis failed: {}", err));
                                    writeln!(stdout, "{}", serde_json::to_string(&res)?)?;
                                }
                            }
                        }
                        Err(e) => {
                            let res = tool_error(req_id, format!("Invalid arguments: {}", e));
                            writeln!(stdout, "{}", serde_json::to_string(&res)?)?;
                        }
                    }
                } else {
                    let res = JsonRpcResponse {
                        jsonrpc: "2.0",
                        id: req_id,
                        result: None,
                        error: Some(json!({
                            "code": -32602,
                            "message": format!("Unknown tool: {}", tool_name)
                        })),
                    };
                    writeln!(stdout, "{}", serde_json::to_string(&res)?)?;
                }
                stdout.flush()?;
            }
            _ if req.id.is_none() => {
                // Unknown notification: JSON-RPC forbids replying to notifications.
            }
            _ => {
                let res = JsonRpcResponse {
                    jsonrpc: "2.0",
                    id: req_id,
                    result: None,
                    error: Some(json!({
                        "code": -32601,
                        "message": format!("Method not found: {}", req.method)
                    })),
                };
                writeln!(stdout, "{}", serde_json::to_string(&res)?)?;
                stdout.flush()?;
            }
        }
    }

    Ok(())
}
