use tauri::State;
use crate::AppState;
use crate::models::{CreateFermentableAdditionInput, CreateHopAdditionInput, CreateRecipeInput, RecipeSummary};
use crate::repositories::fermentable::FermentableRepository;
use crate::repositories::hop::HopRepository;
use crate::repositories::recipe::RecipeRepository;

#[tauri::command]
pub async fn get_recipe_beerxml(state: State<'_, AppState>, recipe_id: String) -> Result<String, String> {
    let recipe = RecipeRepository::new(&state.db).get(&recipe_id).await.map_err(|e| e.to_string())?;

    let style_block = recipe.style.as_ref().map(|s| format!(
        "    <STYLE>\n      <NAME>{}</NAME>\n      <CATEGORY>{}</CATEGORY>\n      <STYLE_GUIDE>{}</STYLE_GUIDE>\n    </STYLE>",
        s.name, s.category, s.style_guide
    )).unwrap_or_default();

    let fermentables: String = recipe.fermentables.iter().map(|f| format!(
        "      <FERMENTABLE>\n        <NAME>{}</NAME>\n        <AMOUNT>{:.4}</AMOUNT>\n        <TYPE>{}</TYPE>\n        <YIELD>{:.1}</YIELD>\n        <COLOR>{:.1}</COLOR>\n      </FERMENTABLE>",
        f.name, f.amount_kg, f.type_, f.yield_pct, f.color_lovibond
    )).collect::<Vec<_>>().join("\n");

    let hops: String = recipe.hops.iter().map(|h| format!(
        "      <HOP>\n        <NAME>{}</NAME>\n        <AMOUNT>{:.5}</AMOUNT>\n        <ALPHA>{:.1}</ALPHA>\n        <USE>{}</USE>\n        <TIME>{:.0}</TIME>\n        <FORM>{}</FORM>\n      </HOP>",
        h.name, h.amount_kg, h.alpha_pct, h.use_, h.time_min, h.form
    )).collect::<Vec<_>>().join("\n");

    let yeasts: String = recipe.yeasts.iter().map(|y| format!(
        "      <YEAST>\n        <NAME>{}</NAME>\n        <TYPE>{}</TYPE>\n        <FORM>{}</FORM>\n        <AMOUNT>{:.4}</AMOUNT>\n      </YEAST>",
        y.name, y.type_, y.form, y.amount.unwrap_or(0.0)
    )).collect::<Vec<_>>().join("\n");

    Ok(format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<RECIPES>\n  <RECIPE>\n    <NAME>{name}</NAME>\n    <VERSION>1</VERSION>\n    <TYPE>{type_}</TYPE>\n    <BREWER>{brewer}</BREWER>\n    <BATCH_SIZE>{batch_size:.1}</BATCH_SIZE>\n    <BOIL_SIZE>{boil_size:.1}</BOIL_SIZE>\n    <BOIL_TIME>{boil_time:.0}</BOIL_TIME>\n    <EFFICIENCY>{efficiency:.1}</EFFICIENCY>\n{style}\n    <FERMENTABLES>\n{fermentables}\n    </FERMENTABLES>\n    <HOPS>\n{hops}\n    </HOPS>\n    <YEASTS>\n{yeasts}\n    </YEASTS>\n  </RECIPE>\n</RECIPES>",
        name = recipe.name, type_ = recipe.type_,
        brewer = recipe.brewer.as_deref().unwrap_or(""),
        batch_size = recipe.batch_size_l, boil_size = recipe.boil_size_l,
        boil_time = recipe.boil_time_min, efficiency = recipe.efficiency_pct.unwrap_or(72.0),
        style = style_block, fermentables = fermentables, hops = hops, yeasts = yeasts,
    ))
}

#[tauri::command]
pub async fn create_recipes_from_beerxml(state: State<'_, AppState>, xml: String) -> Result<Vec<RecipeSummary>, String> {
    let recipes_start = xml.find("<RECIPE>").ok_or("No <RECIPE> found in XML")?;
    let recipes_end = xml.rfind("</RECIPE>").ok_or("No </RECIPE> found in XML")?;
    let recipe_xml = &xml[recipes_start..recipes_end + 9];

    let name = extract_tag(recipe_xml, "NAME").unwrap_or("Imported Recipe".to_string());
    let type_ = extract_tag(recipe_xml, "TYPE").unwrap_or("all_grain".to_string());
    let batch_size = extract_tag(recipe_xml, "BATCH_SIZE").and_then(|v| v.parse().ok()).unwrap_or(23.0);
    let boil_size = extract_tag(recipe_xml, "BOIL_SIZE").and_then(|v| v.parse().ok()).unwrap_or(27.0);
    let boil_time = extract_tag(recipe_xml, "BOIL_TIME").and_then(|v| v.parse().ok()).unwrap_or(60.0);

    let recipe_repo = RecipeRepository::new(&state.db);
    let fermentable_repo = FermentableRepository::new(&state.db);
    let hop_repo = HopRepository::new(&state.db);

    let recipe = recipe_repo.create(CreateRecipeInput {
        name, type_: Some(type_), batch_size_l: Some(batch_size),
        boil_size_l: Some(boil_size), boil_time_min: Some(boil_time),
        equipment_profile_id: None, source_id: None,
    }).await.map_err(|e| e.to_string())?;

    let ferm_xml = extract_between(&xml, "<FERMENTABLES>", "</FERMENTABLES>").unwrap_or_default();
    for ferm_block in split_tags(&ferm_xml, "FERMENTABLE") {
        let fermentable_name = extract_tag(&ferm_block, "NAME").unwrap_or_default();
        if fermentable_name.is_empty() { continue; }
        let _ = fermentable_repo.create(&recipe.id, CreateFermentableAdditionInput {
            fermentable_id: None, name: fermentable_name,
            type_: extract_tag(&ferm_block, "TYPE").unwrap_or("grain".to_string()),
            yield_pct: extract_tag(&ferm_block, "YIELD").and_then(|v| v.parse().ok()).unwrap_or(75.0),
            color_lovibond: extract_tag(&ferm_block, "COLOR").and_then(|v| v.parse().ok()).unwrap_or(2.0),
            amount_kg: extract_tag(&ferm_block, "AMOUNT").and_then(|v| v.parse().ok()).unwrap_or(0.0),
            add_after_boil: None,
        }).await;
    }

    let hops_xml = extract_between(&xml, "<HOPS>", "</HOPS>").unwrap_or_default();
    for hop_block in split_tags(&hops_xml, "HOP") {
        let hop_name = extract_tag(&hop_block, "NAME").unwrap_or_default();
        if hop_name.is_empty() { continue; }
        let _ = hop_repo.create(&recipe.id, CreateHopAdditionInput {
            hop_id: None, name: hop_name,
            alpha_pct: extract_tag(&hop_block, "ALPHA").and_then(|v| v.parse().ok()).unwrap_or(5.0),
            form: extract_tag(&hop_block, "FORM"),
            amount_kg: extract_tag(&hop_block, "AMOUNT").and_then(|v| v.parse().ok()).unwrap_or(0.0),
            use_: extract_tag(&hop_block, "USE").unwrap_or("boil".to_string()),
            time_min: extract_tag(&hop_block, "TIME").and_then(|v| v.parse().ok()).unwrap_or(60.0),
        }).await;
    }

    let summary = recipe_repo.list().await.map_err(|e| e.to_string())?
        .into_iter().filter(|r| r.id == recipe.id).collect();
    Ok(summary)
}

fn extract_tag(xml: &str, tag: &str) -> Option<String> {
    let open = format!("<{}>", tag);
    let close = format!("</{}>", tag);
    let start = xml.find(&open)? + open.len();
    let end = xml.find(&close)?;
    if end > start { Some(xml[start..end].trim().to_string()) } else { None }
}

fn extract_between(xml: &str, open: &str, close: &str) -> Option<String> {
    let start = xml.find(open)? + open.len();
    let end = xml.find(close)?;
    if end > start { Some(xml[start..end].to_string()) } else { None }
}

fn split_tags(xml: &str, tag: &str) -> Vec<String> {
    let open = format!("<{}>", tag);
    let close = format!("</{}>", tag);
    let mut results = Vec::new();
    let mut remaining = xml;
    while let Some(start) = remaining.find(&open) {
        let end = remaining.find(&close).unwrap_or(remaining.len());
        results.push(remaining[start..end + close.len()].to_string());
        remaining = &remaining[end + close.len()..];
    }
    results
}
