// Test-only BeerXML fixture loader for stats tests
#![allow(dead_code)]

use quick_xml::events::Event;
use quick_xml::Reader;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use crate::models::{
    Mash, Recipe, RecipeAdditionFermentable, RecipeAdditionHop, RecipeAdditionYeast,
};

pub struct ExpectedStats {
    pub og: f64,
    pub fg: f64,
    pub ibu: f64,
    pub srm: f64,
}

pub fn load_fixture(filename: &str) -> (Recipe, ExpectedStats) {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("fixtures");
    path.push(filename);

    let mut buf = String::new();
    File::open(&path)
        .unwrap_or_else(|e| panic!("Failed to open fixture {}: {}", path.display(), e))
        .read_to_string(&mut buf)
        .unwrap();

    let mut reader = Reader::from_str(&buf);

    let mut buf_ev = Vec::new();

    // Temporary storage while parsing
    let mut batch_size_l: f64 = 23.0;
    let mut boil_time_min: f64 = 60.0;
    let mut efficiency_pct: Option<f64> = Some(72.0);

    let mut fermentables: Vec<RecipeAdditionFermentable> = Vec::new();
    let mut hops: Vec<RecipeAdditionHop> = Vec::new();
    let mut yeasts: Vec<RecipeAdditionYeast> = Vec::new();

    let mut est_og: Option<f64> = None;
    let mut est_fg: Option<f64> = None;
    let mut est_ibu: Option<f64> = None;
    let mut est_color: Option<f64> = None;

    // Simple state machine for elements
    let mut cur_elem = String::new();
    let mut cur_hop = None::<RecipeAdditionHop>;
    let mut cur_ferm = None::<RecipeAdditionFermentable>;
    let mut cur_yeast = None::<RecipeAdditionYeast>;

    loop {
        match reader.read_event_into(&mut buf_ev) {
            Ok(Event::Start(ref e)) => {
                cur_elem = String::from_utf8_lossy(e.name().as_ref()).to_string();
                match cur_elem.as_str() {
                    "HOP" => {
                        cur_hop = Some(RecipeAdditionHop {
                            id: "h1".into(),
                            recipe_id: "r1".into(),
                            hop_id: None,
                            name: "hop".into(),
                            alpha_pct: 0.0,
                            form: "pellet".into(),
                            amount_kg: 0.0,
                            use_: "Boil".into(),
                            time_min: 0.0,
                            addition_order: 0,
                            hopstand_temp_c: None,
                        });
                    }
                    "FERMENTABLE" => {
                        cur_ferm = Some(RecipeAdditionFermentable {
                            id: "f1".into(),
                            recipe_id: "r1".into(),
                            fermentable_id: None,
                            name: "ferm".into(),
                            type_: "grain".into(),
                            yield_pct: 75.0,
                            color_lovibond: 1.0,
                            amount_kg: 0.0,
                            add_after_boil: false,
                            addition_order: 0,
                        })
                    }
                    "YEAST" => {
                        cur_yeast = Some(RecipeAdditionYeast {
                            id: "y1".into(),
                            recipe_id: "r1".into(),
                            yeast_id: None,
                            name: "yeast".into(),
                            type_: "ale".into(),
                            form: "dry".into(),
                            laboratory: None,
                            product_id: None,
                            attenuation_pct: None,
                            amount: None,
                            amount_is_weight: false,
                            add_to_secondary: false,
                            times_cultured: 0,
                        })
                    }
                    _ => {}
                }
            }
            Ok(Event::Text(e)) => {
                let txt = e.unescape().unwrap().to_string();
                match cur_elem.as_str() {
                    "BATCH_SIZE" => {
                        if let Ok(v) = txt.parse::<f64>() {
                            batch_size_l = v;
                        }
                    }
                    "BOIL_TIME" => {
                        if let Ok(v) = txt.parse::<f64>() {
                            boil_time_min = v;
                        }
                    }
                    "EFFICIENCY" => {
                        if let Ok(v) = txt.parse::<f64>() {
                            efficiency_pct = Some(v);
                        }
                    }
                    "EST_OG" => {
                        est_og = txt.parse::<f64>().ok();
                    }
                    "EST_FG" => {
                        est_fg = txt.parse::<f64>().ok();
                    }
                    "IBU" => {
                        est_ibu = txt.parse::<f64>().ok();
                    }
                    "EST_COLOR" => {
                        est_color = txt.parse::<f64>().ok();
                    }
                    "NAME" => {
                        // assign to current child if present
                        if let Some(h) = cur_hop.as_mut() {
                            h.name = txt.clone();
                        } else if let Some(f) = cur_ferm.as_mut() {
                            f.name = txt.clone();
                        } else if let Some(y) = cur_yeast.as_mut() {
                            y.name = txt.clone();
                        }
                    }
                    "AMOUNT" => {
                        if let Some(h) = cur_hop.as_mut() {
                            if let Ok(v) = txt.parse::<f64>() {
                                // BeerXML AMOUNT is in kilograms
                                h.amount_kg = v;
                            }
                        } else if let Some(f) = cur_ferm.as_mut() {
                            if let Ok(v) = txt.parse::<f64>() {
                                f.amount_kg = v;
                            }
                        }
                    }
                    "ALPHA" => {
                        if let Some(h) = cur_hop.as_mut() {
                            if let Ok(v) = txt.parse::<f64>() {
                                h.alpha_pct = v;
                            }
                        }
                    }
                    "USE" => {
                        if let Some(h) = cur_hop.as_mut() {
                            h.use_ = txt.clone();
                        }
                    }
                    "TIME" => {
                        if let Some(h) = cur_hop.as_mut() {
                            if let Ok(v) = txt.parse::<f64>() {
                                h.time_min = v;
                            }
                        }
                    }
                    "YIELD" => {
                        if let Some(f) = cur_ferm.as_mut() {
                            if let Ok(v) = txt.parse::<f64>() {
                                f.yield_pct = v * 100.0; // some BeerXML yield is fraction
                            }
                        }
                    }
                    "COLOR" => {
                        if let Some(f) = cur_ferm.as_mut() {
                            if let Ok(v) = txt.parse::<f64>() {
                                f.color_lovibond = v;
                            }
                        }
                    }
                    "ATTENUATION" => {
                        if let Some(y) = cur_yeast.as_mut() {
                            if let Ok(v) = txt.parse::<f64>() {
                                y.attenuation_pct = Some(v);
                            }
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::End(ref e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                match name.as_str() {
                    "HOP" => {
                        if let Some(h) = cur_hop.take() {
                            hops.push(h);
                        }
                    }
                    "FERMENTABLE" => {
                        if let Some(f) = cur_ferm.take() {
                            fermentables.push(f);
                        }
                    }
                    "YEAST" => {
                        if let Some(y) = cur_yeast.take() {
                            yeasts.push(y);
                        }
                    }
                    _ => {}
                }
                cur_elem.clear();
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error parsing XML fixture {}: {}", path.display(), e),
            _ => {}
        }
        buf_ev.clear();
    }

    // Build a Recipe from parsed pieces
    let recipe = Recipe {
        id: "fixture".into(),
        name: filename.into(),
        type_: "all_grain".into(),
        brewer: None,
        asst_brewer: None,
        batch_size_l,
        boil_size_l: batch_size_l + 4.0,
        boil_time_min,
        efficiency_pct,
        style_id: None,
        equipment_profile_id: None,
        notes: None,
        taste_notes: None,
        taste_rating: None,
        og: None,
        fg: None,
        fermentation_stages: 1,
        primary_age_days: None,
        primary_temp_c: None,
        secondary_age_days: None,
        secondary_temp_c: None,
        tertiary_age_days: None,
        tertiary_temp_c: None,
        age_days: None,
        age_temp_c: None,
        carbonation_vols: None,
        forced_carbonation: false,
        priming_sugar_name: None,
        carbonation_temp_c: None,
        priming_sugar_equiv: None,
        keg_priming_factor: None,
        date: None,
        source: crate::models::RecipeSource::User,
        created_at: 0,
        updated_at: 0,
        equipment_profile: None,
        style: None,
        fermentables,
        hops,
        yeasts,
        miscs: vec![],
        waters: vec![],
        water_adjustments: vec![],
        mash_water_id: None,
        sparge_water_id: None,
        hopstand_temp_c: None,
        image_path: None,
        mash: Some(Mash {
            id: "m1".into(),
            recipe_id: "fixture".into(),
            name: "Single Infusion".into(),
            grain_temp_c: 20.0,
            tun_temp_c: None,
            sparge_temp_c: None,
            ph: None,
            tun_weight_kg: None,
            tun_specific_heat: None,
            equip_adjust: false,
            ratio_l_per_kg: None,
            notes: None,
            steps: vec![],
        }),
    };

    let expected = ExpectedStats {
        og: est_og.unwrap_or(1.0),
        fg: est_fg.unwrap_or(1.0),
        ibu: est_ibu.unwrap_or(0.0),
        srm: est_color.unwrap_or(0.0),
    };

    (recipe, expected)
}
