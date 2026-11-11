//! ESIL Emulation & Precision Full-Spectrum Timeline Audit Test
//!
//! Audits all events across a 100-year life path,
//! analyzes ESIL traces at specific timestamps from a reverse-engineering perspective,
//! and performs Inter-Process Compatibility (IPC / Gunghap) audits between two systems.

use eon_core::{BirthInfo, Gender};
use eon_saju::{CompatibilityAuditor, DestinyFuzzer, FourPillars, SajuInput, SajuVM};

fn main() {
    println!("╔═══════════════════════════════════════════════════════╗");
    println!("║       Eon Advanced Security Audit: ESIL & IPC         ║");
    println!("╚═══════════════════════════════════════════════════════╝\n");

    // 1. Target A: Kim Sung-ju (born 2004)
    let birth_a = BirthInfo::solar(2004, 11, 27, 22, 0).with_gender(Gender::Male);
    let (y_a, m_a, d_a, h_a) = birth_a.for_saju();
    let pillars_a = FourPillars::calculate(&SajuInput::new_solar(y_a, m_a, d_a, h_a, 0)).unwrap();
    let vm_a = SajuVM::new(pillars_a.clone());

    println!("【Subject A System Info】");
    println!("System Architecture: {}\n", pillars_a.hangul());

    // 2. Step 1 Test: ESIL Trace Analysis
    println!("【Step 1: ESIL Trace Analysis (Reversing)】");
    // Energy inflow analysis for 2026 Bing-Wu year, month 1 (Geng-Yin), day 11 (Yi-Chou), hour 10 (Xin-Si)
    use eon_saju::{EarthlyBranch, GanZi, HeavenlyStem};
    let major = pillars_a
        .major_luck(Gender::Male, 2004, 11, 27, 13, 0)
        .unwrap()
        .cycles[2]
        .ganzi; // Wu-Yin major luck
    let yearly = GanZi::new(HeavenlyStem::Bing, EarthlyBranch::Wu); // Bing-Wu year
    let monthly = GanZi::new(HeavenlyStem::Geng, EarthlyBranch::Yin); // Geng-Yin month
    let daily = GanZi::new(HeavenlyStem::Yi, EarthlyBranch::Chou); // Yi-Chou day
    let hourly = GanZi::new(HeavenlyStem::Xin, EarthlyBranch::Si); // Xin-Si hour

    let frame = vm_a.step(22, major, yearly, Some(monthly), Some(daily), Some(hourly));

    println!("Target Timestamp: 2026-01-11 10:00");
    println!("Event Instruction: [Major:{major}] [Year:{yearly}] [Month:{monthly}] [Day:{daily}] [Hour:{hourly}]");
    println!("ESIL Trace Logs:");
    for log in frame.esil_trace.split("; ") {
        if !log.is_empty() {
            println!("  >> {}", log);
        }
    }
    println!("Final Register State: {:?}", frame.register_state);
    println!("System Stability Score: {:.1}\n", frame.score);

    // 2-2. Interrupt test: 2024 Jia-Chen year (Baihu exception)
    println!("【Step 1-2: Interrupt Vector Table (IVT) Test】");
    let yearly_2024 = GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Chen); // Jia-Chen year (Baihu)
    let frame_2024 = vm_a.step(20, major, yearly_2024, None, None, None);

    println!("Target Event: 2024 Jia-Chen (Baihu Interuupt)");
    println!("ESIL Trace Logs:");
    for log in frame_2024.esil_trace.split("; ") {
        if log.contains("irq") || log.contains("kernel_panic") {
            println!("  [IRQ] >> {}", log);
        }
    }
    println!(
        "Final Register (Earth): {:.1} (Kernel Panic Overflow)",
        frame_2024.register_state.r2_earth
    );
    println!("Impact Score: {:.1}\n", frame_2024.score);

    // 3. Step 2 Test: 100-year full spectrum audit
    println!("【Step 2: Full Spectrum Timeline Audit (100 Years)】");
    let fuzzer = DestinyFuzzer::new(vm_a.clone());
    let luck_all = pillars_a
        .major_luck(Gender::Male, 2004, 11, 27, 13, 0)
        .unwrap();

    println!("Auditing ~43,200 time slots for potential system crashes...");
    let report = fuzzer.audit_high_res(2004, &luck_all);

    println!(
        "Audit Result: {} critical vulnerabilities discovered.",
        report.total_crashes
    );
    println!("Top 5 Critical Event Vectors:");
    for (i, v) in report.critical_vectors.iter().take(5).enumerate() {
        println!(
            "  [{}] Score: {:.1} | Time: {} | Type: {}",
            i + 1,
            v.crash_score,
            v.timestamp.as_deref().unwrap_or("Unknown"),
            v.vulnerability_type
        );
        println!(
            "     └─ Vector: [M:{}] [Y:{}] [Mo:{:?}]",
            v.vector.major, v.vector.yearly, v.vector.monthly
        );
    }
    println!();

    // 4. Step 3 Test: IPC Compatibility Audit
    println!("【Step 3: Inter-Process Compatibility (IPC) Audit】");
    // Simulated Counterpart B (Female born 2005)
    let pillars_b = FourPillars::calculate(&SajuInput::new_solar(2005, 5, 15, 10, 0)).unwrap();
    let vm_b = SajuVM::new(pillars_b);

    println!("Connecting to System B: {}...", vm_b.natal.hangul());
    let audit = CompatibilityAuditor::audit(&vm_a, &vm_b);

    println!("IPC Connection Score: {:.1}/100", audit.sync_score);

    if !audit.synergies.is_empty() {
        println!("[+] Synergies (Shared Resources):");
        for s in &audit.synergies {
            println!("   - {}", s);
        }
    }

    if !audit.conflicts.is_empty() {
        println!("[!] Conflicts (Race Conditions):");
        for c in &audit.conflicts {
            println!("   - {}", c);
        }
    }

    if !audit.deadlocks.is_empty() {
        println!("[x] Deadlocks (Resource Locks):");
        for d in &audit.deadlocks {
            println!("   - {}", d);
        }
    }

    println!("\nMerged Interaction Trace:");
    println!("  >> {}", audit.merged_esil_trace);

    println!(
        "\nConclusion: System {} is {}",
        if audit.sync_score >= 70.0 {
            "STABLE"
        } else if audit.sync_score >= 50.0 {
            "BALANCED"
        } else {
            "CRITICAL"
        },
        if audit.sync_score >= 70.0 {
            "highly compatible."
        } else {
            "requiring resource management."
        }
    );
}
