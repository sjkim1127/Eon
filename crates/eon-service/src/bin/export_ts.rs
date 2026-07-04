use eon_service::dto::*;
use eon_service::error::ServiceError;
use ts_rs::TS;

fn main() -> std::io::Result<()> {
    AnalysisRequest::export().unwrap();
    SajuAnalysisRequest::export().unwrap();
    VedicAnalysisRequest::export().unwrap();
    TransitAnalysisRequest::export().unwrap();
    TierResult::export().unwrap();
    DestinyTierRequest::export().unwrap();
    TierGrade::export().unwrap();
    DomainTier::export().unwrap();
    ScoreResult::export().unwrap();
    DestinyComponent::export().unwrap();
    ServiceError::export().unwrap();

    println!("TypeScript types exported successfully.");
    Ok(())
}
