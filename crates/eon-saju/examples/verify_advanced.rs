//! ESIL 에뮬레이션 및 초정밀 전수 조사 엔진 테스트
//! 
//! 100년치 인생 경로의 모든 이벤트를 전수 조사하고,
//! 특정 시점의 ESIL 트레이스를 리버싱 관점에서 분석합니다.
//! 또한 두 시스템 간의 IPC(궁합) 감사를 수행합니다.

use eon_core::{BirthInfo, Gender, Location};
use eon_saju::{FourPillars, SajuInput, SajuVM, DestinyFuzzer, CompatibilityAuditor};

fn main() {
    println!("╔═══════════════════════════════════════════════════════╗");
    println!("║       Eon Advanced Security Audit: ESIL & IPC         ║");
    println!("╚═══════════════════════════════════════════════════════╝\n");

    // 1. 대상 A: 김성주님 (2004년생)
    let birth_a = BirthInfo::solar(2004, 11, 27, 22, 0)
        .with_gender(Gender::Male);
    let (y_a, m_a, d_a, h_a) = birth_a.for_saju();
    let pillars_a = FourPillars::calculate(&SajuInput::new_solar(y_a, m_a, d_a, h_a, 0)).unwrap();
    let vm_a = SajuVM::new(pillars_a.clone());

    println!("【Subject A System Info】");
    println!("System Architecture: {}\n", pillars_a.hangul());

    // 2. 단계 1 테스트: ESIL Trace 분석
    println!("【Step 1: ESIL Trace Analysis (Reversing)】");
    // 2026년 丙午년 1월(庚寅) 11일(乙丑) 10시(辛巳)의 기운 유입 분석
    use eon_saju::{GanZi, HeavenlyStem, EarthlyBranch};
    let major = pillars_a.major_luck(Gender::Male, 2004, 11, 27, 13, 0).unwrap().cycles[2].ganzi; // 戊寅 대운
    let yearly = GanZi::new(HeavenlyStem::Bing, EarthlyBranch::Wu);    // 丙午년
    let monthly = GanZi::new(HeavenlyStem::Geng, EarthlyBranch::Yin); // 庚寅월
    let daily = GanZi::new(HeavenlyStem::Yi, EarthlyBranch::Chou);    // 乙丑일
    let hourly = GanZi::new(HeavenlyStem::Xin, EarthlyBranch::Si);    // 辛巳시

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

    // 2-2. 인터럽트 테스트: 2024년 甲辰년 (백호대살 예외 발생)
    println!("【Step 1-2: Interrupt Vector Table (IVT) Test】");
    let yearly_2024 = GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Chen); // 甲辰년 (백호살)
    let frame_2024 = vm_a.step(20, major, yearly_2024, None, None, None);
    
    println!("Target Event: 2024 Jia-Chen (Baihu Interuupt)");
    println!("ESIL Trace Logs:");
    for log in frame_2024.esil_trace.split("; ") {
        if log.contains("irq") || log.contains("kernel_panic") {
            println!("  [IRQ] >> {}", log);
        }
    }
    println!("Final Register (Earth): {:.1} (Kernel Panic Overflow)", frame_2024.register_state.r2_earth);
    println!("Impact Score: {:.1}\n", frame_2024.score);

    // 3. 단계 2 테스트: 100년 전수 조사 (Full Spectrum Audit)
    println!("【Step 2: Full Spectrum Timeline Audit (100 Years)】");
    let fuzzer = DestinyFuzzer::new(vm_a.clone());
    let luck_all = pillars_a.major_luck(Gender::Male, 2004, 11, 27, 13, 0).unwrap();
    
    println!("Auditing ~43,200 time slots for potential system crashes...");
    let report = fuzzer.audit_high_res(2004, &luck_all);
    
    println!("Audit Result: {} critical vulnerabilities discovered.", report.total_crashes);
    println!("Top 5 Critical Event Vectors:");
    for (i, v) in report.critical_vectors.iter().take(5).enumerate() {
        println!("  [{}] Score: {:.1} | Time: {} | Type: {}", 
            i + 1, v.crash_score, v.timestamp.as_deref().unwrap_or("Unknown"), v.vulnerability_type);
        println!("     └─ Vector: [M:{}] [Y:{}] [Mo:{:?}]", v.vector.major, v.vector.yearly, v.vector.monthly);
    }
    println!();

    // 4. 단계 3 테스트: IPC 궁합 감사 (Inter-Process Compatibility)
    println!("【Step 3: Inter-Process Compatibility (IPC) Audit】");
    // 가상의 상대방 B (2005년생 여성)
    let pillars_b = FourPillars::calculate(&SajuInput::new_solar(2005, 5, 15, 10, 0)).unwrap();
    let vm_b = SajuVM::new(pillars_b);

    println!("Connecting to System B: {}...", vm_b.natal.hangul());
    let audit = CompatibilityAuditor::audit(&vm_a, &vm_b);
    
    println!("IPC Connection Score: {:.1}/100", audit.sync_score);
    
    if !audit.synergies.is_empty() {
        println!("✅ Synergies (Shared Resources):");
        for s in &audit.synergies { println!("   - {}", s); }
    }
    
    if !audit.conflicts.is_empty() {
        println!("⚠️ Conflicts (Race Conditions):");
        for c in &audit.conflicts { println!("   - {}", c); }
    }

    if !audit.deadlocks.is_empty() {
        println!("🚫 Deadlocks (Resource Locks):");
        for d in &audit.deadlocks { println!("   - {}", d); }
    }

    println!("\nMerged Interaction Trace:");
    println!("  >> {}", audit.merged_esil_trace);
    
    println!("\nConclusion: System {} is {}", 
        if audit.sync_score >= 70.0 { "STABLE" } else if audit.sync_score >= 50.0 { "BALANCED" } else { "CRITICAL" },
        if audit.sync_score >= 70.0 { "highly compatible." } else { "requiring resource management." }
    );
}
