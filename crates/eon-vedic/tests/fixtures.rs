//! Vedic Engine Test Fixtures
//!
//! # Data Sources
//! - B.V. Raman: 1912 Aug 8, 19:43 IST, Bangalore. (Source: "My Experiments with Astrology")
//! - Amitabh Bachchan: 1942 Oct 11, 16:00 IST, Allahabad. (Source: Astro-Databank, Rodden Rating: A)
//! - Albert Einstein: 1879 Mar 14, 11:30 local time, Ulm, Germany. (Source: Astro-Databank, Rodden Rating: AA)
//! - Mahatma Gandhi: 1869 Oct 2, 07:12 IST, Porbandar. (Source: Astro-Databank, Rodden Rating: C)
//! - Indira Gandhi: 1917 Nov 19, 23:11 IST, Allahabad. (Source: Astro-Databank, Rodden Rating: A)
//! - Steve Jobs: 1955 Feb 24, 19:15 local time, San Francisco. (Source: Astro-Databank, Rodden Rating: AA)
//! - Queen Elizabeth II: 1926 Apr 21, 02:40 local time, London. (Source: Astro-Databank, Rodden Rating: AA)
//! - Nelson Mandela: 1918 Jul 18, 14:54 local time, Mvezo, South Africa. (Source: Astro-Databank, Rodden Rating: A)
//! - Sri Yukteswar: 1855 May 10, 19:01 IST, Serampore. (Source: Holy Science / Standard Records)
//!
//! # Validation Strategy
//! Expected values are established using the Eon Vedic Engine baseline (Swiss Ephemeris 2.10).
//! These baselines ensure that future architectural changes do not regress core calculations.

use crate::common::*;
use eon_vedic::planets::VedicPlanet;
use eon_vedic::config::AyanamsaSystem;

pub const FIXTURES: &[VedicTestFixture] = &[
    VedicTestFixture {
        name: "B.V. Raman",
        year: 1912,
        month: 8,
        day: 8,
        hour: 14,
        minute: 13,
        latitude: 13.0,
        longitude: 77.5,
        ayanamsa: AyanamsaSystem::Raman,
        jaimini: Some(ExpectedJaimini {
            atmakaraka: Some(VedicPlanet::Moon),
            arudha_lagna: Some(9),
            upapada_lagna: Some(6),
            a10_rasi: Some(6),
        }),
        varga: Some(ExpectedVarga {
            navamsa_lagna: Some(10),
            dasamsa_lagna: Some(2),
            vargottama_planets: &[VedicPlanet::Mercury, VedicPlanet::Jupiter],
            pushkar_planets: &[VedicPlanet::Mars],
        }),
        dasha: Some(ExpectedDasha {
            current_yogini: Some("Sankata"),
        }),
        tajika: Some(ExpectedTajika {
            punya_rasi: Some(1),
            vidya_rasi: Some(9),
            high_harsha_bala: &[VedicPlanet::Moon],
        }),
        avastha: Some(ExpectedAvastha {
            deeptaadi: &[(VedicPlanet::Jupiter, "Mudita"), (VedicPlanet::Venus, "Kopita")],
        }),
    },
    VedicTestFixture {
        name: "Amitabh Bachchan",
        year: 1942,
        month: 10,
        day: 11,
        hour: 10,
        minute: 30,
        latitude: 25.45,
        longitude: 81.85,
        ayanamsa: AyanamsaSystem::Lahiri,
        jaimini: Some(ExpectedJaimini {
            atmakaraka: Some(VedicPlanet::Sun),
            arudha_lagna: Some(9),
            upapada_lagna: Some(6),
            a10_rasi: Some(4),
        }),
        varga: Some(ExpectedVarga {
            navamsa_lagna: Some(1),
            dasamsa_lagna: Some(6),
            vargottama_planets: &[VedicPlanet::Jupiter],
            pushkar_planets: &[VedicPlanet::Jupiter, VedicPlanet::Venus],
        }),
        dasha: Some(ExpectedDasha {
            current_yogini: Some("Pingala"),
        }),
        tajika: Some(ExpectedTajika {
            punya_rasi: Some(11),
            vidya_rasi: Some(12),
            high_harsha_bala: &[],
        }),
        avastha: Some(ExpectedAvastha {
            deeptaadi: &[(VedicPlanet::Jupiter, "Deepta"), (VedicPlanet::Mercury, "Kopita")],
        }),
    },
    VedicTestFixture {
        name: "Albert Einstein",
        year: 1879,
        month: 3,
        day: 14,
        hour: 10,
        minute: 50,
        latitude: 48.4,
        longitude: 10.0,
        ayanamsa: AyanamsaSystem::Lahiri,
        jaimini: Some(ExpectedJaimini {
            atmakaraka: Some(VedicPlanet::Venus),
            arudha_lagna: Some(1),
            upapada_lagna: Some(10),
            a10_rasi: Some(10),
        }),
        varga: Some(ExpectedVarga {
            navamsa_lagna: Some(12),
            dasamsa_lagna: Some(9),
            vargottama_planets: &[],
            pushkar_planets: &[VedicPlanet::Sun],
        }),
        dasha: Some(ExpectedDasha {
            current_yogini: Some("Bhadrika"),
        }),
        tajika: Some(ExpectedTajika {
            punya_rasi: Some(12),
            vidya_rasi: Some(6),
            high_harsha_bala: &[],
        }),
        avastha: None,
    },
    VedicTestFixture {
        name: "Mahatma Gandhi",
        year: 1869,
        month: 10,
        day: 2,
        hour: 1,
        minute: 42,
        latitude: 21.63,
        longitude: 69.6,
        ayanamsa: AyanamsaSystem::Lahiri,
        jaimini: Some(ExpectedJaimini {
            atmakaraka: Some(VedicPlanet::Jupiter),
            arudha_lagna: Some(8),
            upapada_lagna: Some(7),
            a10_rasi: Some(11),
        }),
        varga: Some(ExpectedVarga {
            navamsa_lagna: Some(4),
            dasamsa_lagna: Some(9),
            vargottama_planets: &[],
            pushkar_planets: &[VedicPlanet::Mars, VedicPlanet::Jupiter, VedicPlanet::Venus],
        }),
        dasha: Some(ExpectedDasha {
            current_yogini: Some("Bhramari"),
        }),
        tajika: Some(ExpectedTajika {
            punya_rasi: Some(5),
            vidya_rasi: Some(8),
            high_harsha_bala: &[],
        }),
        avastha: None,
    },
    VedicTestFixture {
        name: "Indira Gandhi",
        year: 1917,
        month: 11,
        day: 19,
        hour: 17,
        minute: 41,
        latitude: 25.45,
        longitude: 81.85,
        ayanamsa: AyanamsaSystem::Lahiri,
        jaimini: Some(ExpectedJaimini {
            atmakaraka: Some(VedicPlanet::Saturn),
            arudha_lagna: Some(1),
            upapada_lagna: Some(1),
            a10_rasi: Some(9),
        }),
        varga: Some(ExpectedVarga {
            navamsa_lagna: Some(12),
            dasamsa_lagna: Some(9),
            vargottama_planets: &[VedicPlanet::Mars, VedicPlanet::Jupiter],
            pushkar_planets: &[VedicPlanet::Jupiter, VedicPlanet::Venus],
        }),
        dasha: Some(ExpectedDasha {
            current_yogini: Some("Sankata"),
        }),
        tajika: Some(ExpectedTajika {
            punya_rasi: Some(2),
            vidya_rasi: Some(6),
            high_harsha_bala: &[],
        }),
        avastha: None,
    },
    VedicTestFixture {
        name: "Steve Jobs",
        year: 1955,
        month: 2,
        day: 24,
        hour: 19,
        minute: 15,
        latitude: 37.77,
        longitude: -122.42,
        ayanamsa: AyanamsaSystem::Lahiri,
        jaimini: Some(ExpectedJaimini {
            atmakaraka: Some(VedicPlanet::Saturn),
            arudha_lagna: Some(4),
            upapada_lagna: Some(10),
            a10_rasi: Some(3),
        }),
        varga: Some(ExpectedVarga {
            navamsa_lagna: Some(3),
            dasamsa_lagna: Some(3),
            vargottama_planets: &[VedicPlanet::Jupiter, VedicPlanet::Venus],
            pushkar_planets: &[VedicPlanet::Moon, VedicPlanet::Venus],
        }),
        dasha: Some(ExpectedDasha {
            current_yogini: Some("Bhadrika"),
        }),
        tajika: Some(ExpectedTajika {
            punya_rasi: Some(1),
            vidya_rasi: Some(3),
            high_harsha_bala: &[],
        }),
        avastha: None,
    },
    VedicTestFixture {
        name: "Queen Elizabeth II",
        year: 1926,
        month: 4,
        day: 21,
        hour: 1,
        minute: 40,
        latitude: 51.5,
        longitude: -0.12,
        ayanamsa: AyanamsaSystem::Lahiri,
        jaimini: Some(ExpectedJaimini {
            atmakaraka: Some(VedicPlanet::Jupiter),
            arudha_lagna: Some(11),
            upapada_lagna: Some(12),
            a10_rasi: Some(3),
        }),
        varga: Some(ExpectedVarga {
            navamsa_lagna: Some(9),
            dasamsa_lagna: Some(6),
            vargottama_planets: &[],
            pushkar_planets: &[VedicPlanet::Saturn],
        }),
        dasha: Some(ExpectedDasha {
            current_yogini: Some("Bhramari"),
        }),
        tajika: Some(ExpectedTajika {
            punya_rasi: Some(6),
            vidya_rasi: Some(1),
            high_harsha_bala: &[VedicPlanet::Saturn],
        }),
        avastha: None,
    },
    VedicTestFixture {
        name: "Nelson Mandela",
        year: 1918,
        month: 7,
        day: 18,
        hour: 12,
        minute: 54,
        latitude: -28.48,
        longitude: 28.5,
        ayanamsa: AyanamsaSystem::Lahiri,
        jaimini: Some(ExpectedJaimini {
            atmakaraka: Some(VedicPlanet::Venus),
            arudha_lagna: Some(4),
            upapada_lagna: Some(9),
            a10_rasi: Some(3),
        }),
        varga: Some(ExpectedVarga {
            navamsa_lagna: Some(12),
            dasamsa_lagna: Some(1),
            vargottama_planets: &[VedicPlanet::Sun],
            pushkar_planets: &[VedicPlanet::Sun],
        }),
        dasha: Some(ExpectedDasha {
            current_yogini: Some("Dhanya"),
        }),
        tajika: Some(ExpectedTajika {
            punya_rasi: Some(12),
            vidya_rasi: Some(5),
            high_harsha_bala: &[VedicPlanet::Sun],
        }),
        avastha: None,
    },
    VedicTestFixture {
        name: "Sri Yukteswar",
        year: 1855,
        month: 5,
        day: 10,
        hour: 13,
        minute: 30,
        latitude: 22.57,
        longitude: 88.36,
        ayanamsa: AyanamsaSystem::Raman,
        jaimini: Some(ExpectedJaimini {
            atmakaraka: Some(VedicPlanet::Sun),
            arudha_lagna: Some(6),
            upapada_lagna: Some(11),
            a10_rasi: Some(9),
        }),
        varga: Some(ExpectedVarga {
            navamsa_lagna: Some(7),
            dasamsa_lagna: Some(7),
            vargottama_planets: &[],
            pushkar_planets: &[VedicPlanet::Sun, VedicPlanet::Moon, VedicPlanet::Mars],
        }),
        dasha: Some(ExpectedDasha {
            current_yogini: Some("Dhanya"),
        }),
        tajika: Some(ExpectedTajika {
            punya_rasi: Some(10),
            vidya_rasi: Some(6),
            high_harsha_bala: &[VedicPlanet::Moon],
        }),
        avastha: None,
    },
    VedicTestFixture {
        name: "Modern Case",
        year: 2026,
        month: 3,
        day: 26,
        hour: 9,
        minute: 0,
        latitude: 37.56,
        longitude: 126.97,
        ayanamsa: AyanamsaSystem::Lahiri,
        jaimini: Some(ExpectedJaimini {
            atmakaraka: Some(VedicPlanet::Mars),
            arudha_lagna: Some(4),
            upapada_lagna: Some(7),
            a10_rasi: Some(7),
        }),
        varga: Some(ExpectedVarga {
            navamsa_lagna: Some(10),
            dasamsa_lagna: Some(2),
            vargottama_planets: &[VedicPlanet::Mercury, VedicPlanet::Venus],
            pushkar_planets: &[VedicPlanet::Moon, VedicPlanet::Mars],
        }),
        dasha: Some(ExpectedDasha {
            current_yogini: Some("Mangala"),
        }),
        tajika: Some(ExpectedTajika {
            punya_rasi: Some(2),
            vidya_rasi: Some(9),
            high_harsha_bala: &[],
        }),
        avastha: None,
    }
];
