//! MCP tools that wrap the CLI commands one-to-one.
//!
//! Each tool builds the same command struct the CLI parses from flags, runs its `execute`
//! and returns the text summary (default) or the full JSON. `eon_analyze` (the all-engine
//! synthesis) lives in `mcp.rs`.

use anyhow::{anyhow, Result};
use serde_json::{json, Map, Value};

use super::compat::{CompatCommand, CompatSystem};
use super::hd::{HdCommand, HdReturn};
use super::iching::IchingCommand;
use super::numerology::{NumerologyCommand, NumerologySystemArg};
use super::penta::PentaCommand;
use super::qimen::QimenCommand;
use super::report::{ReportCommand, ThemeArg};
use super::saju::SajuCommand;
use super::transit::TransitCommand;
use super::vedic::VedicCommand;
use super::western::WesternCommand;
use super::zwds::ZwdsCommand;
use super::{BirthArgs, EngineFormat, PartnerArgs, Rendered};

/// Typed access to the `arguments` object of a `tools/call` request.
struct Args<'a>(&'a Value);

impl Args<'_> {
    fn str(&self, key: &str) -> Option<String> {
        self.0
            .get(key)
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_string)
    }

    fn require(&self, key: &str) -> Result<String> {
        self.str(key)
            .ok_or_else(|| anyhow!("missing required argument '{key}'"))
    }

    fn bool(&self, key: &str) -> bool {
        self.0.get(key).and_then(Value::as_bool).unwrap_or(false)
    }

    fn f64(&self, key: &str) -> Option<f64> {
        self.0.get(key).and_then(Value::as_f64)
    }

    fn i32(&self, key: &str) -> Result<Option<i32>> {
        match self.0.get(key).and_then(Value::as_i64) {
            Some(v) => Ok(Some(
                i32::try_from(v).map_err(|_| anyhow!("'{key}' is out of range"))?,
            )),
            None => Ok(None),
        }
    }

    /// Birth fields of one person; `prefix` is "" or "partner_".
    fn birth(&self, prefix: &str, default_gender: &str) -> Result<BirthArgs> {
        let k = |name: &str| format!("{prefix}{name}");
        Ok(BirthArgs {
            birth: self.require(&k("birth"))?,
            gender: self
                .str(&k("gender"))
                .unwrap_or_else(|| default_gender.to_string()),
            city: self.str(&k("city")),
            lat: self.f64(&k("lat")),
            lon: self.f64(&k("lon")),
            tz: self.str(&k("timezone")).or_else(|| self.str(&k("tz"))),
            lunar: self.bool(&k("lunar")),
            leap: self.bool(&k("leap")),
            name: self.str(&k("name")),
        })
    }

    fn partner(&self) -> Result<PartnerArgs> {
        let b = self.birth("partner_", "f")?;
        Ok(PartnerArgs {
            birth: b.birth,
            gender: b.gender,
            city: b.city,
            lat: b.lat,
            lon: b.lon,
            tz: b.tz,
            lunar: b.lunar,
            leap: b.leap,
            name: b.name,
        })
    }

    fn format(&self) -> Result<EngineFormat> {
        match self.str("format").as_deref() {
            None | Some("text") => Ok(EngineFormat::Text),
            Some("json") => Ok(EngineFormat::Json),
            Some(other) => Err(anyhow!(
                "Invalid format '{other}'. Expected 'text' or 'json'"
            )),
        }
    }
}

// ───────────────────────────── schema helpers ─────────────────────────────

fn birth_props(prefix: &str, who: &str, with_name: bool) -> Map<String, Value> {
    let mut m = Map::new();
    let mut put = |name: &str, schema: Value| {
        m.insert(format!("{prefix}{name}"), schema);
    };
    put(
        "birth",
        json!({
            "type": "string",
            "description": format!("{who} birth date and time as 'YYYY-MM-DD HH:MM' (e.g. '1990-05-15 10:30'). A date alone means the birth time is unknown, which the time-dependent engines (Vedic, Western, Zi Wei Dou Shu, Human Design, QiMen) reject.")
        }),
    );
    put(
        "gender",
        json!({ "type": "string", "enum": ["m", "f"], "description": format!("{who} gender") }),
    );
    put(
        "lunar",
        json!({ "type": "boolean", "description": "True if the birth date is a lunar calendar date" }),
    );
    put(
        "leap",
        json!({ "type": "boolean", "description": "True if the lunar birth month is a leap month" }),
    );
    put(
        "city",
        json!({ "type": "string", "description": "Birth city (resolves coordinates and timezone), e.g. 'Seoul', 'Tokyo', 'New York'" }),
    );
    put(
        "lat",
        json!({ "type": "number", "description": "Latitude (-90 to 90)" }),
    );
    put(
        "lon",
        json!({ "type": "number", "description": "Longitude (-180 to 180)" }),
    );
    put(
        "timezone",
        json!({ "type": "string", "description": "IANA timezone, e.g. 'Asia/Seoul'" }),
    );
    if with_name {
        m.insert(
            "name".to_string(),
            json!({ "type": "string", "description": "Latin-letter name (Numerology Expression/Soul Urge, report title)" }),
        );
    }
    m
}

fn format_prop() -> Value {
    json!({
        "type": "string",
        "enum": ["text", "json"],
        "default": "text",
        "description": "'text' for a compact summary, 'json' for the full structured result (large)"
    })
}

fn tool(name: &str, description: &str, props: Map<String, Value>, required: &[&str]) -> Value {
    let mut props = props;
    props.insert("format".to_string(), format_prop());
    json!({
        "name": name,
        "description": description,
        "inputSchema": { "type": "object", "properties": props, "required": required }
    })
}

fn with(mut base: Map<String, Value>, extra: Vec<(&str, Value)>) -> Map<String, Value> {
    for (k, v) in extra {
        base.insert(k.to_string(), v);
    }
    base
}

/// Tool definitions for `tools/list` (everything except `eon_analyze`).
pub fn definitions() -> Vec<Value> {
    let single = || birth_props("", "Person's", true);
    let partnered = || {
        let mut m = birth_props("", "First person's", true);
        m.extend(birth_props("partner_", "Partner's", true));
        m
    };

    vec![
        tool(
            "eon_saju",
            "Four Pillars of Destiny (Saju/BaZi): pillars, day master strength, structure, yongshin, major luck, golden period.",
            with(single(), vec![("night_rat", json!({ "type": "boolean", "description": "Split the Zi hour at midnight (night rat hour)" }))]),
            &["birth"],
        ),
        tool(
            "eon_saju_luck",
            "Saju luck cycles (yearly, monthly, daily, hourly) at a moment, with ten gods, special events and load diagnostics around the current age.",
            with(single(), vec![
                ("at", json!({ "type": "string", "description": "Moment to evaluate: 'YYYY-MM-DD[ HH:MM]' in the birth timezone, or RFC 3339 (default: now)" })),
                ("night_rat", json!({ "type": "boolean" })),
            ]),
            &["birth"],
        ),
        tool(
            "eon_report",
            "Themed Saju report in markdown: wealth & career, love & marriage, or health & vitality.",
            with(single(), vec![("theme", json!({ "type": "string", "enum": ["wealth", "love", "health"] }))]),
            &["birth", "theme"],
        ),
        tool(
            "eon_vedic",
            "Vedic astrology (Jyotish): sidereal chart, nakshatra, dashas, yogas, Shadbala, Ashtakavarga, Bhava Bala, gochara, Tajika annual chart, KP.",
            with(single(), vec![
                ("ayanamsa", json!({ "type": "string", "enum": ["Lahiri", "FaganBradley", "Raman", "Krishnamurti"], "description": "Default Lahiri" })),
                ("target_year", json!({ "type": "integer", "description": "Year of the annual (Varshaphala) chart; default the current year" })),
                ("year_type", json!({ "type": "string", "enum": ["Gregorian", "Savana", "Sidereal"] })),
            ]),
            &["birth"],
        ),
        tool(
            "eon_western",
            "Western astrology: planets, houses, aspects, patterns, dignities, antiscia, mutual receptions, midpoints. Optionally one timing technique: secondary progression, transits, solar arc, annual profection, solar return or lunar return (pass at most one).",
            with(single(), vec![
                ("house_system", json!({ "type": "string", "enum": ["Placidus", "Koch", "WholeSign", "Equal", "Porphyry", "Regiomontanus", "Campanus", "Alcabitius", "Morinus", "Topocentric"], "description": "Default Placidus" })),
                ("progress_to", json!({ "type": "string", "description": "Secondary progression to this date: 'YYYY-MM-DD[ HH:MM]' in the birth timezone, or RFC 3339" })),
                ("transit_at", json!({ "type": "string", "description": "Transits: the sky at this date over the natal chart (same date format)" })),
                ("solar_arc_to", json!({ "type": "string", "description": "Solar arc directions to this date" })),
                ("profection_at", json!({ "type": "string", "description": "Annual profection (house and lord of the year) at this date" })),
                ("solar_return", json!({ "type": "integer", "description": "Solar return of this calendar year (after the birth year)" })),
                ("lunar_return_after", json!({ "type": "string", "description": "First lunar return on or after this date" })),
                ("return_city", json!({ "type": "string", "description": "Cast the return chart for this city (relocated return) instead of the birthplace" })),
            ]),
            &["birth"],
        ),
        tool(
            "eon_zwds",
            "Zi Wei Dou Shu (Purple Star): 12 palaces, stars, four transformations, current decade, annual palace, destiny patterns.",
            with(single(), vec![("target_year", json!({ "type": "integer", "description": "Year for the annual palace; default the current year" }))]),
            &["birth"],
        ),
        tool(
            "eon_human_design",
            "Human Design: type, strategy, authority, profile, centers, channels, incarnation cross; optionally overlay a transit date or a planetary return.",
            with(single(), vec![
                ("transit_at", json!({ "type": "string", "description": "Overlay the sky of this date on the natal chart: 'YYYY-MM-DD[ HH:MM]' in the birth timezone, or RFC 3339" })),
                ("return", json!({ "type": "string", "enum": ["solar", "saturn", "chiron"], "description": "Planetary return chart overlaid on the natal chart" })),
                ("year", json!({ "type": "integer", "description": "Year of the solar return (required for return=solar)" })),
            ]),
            &["birth"],
        ),
        tool(
            "eon_human_design_penta",
            "Human Design Penta (group dynamics) for 3 to 5 people.",
            {
                let mut m = Map::new();
                m.insert("members".to_string(), json!({
                    "type": "array", "minItems": 3, "maxItems": 5,
                    "items": { "type": "string" },
                    "description": "Each member as 'YYYY-MM-DD HH:MM' or 'YYYY-MM-DD HH:MM@city'"
                }));
                m.insert("city".to_string(), json!({ "type": "string", "description": "Default city for members without '@city'" }));
                m
            },
            &["members"],
        ),
        tool(
            "eon_qimen",
            "QiMen DunJia birth chart: Yin/Yang Dun, Ju number, chief star, envoy door, nine palaces.",
            single(),
            &["birth"],
        ),
        tool(
            "eon_numerology",
            "Numerology (Pythagorean or Chaldean letters): life path, birthday, expression/soul urge/personality/maturity, hidden passion, balance, cornerstone/capstone, missing numbers (with a Latin name), karmic debts (13/14/16/19), pinnacles, personal year/month/day.",
            with(single(), vec![
                ("target_year", json!({ "type": "integer", "description": "Year for the Personal Year number (no Personal Month/Day); default today" })),
                ("target_date", json!({ "type": "string", "description": "'YYYY-MM-DD' date for the Personal Year, Month and Day; default today. Takes precedence over target_year" })),
                ("system", json!({ "type": "string", "enum": ["pythagorean", "chaldean"], "default": "pythagorean", "description": "Letter-to-number system for the name numbers" })),
            ]),
            &["birth"],
        ),
        tool(
            "eon_iching",
            "I Ching / HeLuo hexagrams: pre-natal, post-natal, governing line and lifetime cycles.",
            single(),
            &["birth"],
        ),
        tool(
            "eon_compatibility",
            "Two-person compatibility across Saju (stems/branches), Vedic (Ashtakoota 36 points, Mangal dosha), Western (synastry + composite) and Human Design (connection chart); system 'numerology' adds a heuristic Life Path / Expression / Soul Urge pair score (give name and partner_name for the name pairs; not part of 'all'). Systems that cannot run (e.g. unknown birth time) are reported as skipped.",
            with(partnered(), vec![
                ("system", json!({ "type": "string", "enum": ["all", "saju", "vedic", "western", "hd", "numerology"], "default": "all" })),
                ("house_system", json!({ "type": "string", "description": "House system for the Western charts (default Placidus)" })),
            ]),
            &["birth", "partner_birth"],
        ),
    ]
}

fn render(r: Rendered, format: EngineFormat) -> Result<String> {
    r.render(format)
}

/// Runs a tool. `None` means the tool name is not one of these tools.
pub fn call(name: &str, args: &Value) -> Option<Result<String, String>> {
    let a = Args(args);
    let result: Result<String> = match name {
        "eon_saju" => (|| {
            let format = a.format()?;
            let cmd = SajuCommand {
                birth: a.birth("", "m")?,
                night_rat: a.bool("night_rat"),
                format,
            };
            render(super::saju::execute(&cmd)?, format)
        })(),
        "eon_saju_luck" => (|| {
            let format = a.format()?;
            let cmd = TransitCommand {
                birth: a.birth("", "m")?,
                at: a.str("at"),
                night_rat: a.bool("night_rat"),
                format,
            };
            render(super::transit::execute(&cmd)?, format)
        })(),
        "eon_report" => (|| {
            let format = a.format()?;
            let theme = match a.require("theme")?.as_str() {
                "wealth" => ThemeArg::Wealth,
                "love" => ThemeArg::Love,
                "health" => ThemeArg::Health,
                other => {
                    return Err(anyhow!(
                        "Invalid theme '{other}'. Expected wealth, love or health"
                    ))
                }
            };
            let cmd = ReportCommand {
                birth: a.birth("", "m")?,
                theme,
                format,
            };
            render(super::report::execute(&cmd)?, format)
        })(),
        "eon_vedic" => (|| {
            let format = a.format()?;
            let cmd = VedicCommand {
                birth: a.birth("", "m")?,
                ayanamsa: a.str("ayanamsa"),
                target_year: a.i32("target_year")?,
                year_type: a.str("year_type"),
                format,
            };
            render(super::vedic::execute(&cmd)?, format)
        })(),
        "eon_western" => (|| {
            let format = a.format()?;
            let cmd = WesternCommand {
                birth: a.birth("", "m")?,
                house_system: a
                    .str("house_system")
                    .unwrap_or_else(|| "Placidus".to_string()),
                progress_to: a.str("progress_to"),
                transit_at: a.str("transit_at"),
                solar_arc_to: a.str("solar_arc_to"),
                profection_at: a.str("profection_at"),
                solar_return: a.i32("solar_return")?,
                lunar_return_after: a.str("lunar_return_after"),
                return_city: a.str("return_city"),
                format,
            };
            let chosen = [
                cmd.progress_to.is_some(),
                cmd.transit_at.is_some(),
                cmd.solar_arc_to.is_some(),
                cmd.profection_at.is_some(),
                cmd.solar_return.is_some(),
                cmd.lunar_return_after.is_some(),
            ]
            .iter()
            .filter(|set| **set)
            .count();
            if chosen > 1 {
                return Err(anyhow!(
                    "Use only one of progress_to, transit_at, solar_arc_to, profection_at, solar_return, lunar_return_after"
                ));
            }
            render(super::western::execute(&cmd)?, format)
        })(),
        "eon_zwds" => (|| {
            let format = a.format()?;
            let cmd = ZwdsCommand {
                birth: a.birth("", "m")?,
                target_year: a.i32("target_year")?,
                format,
            };
            render(super::zwds::execute(&cmd)?, format)
        })(),
        "eon_human_design" => (|| {
            let format = a.format()?;
            let return_type = match a.str("return").as_deref() {
                None => None,
                Some("solar") => Some(HdReturn::Solar),
                Some("saturn") => Some(HdReturn::Saturn),
                Some("chiron") => Some(HdReturn::Chiron),
                Some(other) => {
                    return Err(anyhow!(
                        "Invalid return '{other}'. Expected solar, saturn or chiron"
                    ))
                }
            };
            let cmd = HdCommand {
                birth: a.birth("", "m")?,
                transit_at: a.str("transit_at"),
                return_type,
                year: a.i32("year")?,
                format,
            };
            if cmd.transit_at.is_some() && cmd.return_type.is_some() {
                return Err(anyhow!("Use either 'transit_at' or 'return', not both"));
            }
            render(super::hd::execute(&cmd)?, format)
        })(),
        "eon_human_design_penta" => (|| {
            let format = a.format()?;
            let members = args
                .get("members")
                .and_then(Value::as_array)
                .ok_or_else(|| anyhow!("missing required argument 'members'"))?
                .iter()
                .map(|v| {
                    v.as_str()
                        .map(str::to_string)
                        .ok_or_else(|| anyhow!("'members' must be an array of strings"))
                })
                .collect::<Result<Vec<_>>>()?;
            let cmd = PentaCommand {
                members,
                city: a.str("city"),
                format,
            };
            render(super::penta::execute(&cmd)?, format)
        })(),
        "eon_qimen" => (|| {
            let format = a.format()?;
            let cmd = QimenCommand {
                birth: a.birth("", "m")?,
                format,
            };
            render(super::qimen::execute(&cmd)?, format)
        })(),
        "eon_numerology" => (|| {
            let format = a.format()?;
            let target_year = match a.i32("target_year")? {
                Some(y) => {
                    Some(u32::try_from(y).map_err(|_| anyhow!("'target_year' must be positive"))?)
                }
                None => None,
            };
            let system = match a.str("system").as_deref() {
                None | Some("pythagorean") => NumerologySystemArg::Pythagorean,
                Some("chaldean") => NumerologySystemArg::Chaldean,
                Some(other) => {
                    return Err(anyhow!(
                        "Invalid system '{other}'. Expected pythagorean or chaldean"
                    ))
                }
            };
            let target_date = a.str("target_date");
            let cmd = NumerologyCommand {
                birth: a.birth("", "m")?,
                // A target date wins over a bare target year (the CLI flags conflict).
                target_year: if target_date.is_some() {
                    None
                } else {
                    target_year
                },
                target_date,
                system,
                format,
            };
            render(super::numerology::execute(&cmd)?, format)
        })(),
        "eon_iching" => (|| {
            let format = a.format()?;
            let cmd = IchingCommand {
                birth: a.birth("", "m")?,
                format,
            };
            render(super::iching::execute(&cmd)?, format)
        })(),
        "eon_compatibility" => (|| {
            let format = a.format()?;
            let system = match a.str("system").as_deref() {
                None | Some("all") => CompatSystem::All,
                Some("saju") => CompatSystem::Saju,
                Some("vedic") => CompatSystem::Vedic,
                Some("western") => CompatSystem::Western,
                Some("hd") => CompatSystem::Hd,
                Some("numerology") => CompatSystem::Numerology,
                Some(other) => {
                    return Err(anyhow!(
                        "Invalid system '{other}'. Expected all, saju, vedic, western, hd or numerology"
                    ))
                }
            };
            let cmd = CompatCommand {
                birth: a.birth("", "m")?,
                partner: a.partner()?,
                system,
                house_system: a
                    .str("house_system")
                    .unwrap_or_else(|| "Placidus".to_string()),
                format,
            };
            render(super::compat::execute(&cmd)?, format)
        })(),
        _ => return None,
    };

    Some(result.map_err(|e| format!("{e}")))
}
