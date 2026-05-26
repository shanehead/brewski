use quick_xml::events::{BytesDecl, BytesText, Event};
use quick_xml::reader::Reader;
use quick_xml::Writer;
use tauri::State;

use crate::models::{
    CreateFermentableAdditionInput, CreateHopAdditionInput, CreateMiscAdditionInput,
    CreateRecipeInput, CreateYeastAdditionInput, Recipe, RecipeSummary,
};
use crate::repositories::fermentable::FermentableRepository;
use crate::repositories::hop::HopRepository;
use crate::repositories::misc::MiscRepository;
use crate::repositories::recipe::RecipeRepository;
use crate::repositories::yeast::YeastRepository;
use crate::AppState;

fn build_recipe_beerxml(recipe: &Recipe) -> Result<String, quick_xml::Error> {
    let mut writer = Writer::new_with_indent(Vec::new(), b' ', 2);
    writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))?;

    let brewer = recipe.brewer.as_deref().unwrap_or("");
    let efficiency = format!("{:.1}", recipe.efficiency_pct.unwrap_or(72.0));

    type E = quick_xml::Error;

    writer
        .create_element("RECIPES")
        .write_inner_content(|w| -> Result<(), E> {
            w.create_element("RECIPE")
                .write_inner_content(|w| -> Result<(), E> {
                    w.create_element("NAME")
                        .write_text_content(BytesText::new(&recipe.name))?;
                    w.create_element("VERSION")
                        .write_text_content(BytesText::new("1"))?;
                    w.create_element("TYPE")
                        .write_text_content(BytesText::new(&recipe.type_))?;
                    w.create_element("BREWER")
                        .write_text_content(BytesText::new(brewer))?;
                    w.create_element("BATCH_SIZE")
                        .write_text_content(BytesText::new(&format!(
                            "{:.1}",
                            recipe.batch_size_l
                        )))?;
                    w.create_element("BOIL_SIZE")
                        .write_text_content(BytesText::new(&format!(
                            "{:.1}",
                            recipe.boil_size_l
                        )))?;
                    w.create_element("BOIL_TIME")
                        .write_text_content(BytesText::new(&format!(
                            "{:.0}",
                            recipe.boil_time_min
                        )))?;
                    w.create_element("EFFICIENCY")
                        .write_text_content(BytesText::new(&efficiency))?;

                    if let Some(s) = &recipe.style {
                        w.create_element("STYLE")
                            .write_inner_content(|w| -> Result<(), E> {
                                w.create_element("NAME")
                                    .write_text_content(BytesText::new(&s.name))?;
                                w.create_element("CATEGORY")
                                    .write_text_content(BytesText::new(&s.category))?;
                                w.create_element("STYLE_GUIDE")
                                    .write_text_content(BytesText::new(&s.style_guide))?;
                                Ok(())
                            })?;
                    }

                    w.create_element("FERMENTABLES")
                        .write_inner_content(|w| -> Result<(), E> {
                            for f in &recipe.fermentables {
                                w.create_element("FERMENTABLE").write_inner_content(
                                    |w| -> Result<(), E> {
                                        w.create_element("NAME")
                                            .write_text_content(BytesText::new(&f.name))?;
                                        w.create_element("AMOUNT").write_text_content(
                                            BytesText::new(&format!("{:.4}", f.amount_kg)),
                                        )?;
                                        w.create_element("TYPE")
                                            .write_text_content(BytesText::new(&f.type_))?;
                                        w.create_element("YIELD").write_text_content(
                                            BytesText::new(&format!("{:.1}", f.yield_pct)),
                                        )?;
                                        w.create_element("COLOR").write_text_content(
                                            BytesText::new(&format!("{:.1}", f.color_lovibond)),
                                        )?;
                                        Ok(())
                                    },
                                )?;
                            }
                            Ok(())
                        })?;

                    w.create_element("HOPS")
                        .write_inner_content(|w| -> Result<(), E> {
                            for h in &recipe.hops {
                                w.create_element("HOP").write_inner_content(
                                    |w| -> Result<(), E> {
                                        w.create_element("NAME")
                                            .write_text_content(BytesText::new(&h.name))?;
                                        w.create_element("AMOUNT").write_text_content(
                                            BytesText::new(&format!("{:.5}", h.amount_kg)),
                                        )?;
                                        w.create_element("ALPHA").write_text_content(
                                            BytesText::new(&format!("{:.1}", h.alpha_pct)),
                                        )?;
                                        w.create_element("USE")
                                            .write_text_content(BytesText::new(&h.use_))?;
                                        w.create_element("TIME").write_text_content(
                                            BytesText::new(&format!("{:.0}", h.time_min)),
                                        )?;
                                        w.create_element("FORM")
                                            .write_text_content(BytesText::new(&h.form))?;
                                        Ok(())
                                    },
                                )?;
                            }
                            Ok(())
                        })?;

                    w.create_element("YEASTS")
                        .write_inner_content(|w| -> Result<(), E> {
                            for y in &recipe.yeasts {
                                w.create_element("YEAST").write_inner_content(
                                    |w| -> Result<(), E> {
                                        w.create_element("NAME")
                                            .write_text_content(BytesText::new(&y.name))?;
                                        w.create_element("TYPE")
                                            .write_text_content(BytesText::new(&y.type_))?;
                                        w.create_element("FORM")
                                            .write_text_content(BytesText::new(&y.form))?;
                                        w.create_element("AMOUNT").write_text_content(
                                            BytesText::new(&format!(
                                                "{:.4}",
                                                y.amount.unwrap_or(0.0)
                                            )),
                                        )?;
                                        Ok(())
                                    },
                                )?;
                            }
                            Ok(())
                        })?;

                    Ok(())
                })?;
            Ok(())
        })?;

    Ok(String::from_utf8(writer.into_inner()).expect("quick_xml produced non-UTF-8"))
}

#[tauri::command]
pub async fn get_recipe_beerxml(
    state: State<'_, AppState>,
    recipe_id: String,
) -> Result<String, String> {
    let recipe = RecipeRepository::new(&state.db)
        .get(&recipe_id)
        .await
        .map_err(|e| e.to_string())?;
    build_recipe_beerxml(&recipe).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn write_recipe_beerxml(
    state: State<'_, AppState>,
    recipe_id: String,
    path: String,
) -> Result<(), String> {
    let recipe = RecipeRepository::new(&state.db)
        .get(&recipe_id)
        .await
        .map_err(|e| e.to_string())?;
    let xml = build_recipe_beerxml(&recipe).map_err(|e| e.to_string())?;
    std::fs::write(&path, xml).map_err(|e| e.to_string())
}

// Parsed representation of a single BeerXML <RECIPE> block.
struct ParsedRecipe {
    name: String,
    type_: String,
    batch_size_l: f64,
    boil_size_l: f64,
    boil_time_min: f64,
    fermentables: Vec<CreateFermentableAdditionInput>,
    hops: Vec<CreateHopAdditionInput>,
    yeasts: Vec<CreateYeastAdditionInput>,
    miscs: Vec<CreateMiscAdditionInput>,
}

/// Parse all `<RECIPE>` blocks from a BeerXML string.
fn parse_beerxml(xml: &str) -> Result<Vec<ParsedRecipe>, String> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut recipes = Vec::new();

    loop {
        match reader.read_event().map_err(|e| e.to_string())? {
            Event::Start(e) if e.name().as_ref() == b"RECIPE" => {
                recipes.push(parse_recipe(&mut reader)?);
            }
            Event::Eof => break,
            _ => {}
        }
    }

    Ok(recipes)
}

fn parse_recipe(reader: &mut Reader<&[u8]>) -> Result<ParsedRecipe, String> {
    let mut name = String::from("Imported Recipe");
    let mut type_ = String::from("all_grain");
    let mut batch_size_l = 23.0f64;
    let mut boil_size_l = 27.0f64;
    let mut boil_time_min = 60.0f64;
    let mut fermentables = Vec::new();
    let mut hops = Vec::new();
    let mut yeasts = Vec::new();
    let mut miscs = Vec::new();

    loop {
        match reader.read_event().map_err(|e| e.to_string())? {
            Event::Start(e) => match e.name().as_ref() {
                b"NAME" => name = read_text(reader)?,
                b"TYPE" => type_ = read_text(reader)?,
                b"BATCH_SIZE" => batch_size_l = read_text(reader)?.parse().unwrap_or(23.0),
                b"BOIL_SIZE" => boil_size_l = read_text(reader)?.parse().unwrap_or(27.0),
                b"BOIL_TIME" => boil_time_min = read_text(reader)?.parse().unwrap_or(60.0),
                // Container elements — just descend into them; children are matched below
                b"FERMENTABLES" | b"HOPS" | b"YEASTS" | b"MISCS" => {}
                b"FERMENTABLE" => fermentables.push(parse_fermentable(reader)?),
                b"HOP" => hops.push(parse_hop(reader)?),
                b"YEAST" => yeasts.push(parse_yeast(reader)?),
                b"MISC" => miscs.push(parse_misc(reader)?),
                _ => skip_element(reader, e.name().as_ref())?,
            },
            Event::End(e) if e.name().as_ref() == b"RECIPE" => break,
            Event::Eof => return Err("Unexpected EOF inside <RECIPE>".into()),
            _ => {}
        }
    }

    Ok(ParsedRecipe {
        name,
        type_,
        batch_size_l,
        boil_size_l,
        boil_time_min,
        fermentables,
        hops,
        yeasts,
        miscs,
    })
}

fn parse_fermentable(reader: &mut Reader<&[u8]>) -> Result<CreateFermentableAdditionInput, String> {
    let mut name = String::new();
    let mut type_ = String::from("grain");
    let mut yield_pct = 75.0f64;
    let mut color_lovibond = 2.0f64;
    let mut amount_kg = 0.0f64;
    let mut add_after_boil = None;

    loop {
        match reader.read_event().map_err(|e| e.to_string())? {
            Event::Start(e) => match e.name().as_ref() {
                b"NAME" => name = read_text(reader)?,
                b"TYPE" => type_ = read_text(reader)?,
                b"YIELD" => yield_pct = read_text(reader)?.parse().unwrap_or(75.0),
                b"COLOR" => color_lovibond = read_text(reader)?.parse().unwrap_or(2.0),
                b"AMOUNT" => amount_kg = read_text(reader)?.parse().unwrap_or(0.0),
                b"ADD_AFTER_BOIL" => {
                    add_after_boil = Some(read_text(reader)?.eq_ignore_ascii_case("true"))
                }
                _ => skip_element(reader, e.name().as_ref())?,
            },
            Event::End(e) if e.name().as_ref() == b"FERMENTABLE" => break,
            Event::Eof => return Err("Unexpected EOF inside <FERMENTABLE>".into()),
            _ => {}
        }
    }

    Ok(CreateFermentableAdditionInput {
        fermentable_id: None,
        name,
        type_,
        yield_pct,
        color_lovibond,
        amount_kg,
        add_after_boil,
    })
}

fn parse_hop(reader: &mut Reader<&[u8]>) -> Result<CreateHopAdditionInput, String> {
    let mut name = String::new();
    let mut alpha_pct = 5.0f64;
    let mut form = None;
    let mut amount_kg = 0.0f64;
    let mut use_ = String::from("boil");
    let mut time_min = 60.0f64;

    loop {
        match reader.read_event().map_err(|e| e.to_string())? {
            Event::Start(e) => match e.name().as_ref() {
                b"NAME" => name = read_text(reader)?,
                b"ALPHA" => alpha_pct = read_text(reader)?.parse().unwrap_or(5.0),
                b"FORM" => form = Some(read_text(reader)?),
                b"AMOUNT" => amount_kg = read_text(reader)?.parse().unwrap_or(0.0),
                b"USE" => use_ = read_text(reader)?,
                b"TIME" => time_min = read_text(reader)?.parse().unwrap_or(60.0),
                _ => skip_element(reader, e.name().as_ref())?,
            },
            Event::End(e) if e.name().as_ref() == b"HOP" => break,
            Event::Eof => return Err("Unexpected EOF inside <HOP>".into()),
            _ => {}
        }
    }

    Ok(CreateHopAdditionInput {
        hop_id: None,
        name,
        alpha_pct,
        form,
        amount_kg,
        use_,
        time_min,
        hopstand_temp_c: None,
    })
}

fn parse_yeast(reader: &mut Reader<&[u8]>) -> Result<CreateYeastAdditionInput, String> {
    let mut name = String::new();
    let mut type_ = String::from("ale");
    let mut form = String::from("liquid");
    let mut laboratory = None;
    let mut product_id = None;
    let mut attenuation_pct = None;
    let mut amount = None;
    let mut amount_is_weight = None;
    let mut add_to_secondary = None;

    loop {
        match reader.read_event().map_err(|e| e.to_string())? {
            Event::Start(e) => match e.name().as_ref() {
                b"NAME" => name = read_text(reader)?,
                b"TYPE" => type_ = read_text(reader)?,
                b"FORM" => form = read_text(reader)?,
                b"LABORATORY" => laboratory = Some(read_text(reader)?),
                b"PRODUCT_ID" => product_id = Some(read_text(reader)?),
                b"ATTENUATION" => attenuation_pct = read_text(reader)?.parse().ok(),
                b"AMOUNT" => amount = read_text(reader)?.parse().ok(),
                b"AMOUNT_IS_WEIGHT" => {
                    amount_is_weight = Some(read_text(reader)?.eq_ignore_ascii_case("true"))
                }
                b"ADD_TO_SECONDARY" => {
                    add_to_secondary = Some(read_text(reader)?.eq_ignore_ascii_case("true"))
                }
                _ => skip_element(reader, e.name().as_ref())?,
            },
            Event::End(e) if e.name().as_ref() == b"YEAST" => break,
            Event::Eof => return Err("Unexpected EOF inside <YEAST>".into()),
            _ => {}
        }
    }

    Ok(CreateYeastAdditionInput {
        yeast_id: None,
        name,
        type_,
        form,
        laboratory,
        product_id,
        attenuation_pct,
        amount,
        amount_is_weight,
        add_to_secondary,
        times_cultured: None,
    })
}

fn parse_misc(reader: &mut Reader<&[u8]>) -> Result<CreateMiscAdditionInput, String> {
    let mut name = String::new();
    let mut type_ = String::from("other");
    let mut use_ = String::from("boil");
    let mut amount = 0.0f64;
    let mut amount_is_weight = None;
    let mut time_min = 0.0f64;

    loop {
        match reader.read_event().map_err(|e| e.to_string())? {
            Event::Start(e) => match e.name().as_ref() {
                b"NAME" => name = read_text(reader)?,
                b"TYPE" => type_ = read_text(reader)?,
                b"USE" => use_ = read_text(reader)?,
                b"AMOUNT" => amount = read_text(reader)?.parse().unwrap_or(0.0),
                b"AMOUNT_IS_WEIGHT" => {
                    amount_is_weight = Some(read_text(reader)?.eq_ignore_ascii_case("true"))
                }
                b"TIME" => time_min = read_text(reader)?.parse().unwrap_or(0.0),
                _ => skip_element(reader, e.name().as_ref())?,
            },
            Event::End(e) if e.name().as_ref() == b"MISC" => break,
            Event::Eof => return Err("Unexpected EOF inside <MISC>".into()),
            _ => {}
        }
    }

    Ok(CreateMiscAdditionInput {
        misc_id: None,
        name,
        type_,
        use_,
        amount,
        amount_is_weight,
        time_min,
    })
}

/// Read the text content of the current element (already past the opening tag).
fn read_text(reader: &mut Reader<&[u8]>) -> Result<String, String> {
    let mut text = String::new();
    loop {
        match reader.read_event().map_err(|e| e.to_string())? {
            Event::Text(e) => text = e.unescape().map_err(|e| e.to_string())?.into_owned(),
            Event::End(_) => break,
            Event::Eof => return Err("Unexpected EOF reading element text".into()),
            _ => {}
        }
    }
    Ok(text)
}

/// Skip all events until the matching end tag for `tag_name`.
fn skip_element(reader: &mut Reader<&[u8]>, tag_name: &[u8]) -> Result<(), String> {
    let mut depth = 1usize;
    loop {
        match reader.read_event().map_err(|e| e.to_string())? {
            Event::Start(_) => depth += 1,
            Event::End(_) => {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            Event::Eof => {
                return Err(format!(
                    "Unexpected EOF skipping <{}>",
                    String::from_utf8_lossy(tag_name)
                ))
            }
            _ => {}
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn create_recipes_from_beerxml(
    state: State<'_, AppState>,
    xml: String,
) -> Result<Vec<RecipeSummary>, String> {
    let parsed = parse_beerxml(&xml)?;
    if parsed.is_empty() {
        return Err("No <RECIPE> elements found in XML".into());
    }

    let recipe_repo = RecipeRepository::new(&state.db);
    let fermentable_repo = FermentableRepository::new(&state.db);
    let hop_repo = HopRepository::new(&state.db);
    let yeast_repo = YeastRepository::new(&state.db);
    let misc_repo = MiscRepository::new(&state.db);

    let mut imported_ids = Vec::new();

    for p in parsed {
        let recipe = recipe_repo
            .create(CreateRecipeInput {
                name: p.name,
                type_: Some(p.type_),
                batch_size_l: Some(p.batch_size_l),
                boil_size_l: Some(p.boil_size_l),
                boil_time_min: Some(p.boil_time_min),
                ..Default::default()
            })
            .await
            .map_err(|e| e.to_string())?;

        for f in p.fermentables {
            fermentable_repo
                .create(&recipe.id, f)
                .await
                .map_err(|e| e.to_string())?;
        }
        for h in p.hops {
            hop_repo
                .create(&recipe.id, h)
                .await
                .map_err(|e| e.to_string())?;
        }
        for y in p.yeasts {
            yeast_repo
                .create(&recipe.id, y)
                .await
                .map_err(|e| e.to_string())?;
        }
        for m in p.miscs {
            misc_repo
                .create(&recipe.id, m)
                .await
                .map_err(|e| e.to_string())?;
        }

        imported_ids.push(recipe.id);
    }

    let all = recipe_repo.list().await.map_err(|e| e.to_string())?;
    Ok(all
        .into_iter()
        .filter(|r| imported_ids.contains(&r.id))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SINGLE_RECIPE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<RECIPES>
  <RECIPE>
    <NAME>Pale Ale</NAME>
    <TYPE>All Grain</TYPE>
    <BATCH_SIZE>23.0</BATCH_SIZE>
    <BOIL_SIZE>27.0</BOIL_SIZE>
    <BOIL_TIME>60</BOIL_TIME>
    <FERMENTABLES>
      <FERMENTABLE>
        <NAME>Pale Malt</NAME>
        <TYPE>Grain</TYPE>
        <AMOUNT>4.5</AMOUNT>
        <YIELD>78.0</YIELD>
        <COLOR>1.8</COLOR>
        <ADD_AFTER_BOIL>FALSE</ADD_AFTER_BOIL>
      </FERMENTABLE>
    </FERMENTABLES>
    <HOPS>
      <HOP>
        <NAME>Cascade</NAME>
        <ALPHA>5.5</ALPHA>
        <AMOUNT>0.028</AMOUNT>
        <USE>Boil</USE>
        <TIME>60</TIME>
        <FORM>Pellet</FORM>
      </HOP>
    </HOPS>
    <YEASTS>
      <YEAST>
        <NAME>US-05</NAME>
        <TYPE>Ale</TYPE>
        <FORM>Dry</FORM>
        <LABORATORY>Fermentis</LABORATORY>
        <ATTENUATION>77.0</ATTENUATION>
      </YEAST>
    </YEASTS>
    <MISCS>
      <MISC>
        <NAME>Irish Moss</NAME>
        <TYPE>Fining</TYPE>
        <USE>Boil</USE>
        <AMOUNT>0.005</AMOUNT>
        <TIME>15</TIME>
      </MISC>
    </MISCS>
  </RECIPE>
</RECIPES>"#;

    #[test]
    fn test_parse_single_recipe() {
        let recipes = parse_beerxml(SINGLE_RECIPE).unwrap();
        assert_eq!(recipes.len(), 1);
        let r = &recipes[0];
        assert_eq!(r.name, "Pale Ale");
        assert_eq!(r.batch_size_l, 23.0);
        assert_eq!(r.fermentables.len(), 1);
        assert_eq!(r.fermentables[0].name, "Pale Malt");
        assert_eq!(r.fermentables[0].amount_kg, 4.5);
        assert_eq!(r.fermentables[0].add_after_boil, Some(false));
        assert_eq!(r.hops.len(), 1);
        assert_eq!(r.hops[0].name, "Cascade");
        assert_eq!(r.hops[0].alpha_pct, 5.5);
        assert_eq!(r.hops[0].form, Some("Pellet".into()));
        assert_eq!(r.yeasts.len(), 1);
        assert_eq!(r.yeasts[0].name, "US-05");
        assert_eq!(r.yeasts[0].attenuation_pct, Some(77.0));
        assert_eq!(r.yeasts[0].laboratory, Some("Fermentis".into()));
        assert_eq!(r.miscs.len(), 1);
        assert_eq!(r.miscs[0].name, "Irish Moss");
        assert_eq!(r.miscs[0].time_min, 15.0);
    }

    #[test]
    fn test_parse_multiple_recipes() {
        let xml = format!(
            "<RECIPES>{0}{0}</RECIPES>",
            &SINGLE_RECIPE[SINGLE_RECIPE.find("<RECIPE>").unwrap()
                ..SINGLE_RECIPE.rfind("</RECIPE>").unwrap() + 9]
        );
        let recipes = parse_beerxml(&xml).unwrap();
        assert_eq!(recipes.len(), 2);
    }

    #[test]
    fn test_parse_empty_returns_empty() {
        let recipes = parse_beerxml("<RECIPES></RECIPES>").unwrap();
        assert!(recipes.is_empty());
    }

    #[test]
    fn test_parse_malformed_xml_returns_error() {
        assert!(parse_beerxml("<RECIPES><RECIPE><NAME>Oops</NAME>").is_err());
    }

    #[test]
    fn test_build_recipe_beerxml_escapes_special_chars() {
        use crate::models::{Recipe, RecipeSource};

        let recipe = Recipe {
            id: "r2".to_string(),
            name: "Oats & Honey <Wheat>".to_string(),
            type_: "all_grain".to_string(),
            batch_size_l: 20.0,
            boil_size_l: 25.0,
            boil_time_min: 60.0,
            brewer: Some("Tom & Jerry".to_string()),
            efficiency_pct: Some(72.0),
            source: RecipeSource::User,
            fermentation_stages: 1,
            forced_carbonation: false,
            created_at: 0,
            updated_at: 0,
            fermentables: vec![],
            hops: vec![],
            yeasts: vec![],
            miscs: vec![],
            waters: vec![],
            water_adjustments: vec![],
            age_days: None,
            age_temp_c: None,
            asst_brewer: None,
            carbonation_temp_c: None,
            carbonation_vols: None,
            date: None,
            equipment_profile: None,
            equipment_profile_id: None,
            fg: None,
            hopstand_temp_c: None,
            keg_priming_factor: None,
            mash: None,
            mash_water_id: None,
            notes: None,
            og: None,
            primary_age_days: None,
            primary_temp_c: None,
            priming_sugar_equiv: None,
            priming_sugar_name: None,
            secondary_age_days: None,
            secondary_temp_c: None,
            sparge_water_id: None,
            style: None,
            style_id: None,
            taste_notes: None,
            taste_rating: None,
            tertiary_age_days: None,
            tertiary_temp_c: None,
            image_path: None,
        };

        let xml = build_recipe_beerxml(&recipe).unwrap();
        assert!(xml.contains("<NAME>Oats &amp; Honey &lt;Wheat&gt;</NAME>"));
        assert!(xml.contains("<BREWER>Tom &amp; Jerry</BREWER>"));
        assert!(!xml.contains("<NAME>Oats & Honey"));
    }

    #[test]
    fn test_build_recipe_beerxml_contains_recipe_fields() {
        use crate::models::{Recipe, RecipeSource};

        let recipe = Recipe {
            id: "r1".to_string(),
            name: "Pale Ale".to_string(),
            type_: "all_grain".to_string(),
            batch_size_l: 23.0,
            boil_size_l: 27.0,
            boil_time_min: 60.0,
            brewer: Some("Test Brewer".to_string()),
            efficiency_pct: Some(75.0),
            source: RecipeSource::User,
            fermentation_stages: 1,
            forced_carbonation: false,
            created_at: 0,
            updated_at: 0,
            fermentables: vec![],
            hops: vec![],
            yeasts: vec![],
            miscs: vec![],
            waters: vec![],
            water_adjustments: vec![],
            // All Option fields:
            age_days: None,
            age_temp_c: None,
            asst_brewer: None,
            carbonation_temp_c: None,
            carbonation_vols: None,
            date: None,
            equipment_profile: None,
            equipment_profile_id: None,
            fg: None,
            hopstand_temp_c: None,
            keg_priming_factor: None,
            mash: None,
            mash_water_id: None,
            notes: None,
            og: None,
            primary_age_days: None,
            primary_temp_c: None,
            priming_sugar_equiv: None,
            priming_sugar_name: None,
            secondary_age_days: None,
            secondary_temp_c: None,
            sparge_water_id: None,
            style: None,
            style_id: None,
            taste_notes: None,
            taste_rating: None,
            tertiary_age_days: None,
            tertiary_temp_c: None,
            image_path: None,
        };

        let xml = build_recipe_beerxml(&recipe).unwrap();
        assert!(xml.starts_with("<?xml version=\"1.0\""));
        assert!(xml.contains("<NAME>Pale Ale</NAME>"));
        assert!(xml.contains("<BATCH_SIZE>23.0</BATCH_SIZE>"));
        assert!(xml.contains("<BOIL_TIME>60</BOIL_TIME>"));
        assert!(xml.contains("<BREWER>Test Brewer</BREWER>"));
        assert!(xml.contains("<EFFICIENCY>75.0</EFFICIENCY>"));
    }
}
