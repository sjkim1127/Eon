use serde::{Deserialize, Serialize};
use crate::planets::VedicPlanet;
use crate::chart::{VedicPosition, VedicChart};
use crate::analysis::relationships::{RelationshipEngine, RelationshipType};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BaladiAvastha {
    Bala,    // Infant (0-6)
    Kumara,  // Youthful (6-12)
    Yuva,    // Young Adult (12-18)
    Vriddha, // Old (18-24)
    Mrita,   // Dead (24-30)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum JagradadiAvastha {
    Jagrat,   // Awake (0-10)
    Swapna,   // Dream (10-20)
    Sushupti, // Sleep (20-30)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DeeptaadiAvastha {
    Deepta,   // Exalted
    Svastha,  // Own Sign
    Mudita,   // Great Friend
    Shanta,   // Friend
    Deena,    // Neutral
    Dukhita,  // Enemy
    Vikala,   // Great Enemy
    Khala,    // Debilitated
    Kopita,   // Combust
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LajjitadiAvastha {
    Lajjita,
    Garvita,
    Kshudhita,
    Trishita,
    Mudita,
    Kshobhita,
    Neutral,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanetAvastha {
    pub planet: VedicPlanet,
    pub baladi: BaladiAvastha,
    pub jagradadi: JagradadiAvastha,
    pub deeptaadi: DeeptaadiAvastha,
    pub lajjitadi: LajjitadiAvastha,
}

pub struct AvasthaEngine;

impl AvasthaEngine {
    pub fn calculate(pos: &VedicPosition, chart: &VedicChart) -> PlanetAvastha {
        let deg_in_sign = pos.sidereal_deg % 30.0;
        let rasi = pos.rasi;
        let is_odd = rasi % 2 != 0;

        let baladi = if is_odd {
            if deg_in_sign < 6.0 { BaladiAvastha::Bala }
            else if deg_in_sign < 12.0 { BaladiAvastha::Kumara }
            else if deg_in_sign < 18.0 { BaladiAvastha::Yuva }
            else if deg_in_sign < 24.0 { BaladiAvastha::Vriddha }
            else { BaladiAvastha::Mrita }
        } else if deg_in_sign < 6.0 { BaladiAvastha::Mrita }
        else if deg_in_sign < 12.0 { BaladiAvastha::Vriddha }
        else if deg_in_sign < 18.0 { BaladiAvastha::Yuva }
        else if deg_in_sign < 24.0 { BaladiAvastha::Kumara }
        else { BaladiAvastha::Bala };

        let jagradadi = if is_odd {
            if deg_in_sign < 10.0 { JagradadiAvastha::Jagrat }
            else if deg_in_sign < 20.0 { JagradadiAvastha::Swapna }
            else { JagradadiAvastha::Sushupti }
        } else if deg_in_sign < 10.0 { JagradadiAvastha::Sushupti }
        else if deg_in_sign < 20.0 { JagradadiAvastha::Swapna }
        else { JagradadiAvastha::Jagrat };

        // Deeptaadi Avastha
        let deeptaadi = if pos.is_combust {
            DeeptaadiAvastha::Kopita
        } else if rasi == pos.planet.exaltation_rasi() {
            DeeptaadiAvastha::Deepta
        } else if rasi == pos.planet.debilitation_rasi() {
            DeeptaadiAvastha::Khala
        } else {
            let lord = VedicPlanet::get_ruler_of(rasi);
            if lord == pos.planet {
                DeeptaadiAvastha::Svastha
            } else {
                let rel = RelationshipEngine::get_relationship(pos.planet, lord, chart);
                match rel {
                    RelationshipType::GreatFriend => DeeptaadiAvastha::Mudita,
                    RelationshipType::Friend => DeeptaadiAvastha::Shanta,
                    RelationshipType::Neutral => DeeptaadiAvastha::Deena,
                    RelationshipType::Enemy => DeeptaadiAvastha::Dukhita,
                    RelationshipType::GreatEnemy => DeeptaadiAvastha::Vikala,
                }
            }
        };

        let lajjitadi = calculate_lajjitadi(pos, &deeptaadi, chart);

        PlanetAvastha {
            planet: pos.planet,
            baladi,
            jagradadi,
            deeptaadi,
            lajjitadi,
        }
    }
}

// ── Lajjitadi Helpers ────────────────────────────────────────────────────────

fn is_aspected_by(planet_house: u8, aspecting: VedicPlanet, chart: &VedicChart) -> bool {
    chart.aspects.iter().any(|r| {
        r.aspecting_planet == aspecting && r.aspected_houses.contains(&planet_house)
    })
}

fn is_conjunct_with(pos: &VedicPosition, target: VedicPlanet, chart: &VedicChart) -> bool {
    chart.planets.iter().any(|p| p.planet == target && p.rasi == pos.rasi)
}

fn is_conjunct_malefic(pos: &VedicPosition, chart: &VedicChart) -> bool {
    let malefics = [VedicPlanet::Sun, VedicPlanet::Mars, VedicPlanet::Saturn, VedicPlanet::Rahu, VedicPlanet::Ketu];
    malefics.iter().any(|&m| m != pos.planet && is_conjunct_with(pos, m, chart))
}

fn is_aspected_by_malefic(pos: &VedicPosition, chart: &VedicChart) -> bool {
    let malefics = [VedicPlanet::Sun, VedicPlanet::Mars, VedicPlanet::Saturn, VedicPlanet::Rahu, VedicPlanet::Ketu];
    malefics.iter().any(|&m| m != pos.planet && is_aspected_by(pos.house_index, m, chart))
}

fn is_aspected_by_enemy(pos: &VedicPosition, chart: &VedicChart) -> bool {
    chart.planets.iter().any(|p| {
        if p.planet == pos.planet { return false; }
        let rel = RelationshipEngine::get_relationship(pos.planet, p.planet, chart);
        let is_enemy = rel == RelationshipType::Enemy || rel == RelationshipType::GreatEnemy;
        is_enemy && is_aspected_by(pos.house_index, p.planet, chart)
    })
}

fn calculate_lajjitadi(pos: &VedicPosition, deeptaadi: &DeeptaadiAvastha, chart: &VedicChart) -> LajjitadiAvastha {
    // 1. Lajjita: conjunct malefic in 5th house
    if pos.house_index == 5 && is_conjunct_malefic(pos, chart) {
        return LajjitadiAvastha::Lajjita;
    }

    // 2. Garvita: exalted or own sign
    if *deeptaadi == DeeptaadiAvastha::Deepta || *deeptaadi == DeeptaadiAvastha::Svastha {
        return LajjitadiAvastha::Garvita;
    }

    // 3. Kshobhita: combust or aspected by malefic in enemy sign
    let is_in_enemy_sign = *deeptaadi == DeeptaadiAvastha::Dukhita || *deeptaadi == DeeptaadiAvastha::Vikala;
    if pos.is_combust || (is_in_enemy_sign && is_aspected_by_malefic(pos, chart)) {
        return LajjitadiAvastha::Kshobhita;
    }

    // 4. Kshudhita: in enemy sign, or conjunct/aspected by enemy, or conjunct Saturn
    let is_conjunct_enemy = chart.planets.iter().any(|p| {
        if p.planet == pos.planet { return false; }
        let rel = RelationshipEngine::get_relationship(pos.planet, p.planet, chart);
        (rel == RelationshipType::Enemy || rel == RelationshipType::GreatEnemy) && p.rasi == pos.rasi
    });
    if is_in_enemy_sign || is_conjunct_enemy || is_aspected_by_enemy(pos, chart) || (pos.planet != VedicPlanet::Saturn && is_conjunct_with(pos, VedicPlanet::Saturn, chart)) {
        return LajjitadiAvastha::Kshudhita;
    }

    // 5. Trishita: in water sign (4, 8, 12) aspected by malefic and no benefic aspects
    let is_water_sign = [4, 8, 12].contains(&pos.rasi);
    let has_benefic_aspect = [VedicPlanet::Jupiter, VedicPlanet::Venus, VedicPlanet::Mercury].iter().any(|&b| {
        b != pos.planet && is_aspected_by(pos.house_index, b, chart)
    });
    if is_water_sign && is_aspected_by_malefic(pos, chart) && !has_benefic_aspect {
        return LajjitadiAvastha::Trishita;
    }

    // 6. Mudita: in friend sign, conjunct/aspected by friend, conjunct Jupiter
    let is_friend_sign = *deeptaadi == DeeptaadiAvastha::Mudita || *deeptaadi == DeeptaadiAvastha::Shanta;
    let is_conjunct_friend = chart.planets.iter().any(|p| {
        if p.planet == pos.planet { return false; }
        let rel = RelationshipEngine::get_relationship(pos.planet, p.planet, chart);
        (rel == RelationshipType::Friend || rel == RelationshipType::GreatFriend) && p.rasi == pos.rasi
    });
    let is_aspected_by_friend = chart.planets.iter().any(|p| {
        if p.planet == pos.planet { return false; }
        let rel = RelationshipEngine::get_relationship(pos.planet, p.planet, chart);
        (rel == RelationshipType::Friend || rel == RelationshipType::GreatFriend) && is_aspected_by(pos.house_index, p.planet, chart)
    });
    if is_friend_sign || is_conjunct_friend || is_aspected_by_friend || (pos.planet != VedicPlanet::Jupiter && is_conjunct_with(pos, VedicPlanet::Jupiter, chart)) {
        return LajjitadiAvastha::Mudita;
    }

    LajjitadiAvastha::Neutral
}
