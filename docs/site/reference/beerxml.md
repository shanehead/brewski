# BeerXML

BeerXML is a standard file format for sharing homebrewing recipes between apps. Brewski supports BeerXML 1.0 for both import and export.

## Importing a recipe

Click **Import BeerXML** in the recipe list sidebar. Select a `.xml` file exported from another app and Brewski imports it.

### What comes in

| Data | Imported |
|---|---|
| Recipe name | Yes |
| Style | Yes |
| Batch size and boil time | Yes |
| Fermentables (name, yield, color) | Yes |
| Hops (name, alpha, use, time) | Yes |
| Yeast (name, attenuation) | Yes |
| Mash steps | Yes |
| Water chemistry | No |
| Equipment profile | No |
| Batch/brew day logs | No |

### What to check after import

**Equipment profile.** BeerXML carries equipment data, but Brewski uses your own profiles instead. After import, go to the recipe's **Overview** tab and pick your equipment profile from the dropdown.

**Water chemistry.** Water profile data isn't transferred. If the original recipe had mineral additions, re-enter them in the **Water** tab.

**Custom ingredients.** Brewski matches imported ingredients by name against its built-in library. If a name doesn't match, that ingredient becomes a custom entry with the data from the file. Check your ingredient list after import if anything looks unfamiliar.

### Compatible source apps

Brewski accepts BeerXML exports from:

- Brewfather
- BeerSmith
- BrewUnited
- BrewSmith Mobile
- Any BeerXML 1.0-compatible tool

## Exporting a recipe

Open a recipe and click **Export** (or find it in the recipe menu). Brewski generates a `.xml` file you can share or open in another app.

### What goes out

| Data | Exported |
|---|---|
| Recipe name | Yes |
| Style | Yes |
| Batch size | Yes |
| Fermentables | Yes |
| Hops | Yes |
| Yeast | Yes |
| Mash steps | Yes |
| Notes | Yes |
| Water chemistry | No |
| Batch/brew day logs | No |

Exported files are compatible with Brewfather, BeerSmith, BrewUnited, BrewSmith Mobile, and any other BeerXML 1.0-compatible tool.
