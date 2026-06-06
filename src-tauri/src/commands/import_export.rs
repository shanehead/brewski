use quick_xml::events::{BytesDecl, BytesText, Event};
use quick_xml::reader::Reader;
use quick_xml::Writer;
use tauri::State;

use crate::error::AppError;
use crate::models::{
    CreateFermentableAdditionInput, CreateHopAdditionInput, CreateMashStepInput,
    CreateMiscAdditionInput, CreateRecipeInput, CreateYeastAdditionInput, Recipe, RecipeSummary,
    UpdateMashInput,
};
use crate::repositories::fermentable::FermentableRepository;
use crate::repositories::hop::HopRepository;
use crate::repositories::mash::MashRepository;
use crate::repositories::misc::MiscRepository;
use crate::repositories::recipe::RecipeRepository;
use crate::repositories::yeast::YeastRepository;
use crate::AppState;

fn bool_str(b: bool) -> &'static str {
    if b {
        "TRUE"
    } else {
        "FALSE"
    }
}

fn build_recipe_beerxml(recipe: &Recipe) -> Result<String, quick_xml::Error> {
    let mut writer = Writer::new_with_indent(Vec::new(), b' ', 2);
    writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))?;

    let brewer = recipe.brewer.as_deref().unwrap_or("");
    let efficiency = format!("{:.1}", recipe.efficiency_pct.unwrap_or(72.0));

    type E = std::io::Error;

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
                                        w.create_element("ADD_AFTER_BOIL").write_text_content(
                                            BytesText::new(bool_str(f.add_after_boil)),
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
                                        w.create_element("AMOUNT_IS_WEIGHT").write_text_content(
                                            BytesText::new(bool_str(y.amount_is_weight)),
                                        )?;
                                        if let Some(lab) = &y.laboratory {
                                            w.create_element("LABORATORY")
                                                .write_text_content(BytesText::new(lab))?;
                                        }
                                        if let Some(pid) = &y.product_id {
                                            w.create_element("PRODUCT_ID")
                                                .write_text_content(BytesText::new(pid))?;
                                        }
                                        if let Some(att) = y.attenuation_pct {
                                            w.create_element("ATTENUATION").write_text_content(
                                                BytesText::new(&format!("{:.1}", att)),
                                            )?;
                                        }
                                        w.create_element("ADD_TO_SECONDARY").write_text_content(
                                            BytesText::new(bool_str(y.add_to_secondary)),
                                        )?;
                                        Ok(())
                                    },
                                )?;
                            }
                            Ok(())
                        })?;

                    w.create_element("MISCS")
                        .write_inner_content(|w| -> Result<(), E> {
                            for m in &recipe.miscs {
                                w.create_element("MISC").write_inner_content(
                                    |w| -> Result<(), E> {
                                        w.create_element("NAME")
                                            .write_text_content(BytesText::new(&m.name))?;
                                        w.create_element("TYPE")
                                            .write_text_content(BytesText::new(&m.type_))?;
                                        w.create_element("USE")
                                            .write_text_content(BytesText::new(&m.use_))?;
                                        w.create_element("AMOUNT").write_text_content(
                                            BytesText::new(&format!("{:.4}", m.amount)),
                                        )?;
                                        w.create_element("AMOUNT_IS_WEIGHT").write_text_content(
                                            BytesText::new(bool_str(m.amount_is_weight)),
                                        )?;
                                        w.create_element("TIME").write_text_content(
                                            BytesText::new(&format!("{:.0}", m.time_min)),
                                        )?;
                                        Ok(())
                                    },
                                )?;
                            }
                            Ok(())
                        })?;

                    if let Some(mash) = &recipe.mash {
                        w.create_element("MASH")
                            .write_inner_content(|w| -> Result<(), E> {
                                w.create_element("NAME")
                                    .write_text_content(BytesText::new(&mash.name))?;
                                w.create_element("GRAIN_TEMP").write_text_content(
                                    BytesText::new(&format!("{:.1}", mash.grain_temp_c)),
                                )?;
                                if let Some(sparge) = mash.sparge_temp_c {
                                    w.create_element("SPARGE_TEMP").write_text_content(
                                        BytesText::new(&format!("{:.1}", sparge)),
                                    )?;
                                }
                                if let Some(ph) = mash.ph {
                                    w.create_element("PH").write_text_content(BytesText::new(
                                        &format!("{:.2}", ph),
                                    ))?;
                                }
                                w.create_element("MASH_STEPS").write_inner_content(
                                    |w| -> Result<(), E> {
                                        for step in &mash.steps {
                                            w.create_element("MASH_STEP").write_inner_content(
                                                |w| -> Result<(), E> {
                                                    w.create_element("NAME").write_text_content(
                                                        BytesText::new(&step.name),
                                                    )?;
                                                    w.create_element("TYPE").write_text_content(
                                                        BytesText::new(&step.type_),
                                                    )?;
                                                    w.create_element("STEP_TEMP")
                                                        .write_text_content(BytesText::new(
                                                            &format!("{:.1}", step.step_temp_c),
                                                        ))?;
                                                    w.create_element("STEP_TIME")
                                                        .write_text_content(BytesText::new(
                                                            &format!("{}", step.step_time_min),
                                                        ))?;
                                                    if let Some(infuse) = step.infuse_amount_l {
                                                        w.create_element("INFUSE_AMOUNT")
                                                            .write_text_content(BytesText::new(
                                                                &format!("{:.3}", infuse),
                                                            ))?;
                                                    }
                                                    if let Some(ramp) = step.ramp_time_min {
                                                        w.create_element("RAMP_TIME")
                                                            .write_text_content(BytesText::new(
                                                                &format!("{}", ramp),
                                                            ))?;
                                                    }
                                                    if let Some(end) = step.end_temp_c {
                                                        w.create_element("END_TEMP")
                                                            .write_text_content(BytesText::new(
                                                                &format!("{:.1}", end),
                                                            ))?;
                                                    }
                                                    Ok(())
                                                },
                                            )?;
                                        }
                                        Ok(())
                                    },
                                )?;
                                Ok(())
                            })?;
                    }

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
) -> Result<String, AppError> {
    let recipe = RecipeRepository::new(&state.db).get(&recipe_id).await?;
    build_recipe_beerxml(&recipe).map_err(|e| AppError::Internal(e.to_string()))
}

#[tauri::command]
pub async fn write_recipe_beerxml(
    state: State<'_, AppState>,
    recipe_id: String,
    path: String,
) -> Result<(), AppError> {
    let recipe = RecipeRepository::new(&state.db).get(&recipe_id).await?;
    let xml = build_recipe_beerxml(&recipe).map_err(|e| AppError::Internal(e.to_string()))?;
    std::fs::write(&path, xml).map_err(|e| AppError::Internal(e.to_string()))
}

struct ParsedMashStep {
    name: String,
    type_: String,
    step_temp_c: f64,
    step_time_min: i64,
    infuse_amount_l: Option<f64>,
    ramp_time_min: Option<i64>,
    end_temp_c: Option<f64>,
}

struct ParsedMash {
    name: String,
    grain_temp_c: f64,
    sparge_temp_c: Option<f64>,
    ph: Option<f64>,
    steps: Vec<ParsedMashStep>,
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
    mash: Option<ParsedMash>,
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
    let mut mash = None;

    loop {
        match reader.read_event().map_err(|e| e.to_string())? {
            Event::Start(e) => match e.name().as_ref() {
                b"NAME" => name = read_text(reader)?,
                b"TYPE" => type_ = read_text(reader)?,
                b"BATCH_SIZE" => batch_size_l = read_text(reader)?.parse().unwrap_or(23.0),
                b"BOIL_SIZE" => boil_size_l = read_text(reader)?.parse().unwrap_or(27.0),
                b"BOIL_TIME" => boil_time_min = read_text(reader)?.parse().unwrap_or(60.0),
                // Container elements — just descend into them; children are matched below
                b"FERMENTABLES" | b"HOPS" | b"YEASTS" | b"MISCS" | b"MASH_STEPS" => {}
                b"FERMENTABLE" => fermentables.push(parse_fermentable(reader)?),
                b"HOP" => hops.push(parse_hop(reader)?),
                b"YEAST" => yeasts.push(parse_yeast(reader)?),
                b"MISC" => miscs.push(parse_misc(reader)?),
                b"MASH" => mash = Some(parse_mash(reader)?),
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
        mash,
    })
}

fn parse_mash(reader: &mut Reader<&[u8]>) -> Result<ParsedMash, String> {
    let mut name = String::from("Mash");
    let mut grain_temp_c = 20.0f64;
    let mut sparge_temp_c = None;
    let mut ph = None;
    let mut steps = Vec::new();

    loop {
        match reader.read_event().map_err(|e| e.to_string())? {
            Event::Start(e) => match e.name().as_ref() {
                b"NAME" => name = read_text(reader)?,
                b"GRAIN_TEMP" => grain_temp_c = read_text(reader)?.parse().unwrap_or(20.0),
                b"SPARGE_TEMP" => sparge_temp_c = read_text(reader)?.parse().ok(),
                b"PH" => ph = read_text(reader)?.parse().ok(),
                b"MASH_STEPS" => {}
                b"MASH_STEP" => steps.push(parse_mash_step(reader)?),
                _ => skip_element(reader, e.name().as_ref())?,
            },
            Event::End(e) if e.name().as_ref() == b"MASH" => break,
            Event::Eof => return Err("Unexpected EOF inside <MASH>".into()),
            _ => {}
        }
    }

    Ok(ParsedMash {
        name,
        grain_temp_c,
        sparge_temp_c,
        ph,
        steps,
    })
}

fn parse_mash_step(reader: &mut Reader<&[u8]>) -> Result<ParsedMashStep, String> {
    let mut name = String::from("Mash Step");
    let mut type_ = String::from("Infusion");
    let mut step_temp_c = 68.0f64;
    let mut step_time_min = 60i64;
    let mut infuse_amount_l = None;
    let mut ramp_time_min = None;
    let mut end_temp_c = None;

    loop {
        match reader.read_event().map_err(|e| e.to_string())? {
            Event::Start(e) => match e.name().as_ref() {
                b"NAME" => name = read_text(reader)?,
                b"TYPE" => type_ = read_text(reader)?,
                b"STEP_TEMP" => step_temp_c = read_text(reader)?.parse().unwrap_or(68.0),
                b"STEP_TIME" => step_time_min = read_text(reader)?.parse().unwrap_or(60),
                b"INFUSE_AMOUNT" => infuse_amount_l = read_text(reader)?.parse().ok(),
                b"RAMP_TIME" => ramp_time_min = read_text(reader)?.parse().ok(),
                b"END_TEMP" => end_temp_c = read_text(reader)?.parse().ok(),
                _ => skip_element(reader, e.name().as_ref())?,
            },
            Event::End(e) if e.name().as_ref() == b"MASH_STEP" => break,
            Event::Eof => return Err("Unexpected EOF inside <MASH_STEP>".into()),
            _ => {}
        }
    }

    Ok(ParsedMashStep {
        name,
        type_,
        step_temp_c,
        step_time_min,
        infuse_amount_l,
        ramp_time_min,
        end_temp_c,
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
            Event::Text(e) => {
                let decoded = e.decode().map_err(|e| e.to_string())?;
                text = quick_xml::escape::unescape(&decoded)
                    .map_err(|e| e.to_string())?
                    .into_owned();
            }
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
) -> Result<Vec<RecipeSummary>, AppError> {
    let parsed = parse_beerxml(&xml).map_err(AppError::Conversion)?;
    if parsed.is_empty() {
        return Err(AppError::Conversion(
            "No <RECIPE> elements found in XML".into(),
        ));
    }

    let recipe_repo = RecipeRepository::new(&state.db);
    let fermentable_repo = FermentableRepository::new(&state.db);
    let hop_repo = HopRepository::new(&state.db);
    let yeast_repo = YeastRepository::new(&state.db);
    let misc_repo = MiscRepository::new(&state.db);
    let mash_repo = MashRepository::new(&state.db);

    let mut summaries = Vec::new();

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
            .await?;

        for f in p.fermentables {
            fermentable_repo.create(&recipe.id, f).await?;
        }
        for h in p.hops {
            hop_repo.create(&recipe.id, h).await?;
        }
        for y in p.yeasts {
            yeast_repo.create(&recipe.id, y).await?;
        }
        for m in p.miscs {
            misc_repo.create(&recipe.id, m).await?;
        }
        if let Some(pm) = p.mash {
            let mash = mash_repo
                .upsert_for_recipe(
                    &recipe.id,
                    UpdateMashInput {
                        name: Some(pm.name),
                        grain_temp_c: Some(pm.grain_temp_c),
                        sparge_temp_c: pm.sparge_temp_c,
                        ph: pm.ph,
                        ..Default::default()
                    },
                )
                .await?;
            for step in pm.steps {
                mash_repo
                    .create_step(
                        &mash.id,
                        CreateMashStepInput {
                            name: step.name,
                            type_: Some(step.type_),
                            step_temp_c: step.step_temp_c,
                            step_time_min: step.step_time_min,
                            infuse_amount_l: step.infuse_amount_l,
                            ramp_time_min: step.ramp_time_min,
                            end_temp_c: step.end_temp_c,
                        },
                    )
                    .await?;
            }
        }

        summaries.push(RecipeSummary::try_from(recipe)?);
    }

    Ok(summaries)
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

    #[test]
    fn test_export_import_round_trip_preserves_all_parsed_fields() {
        use crate::models::{
            Recipe, RecipeAdditionFermentable, RecipeAdditionMisc, RecipeAdditionYeast,
            RecipeSource,
        };

        let recipe = Recipe {
            id: "r1".to_string(),
            name: "Round Trip Ale".to_string(),
            type_: "all_grain".to_string(),
            batch_size_l: 23.0,
            boil_size_l: 27.0,
            boil_time_min: 60.0,
            brewer: Some("Tester".to_string()),
            efficiency_pct: Some(72.0),
            source: RecipeSource::User,
            fermentation_stages: 1,
            forced_carbonation: false,
            created_at: 0,
            updated_at: 0,
            fermentables: vec![RecipeAdditionFermentable {
                add_after_boil: true,
                addition_order: 0,
                amount_kg: 4.5,
                color_lovibond: 1.8,
                fermentable_id: None,
                id: "f1".to_string(),
                name: "Pale Malt".to_string(),
                recipe_id: "r1".to_string(),
                type_: "grain".to_string(),
                yield_pct: 78.0,
            }],
            hops: vec![],
            yeasts: vec![RecipeAdditionYeast {
                add_to_secondary: false,
                amount: Some(0.011),
                amount_is_weight: true,
                attenuation_pct: Some(77.0),
                form: "dry".to_string(),
                id: "y1".to_string(),
                laboratory: Some("Fermentis".to_string()),
                name: "US-05".to_string(),
                product_id: Some("US-05".to_string()),
                recipe_id: "r1".to_string(),
                times_cultured: 0,
                type_: "ale".to_string(),
                yeast_id: None,
            }],
            miscs: vec![RecipeAdditionMisc {
                addition_order: 0,
                amount: 0.005,
                amount_is_weight: true,
                id: "m1".to_string(),
                misc_id: None,
                name: "Irish Moss".to_string(),
                recipe_id: "r1".to_string(),
                time_min: 15.0,
                type_: "fining".to_string(),
                use_: "boil".to_string(),
            }],
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
        let parsed = parse_beerxml(&xml).unwrap();
        assert_eq!(parsed.len(), 1);
        let r = &parsed[0];

        // Fermentable: add_after_boil must survive
        assert_eq!(r.fermentables.len(), 1);
        assert_eq!(r.fermentables[0].add_after_boil, Some(true));

        // Misc: entire section must survive
        assert_eq!(r.miscs.len(), 1);
        assert_eq!(r.miscs[0].name, "Irish Moss");
        assert_eq!(r.miscs[0].type_, "fining");
        assert_eq!(r.miscs[0].use_, "boil");
        assert!((r.miscs[0].amount - 0.005).abs() < 1e-9);
        assert_eq!(r.miscs[0].amount_is_weight, Some(true));
        assert!((r.miscs[0].time_min - 15.0).abs() < 1e-9);

        // Yeast: lab/product/attenuation/amount_is_weight/add_to_secondary must survive
        assert_eq!(r.yeasts.len(), 1);
        assert_eq!(r.yeasts[0].laboratory, Some("Fermentis".to_string()));
        assert_eq!(r.yeasts[0].product_id, Some("US-05".to_string()));
        assert_eq!(r.yeasts[0].attenuation_pct, Some(77.0));
        assert_eq!(r.yeasts[0].amount_is_weight, Some(true));
        assert_eq!(r.yeasts[0].add_to_secondary, Some(false));
        assert!((r.yeasts[0].amount.unwrap() - 0.011).abs() < 1e-9);
    }

    #[test]
    fn test_export_import_round_trip_preserves_mash() {
        use crate::models::{Mash, MashStep, Recipe, RecipeSource};

        let recipe = Recipe {
            id: "r1".to_string(),
            name: "Mash Test".to_string(),
            type_: "all_grain".to_string(),
            batch_size_l: 23.0,
            boil_size_l: 27.0,
            boil_time_min: 60.0,
            brewer: None,
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
            mash: Some(Mash {
                id: "mash1".to_string(),
                recipe_id: "r1".to_string(),
                name: "Single Infusion".to_string(),
                grain_temp_c: 18.0,
                sparge_temp_c: Some(76.0),
                ph: Some(5.4),
                tun_temp_c: None,
                tun_weight_kg: None,
                tun_specific_heat: None,
                equip_adjust: false,
                ratio_l_per_kg: None,
                notes: None,
                steps: vec![MashStep {
                    id: "s1".to_string(),
                    mash_id: "mash1".to_string(),
                    name: "Mash In".to_string(),
                    type_: "Infusion".to_string(),
                    step_temp_c: 67.0,
                    step_time_min: 60,
                    infuse_amount_l: Some(15.5),
                    ramp_time_min: Some(5),
                    end_temp_c: Some(66.0),
                    step_order: 0,
                }],
            }),
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
        assert!(xml.contains("<MASH>"));
        assert!(xml.contains("<GRAIN_TEMP>18.0</GRAIN_TEMP>"));
        assert!(xml.contains("<SPARGE_TEMP>76.0</SPARGE_TEMP>"));
        assert!(xml.contains("<PH>5.40</PH>"));
        assert!(xml.contains("<MASH_STEP>"));
        assert!(xml.contains("<STEP_TEMP>67.0</STEP_TEMP>"));
        assert!(xml.contains("<INFUSE_AMOUNT>15.500</INFUSE_AMOUNT>"));
        assert!(xml.contains("<RAMP_TIME>5</RAMP_TIME>"));
        assert!(xml.contains("<END_TEMP>66.0</END_TEMP>"));

        let parsed = parse_beerxml(&xml).unwrap();
        let r = &parsed[0];
        let m = r.mash.as_ref().unwrap();
        assert_eq!(m.name, "Single Infusion");
        assert!((m.grain_temp_c - 18.0).abs() < 1e-9);
        assert_eq!(m.sparge_temp_c, Some(76.0));
        assert!((m.ph.unwrap() - 5.4).abs() < 1e-9);
        assert_eq!(m.steps.len(), 1);
        assert_eq!(m.steps[0].name, "Mash In");
        assert_eq!(m.steps[0].type_, "Infusion");
        assert!((m.steps[0].step_temp_c - 67.0).abs() < 1e-9);
        assert_eq!(m.steps[0].step_time_min, 60);
        assert!((m.steps[0].infuse_amount_l.unwrap() - 15.5).abs() < 1e-3);
        assert_eq!(m.steps[0].ramp_time_min, Some(5));
        assert!((m.steps[0].end_temp_c.unwrap() - 66.0).abs() < 1e-9);
    }
}
