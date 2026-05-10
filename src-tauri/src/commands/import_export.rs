use quick_xml::events::Event;
use quick_xml::reader::Reader;
use tauri::State;

use crate::models::{
    CreateFermentableAdditionInput, CreateHopAdditionInput, CreateMiscAdditionInput,
    CreateRecipeInput, CreateYeastAdditionInput, RecipeSummary,
};
use crate::repositories::fermentable::FermentableRepository;
use crate::repositories::hop::HopRepository;
use crate::repositories::misc::MiscRepository;
use crate::repositories::recipe::RecipeRepository;
use crate::repositories::yeast::YeastRepository;
use crate::AppState;

#[tauri::command]
pub async fn get_recipe_beerxml(
    state: State<'_, AppState>,
    recipe_id: String,
) -> Result<String, String> {
    let recipe = RecipeRepository::new(&state.db)
        .get(&recipe_id)
        .await
        .map_err(|e| e.to_string())?;

    let style_block = recipe
        .style
        .as_ref()
        .map(|s| {
            format!(
                "    <STYLE>\n      <NAME>{}</NAME>\n      <CATEGORY>{}</CATEGORY>\n      <STYLE_GUIDE>{}</STYLE_GUIDE>\n    </STYLE>",
                s.name, s.category, s.style_guide
            )
        })
        .unwrap_or_default();

    let fermentables: String = recipe
        .fermentables
        .iter()
        .map(|f| {
            format!(
                "      <FERMENTABLE>\n        <NAME>{}</NAME>\n        <AMOUNT>{:.4}</AMOUNT>\n        <TYPE>{}</TYPE>\n        <YIELD>{:.1}</YIELD>\n        <COLOR>{:.1}</COLOR>\n      </FERMENTABLE>",
                f.name, f.amount_kg, f.type_, f.yield_pct, f.color_lovibond
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let hops: String = recipe
        .hops
        .iter()
        .map(|h| {
            format!(
                "      <HOP>\n        <NAME>{}</NAME>\n        <AMOUNT>{:.5}</AMOUNT>\n        <ALPHA>{:.1}</ALPHA>\n        <USE>{}</USE>\n        <TIME>{:.0}</TIME>\n        <FORM>{}</FORM>\n      </HOP>",
                h.name, h.amount_kg, h.alpha_pct, h.use_, h.time_min, h.form
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let yeasts: String = recipe
        .yeasts
        .iter()
        .map(|y| {
            format!(
                "      <YEAST>\n        <NAME>{}</NAME>\n        <TYPE>{}</TYPE>\n        <FORM>{}</FORM>\n        <AMOUNT>{:.4}</AMOUNT>\n      </YEAST>",
                y.name, y.type_, y.form, y.amount.unwrap_or(0.0)
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    Ok(format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<RECIPES>\n  <RECIPE>\n    <NAME>{name}</NAME>\n    <VERSION>1</VERSION>\n    <TYPE>{type_}</TYPE>\n    <BREWER>{brewer}</BREWER>\n    <BATCH_SIZE>{batch_size:.1}</BATCH_SIZE>\n    <BOIL_SIZE>{boil_size:.1}</BOIL_SIZE>\n    <BOIL_TIME>{boil_time:.0}</BOIL_TIME>\n    <EFFICIENCY>{efficiency:.1}</EFFICIENCY>\n{style}\n    <FERMENTABLES>\n{fermentables}\n    </FERMENTABLES>\n    <HOPS>\n{hops}\n    </HOPS>\n    <YEASTS>\n{yeasts}\n    </YEASTS>\n  </RECIPE>\n</RECIPES>",
        name = recipe.name,
        type_ = recipe.type_,
        brewer = recipe.brewer.as_deref().unwrap_or(""),
        batch_size = recipe.batch_size_l,
        boil_size = recipe.boil_size_l,
        boil_time = recipe.boil_time_min,
        efficiency = recipe.efficiency_pct.unwrap_or(72.0),
        style = style_block,
        fermentables = fermentables,
        hops = hops,
        yeasts = yeasts,
    ))
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
                equipment_profile_id: None,
                source_id: None,
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
}
