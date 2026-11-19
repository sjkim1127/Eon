use anyhow::Result;
use clap::{Parser, Subcommand};

mod commands;
mod format;

use commands::*;

#[derive(Parser, Debug)]
#[command(
    name = "eon",
    author = "SJKim <sjkim1127@github.com>",
    version = "0.1.2",
    about = "Eon: The Destiny Reversing Intelligence Engine (Native CLI & AI Agent Interface)",
    long_about = "High-performance native CLI that analyzes destiny across 8 ancient & modern metaphysical systems:\n\
                  - Saju (Four Pillars of Destiny)\n\
                  - Vedic Astrology (Jyotish)\n\
                  - Western Astrology\n\
                  - Zi Wei Dou Shu (Purple Star Astrology)\n\
                  - Human Design\n\
                  - QiMen DunJia\n\
                  - Pythagorean Numerology\n\
                  - Destiny Tier 7.0 (8-Engine Omni Synthesis)\n\n\
                  Built specifically for AI Agents, LLM Context Prompts, and Metaphysical Engineers."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Comprehensive 8-engine destiny analysis (Saju + Vedic + Western + ZWDS + HD + QiMen + Numerology + Tier 7.0)
    Analyze(analyze::AnalyzeCommand),

    /// Generate an optimized LLM system prompt / context block for AI Agents
    Prompt(prompt::PromptCommand),

    /// Saju (Four Pillars of Destiny) analysis
    Saju(saju::SajuCommand),

    /// Vedic Astrology (Jyotish) analysis
    Vedic(vedic::VedicCommand),

    /// Western Astrology (Big 3, Houses, Aspects) analysis
    Western(western::WesternCommand),

    /// Zi Wei Dou Shu (12 Palaces, Stars, Patterns) analysis
    Zwds(zwds::ZwdsCommand),

    /// Human Design (Type, Authority, Centers, Channels) analysis
    Hd(hd::HdCommand),

    /// QiMen DunJia (9 Palaces, 8 Doors, 9 Stars, 8 Deities) analysis
    Qimen(qimen::QimenCommand),

    /// I Ching / HeLuo Destiny Hexagrams (Pre-natal, Post-natal, Lifetime Cycles) analysis
    Iching(iching::IchingCommand),

    /// Pythagorean Numerology (5 Core Numbers, Pinnacles, Personal Year) analysis
    Numerology(numerology::NumerologyCommand),

    /// Destiny Tier 7.0 (15 weighted components & 8D Quantum Synergies)
    Tier(tier::TierCommand),

    /// Run Model Context Protocol (MCP) stdio server for Claude, Cursor, and AI Agents
    Mcp(mcp::McpCommand),
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze(cmd) => analyze::run(cmd),
        Commands::Prompt(cmd) => prompt::run(cmd),
        Commands::Saju(cmd) => saju::run(cmd),
        Commands::Vedic(cmd) => vedic::run(cmd),
        Commands::Western(cmd) => western::run(cmd),
        Commands::Zwds(cmd) => zwds::run(cmd),
        Commands::Hd(cmd) => hd::run(cmd),
        Commands::Qimen(cmd) => qimen::run(cmd),
        Commands::Iching(cmd) => iching::run(cmd),
        Commands::Numerology(cmd) => numerology::run(cmd),
        Commands::Tier(cmd) => tier::run(cmd),
        Commands::Mcp(cmd) => mcp::run(cmd),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(args: &[&str]) -> Result<Cli, clap::Error> {
        Cli::try_parse_from(std::iter::once("eon").chain(args.iter().copied()))
    }

    #[test]
    fn negative_coordinates_are_accepted_without_equals_sign() {
        let cli = parse(&[
            "saju",
            "--birth",
            "1990-05-15 10:30",
            "--lat",
            "-33.87",
            "--lon",
            "-122.4",
        ])
        .expect("negative lat/lon must parse");
        match cli.command {
            Commands::Saju(cmd) => {
                assert_eq!(cmd.birth.lat, Some(-33.87));
                assert_eq!(cmd.birth.lon, Some(-122.4));
            }
            other => panic!("unexpected command: {other:?}"),
        }
    }

    #[test]
    fn single_engine_formats_list_only_what_they_do() {
        assert!(parse(&["saju", "--birth", "1990-05-15", "--format", "json"]).is_ok());
        assert!(parse(&["saju", "--birth", "1990-05-15", "--format", "text"]).is_ok());
        // Legacy spellings stay accepted as aliases of `text`.
        assert!(parse(&["saju", "--birth", "1990-05-15", "--format", "agent"]).is_ok());
        assert!(parse(&["saju", "--birth", "1990-05-15", "--format", "markdown"]).is_ok());
        assert!(parse(&["saju", "--birth", "1990-05-15", "--format", "bogus"]).is_err());
    }

    #[test]
    fn analyze_text_is_an_alias_of_agent() {
        let agent = parse(&["analyze", "--birth", "1990-05-15", "--format", "agent"]).unwrap();
        let text = parse(&["analyze", "--birth", "1990-05-15", "--format", "text"]).unwrap();
        let fmt = |c: Cli| match c.command {
            Commands::Analyze(a) => a.format,
            other => panic!("unexpected command: {other:?}"),
        };
        assert_eq!(fmt(agent), fmt(text));
    }
}
