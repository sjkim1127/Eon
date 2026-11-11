use anyhow::Result;
use clap::Args;
use eon_service::dto::*;
use eon_service::facade;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};

use crate::format::agent::format_agent_context;

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
                            "version": "7.0.0"
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
                                            "description": "Birth datetime in 'YYYY-MM-DD HH:MM' format (e.g. '1990-05-15 10:30')"
                                        },
                                        "gender": {
                                            "type": "string",
                                            "enum": ["m", "f"],
                                            "description": "Gender: 'm' for male, 'f' for female",
                                            "default": "m"
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
                                            "description": "Output format: 'agent' for LLM prompt context, 'json' for raw data",
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

                    let birth_args = crate::commands::BirthArgs {
                        birth: birth_str.to_string(),
                        gender: gender_str.to_string(),
                        city: city_str,
                        lat: lat_val,
                        lon: lon_val,
                        tz: tz_str,
                        lunar: false,
                        leap: false,
                        name: name_str,
                    };

                    match birth_args.to_analysis_input() {
                        Ok(input) => {
                            let is_male = birth_args.is_male();
                            let saju_in =
                                SajuAnalysisInput::new(input.clone(), is_male, false, Some(false));
                            let vedic_in =
                                VedicAnalysisInput::new(input.clone(), Some(false), None);
                            let western_in =
                                WesternAnalysisInput::new(input.clone(), "Placidus".to_string());
                            let zwds_in = ZwdsAnalysisInput::new(input.clone(), is_male, None);
                            let hd_in = HumanDesignAnalysisInput::new(input.clone());
                            let qimen_in = QimenAnalysisInput::new(input.clone(), is_male);
                            let num_in = NumerologyAnalysisInput::new(
                                input.clone(),
                                birth_args.name.clone(),
                                None,
                            );

                            let saju_res = facade::analyze_saju(saju_in).ok();
                            let vedic_res = facade::analyze_vedic(vedic_in).ok();
                            let western_res = facade::analyze_western(western_in).ok();
                            let zwds_res = facade::analyze_zwds(zwds_in).ok();
                            let hd_res = facade::analyze_human_design(hd_in).ok();
                            let qimen_res = facade::analyze_qimen(qimen_in).ok();
                            let num_res = facade::analyze_numerology(num_in).ok();

                            let tier_res = if let (Some(s), Some(v)) = (&saju_res, &vedic_res) {
                                let omni_input = OmniDestinyTierInput {
                                    saju: s.clone(),
                                    vedic: v.clone(),
                                    western: western_res.clone(),
                                    zwds: zwds_res.clone(),
                                    human_design: hd_res.clone(),
                                    qimen: qimen_res.clone(),
                                    numerology: num_res.clone(),
                                    transit: None,
                                };
                                facade::analyze_destiny_tier_omni(omni_input).ok()
                            } else {
                                None
                            };

                            let content_text = if format_str == "json" {
                                json!({
                                    "saju": saju_res,
                                    "vedic": vedic_res,
                                    "western": western_res,
                                    "zwds": zwds_res,
                                    "human_design": hd_res,
                                    "qimen": qimen_res,
                                    "numerology": num_res,
                                    "tier": tier_res,
                                })
                                .to_string()
                            } else {
                                format_agent_context(
                                    &input,
                                    saju_res.as_ref(),
                                    vedic_res.as_ref(),
                                    western_res.as_ref(),
                                    zwds_res.as_ref(),
                                    hd_res.as_ref(),
                                    qimen_res.as_ref(),
                                    num_res.as_ref(),
                                    tier_res.as_ref(),
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
                        Err(e) => {
                            let res = JsonRpcResponse {
                                jsonrpc: "2.0",
                                id: req_id,
                                result: None,
                                error: Some(json!({
                                    "code": -32602,
                                    "message": format!("Invalid arguments: {}", e)
                                })),
                            };
                            writeln!(stdout, "{}", serde_json::to_string(&res)?)?;
                        }
                    }
                } else {
                    let res = JsonRpcResponse {
                        jsonrpc: "2.0",
                        id: req_id,
                        result: None,
                        error: Some(json!({
                            "code": -32601,
                            "message": format!("Tool not found: {}", tool_name)
                        })),
                    };
                    writeln!(stdout, "{}", serde_json::to_string(&res)?)?;
                }
                stdout.flush()?;
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
