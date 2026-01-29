use eon_vedic::names::{get_nakshatra_name, get_rasi_name};
use eon_vedic::planets::VedicPlanet;
use eon_vedic::chart::VedicChartCalculator;
use chrono::{TimeZone, Utc};

#[test]
fn test_verify_user_d1_chart() {
    // 사용자 D1 차트 데이터 (검증용)
    // Sun: Anuradha (17) Pada 3
    // Moon: Rohini (4) Pada 3
    // Ascendant (ASC): Pushya (8) Pada 4
    
    // 이 테스트는 계산 로직이 맞는지 확인하기 위한 Mock Test임.
    // 실제로는 날짜를 넣어서 계산해야 하지만, 제공해주신 데이터가 이미 결과값이므로
    // 역으로 날짜를 추정하거나, 로직이 이 결과를 낼 수 있는지 확인하는 용도로는 아직 부족.
    // 일단은 나크샤트라 이름 매핑이 잘 되는지 확인.

    assert_eq!(get_nakshatra_name(17), "Anuradha");
    assert_eq!(get_nakshatra_name(4), "Rohini");
    assert_eq!(get_nakshatra_name(8), "Pushya");

    // 실제 생년월일시: 2004년 11월 27일 22:00 KST
    // KST = UTC + 9
    // UTC = 13:00
    // Latitude: 37.3167 (37°19'N), Longitude: 126.8167 (126°49'E)
    let time = Utc.ymd(2004, 11, 27).and_hms(13, 0, 0);
    
    // 2. Mean Node 설정으로 다시 계산 (Astro-Seek 기본값)
    use eon_vedic::config::{VedicConfig, NodeCalculation, AyanamsaSystem};
    
    let config = VedicConfig {
        ayanamsa: AyanamsaSystem::Lahiri,
        node_calc: NodeCalculation::MeanNode,
    };
    let calculator = VedicChartCalculator::with_config(config);
    let chart = calculator.calculate(time);

    println!("\n=== Calculated Vedic Chart (Mean Node) for User ===");
    println!("D1(Rasi)  D2(Hora)  D3(Drekk) D4(Chatur) D7(Sapt)  D9(Navam) D10(Das)  D12(Dwada)");
    println!("--------------------------------------------------------------------------------");
    
    for pos in &chart {
        println!("{:7} | {:9} | {:9} | {:9} | {:9} | {:9} | {:9} | {:9} | {:9}", 
            format!("{:?}", pos.planet),
            get_rasi_name(pos.rasi),
            get_rasi_name(pos.hora_rasi),
            get_rasi_name(pos.drekkana_rasi),
            get_rasi_name(pos.chaturthamsha_rasi),
            get_rasi_name(pos.saptamsa_rasi),
            get_rasi_name(pos.navamsa_rasi),
            get_rasi_name(pos.dasamsa_rasi),
            get_rasi_name(pos.dwadasamsa_rasi)
        );
    }
    
    // 검증 포인트 (제공해주신 데이터와 비교 - Mean Node)
    // Rahu: Aswini (1) Pada 2
    let rahu = chart.iter().find(|p| p.planet == VedicPlanet::Rahu).unwrap();
    assert_eq!(get_nakshatra_name(rahu.nakshatra), "Ashwini", "Rahu should be in Ashwini");
    assert_eq!(rahu.pada, 2, "Rahu should be in Pada 2 (Mean Node)");

    // Ketu: Chitra (14) Pada 4
    let ketu = chart.iter().find(|p| p.planet == VedicPlanet::Ketu).unwrap();
    assert_eq!(get_nakshatra_name(ketu.nakshatra), "Chitra", "Ketu should be in Chitra");
    assert_eq!(ketu.pada, 4, "Ketu should be in Pada 4 (Mean Node)");
    
    // Saturn Vargottam Check (D1 Cancer, D9 Cancer) -> Previous Check Confirmed
    let saturn = chart.iter().find(|p| p.planet == VedicPlanet::Saturn).unwrap();
    assert_eq!(saturn.rasi, 4, "Saturn D1 Cancer");
    assert_eq!(saturn.navamsa_rasi, 4, "Saturn D9 Cancer (Vargottam)");
}
