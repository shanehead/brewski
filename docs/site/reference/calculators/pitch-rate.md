# Pitch Rate

The **Pitch Rate** calculator figures out how many yeast cells you need for your batch and whether your pack has enough viable cells, or if you need to build a starter. You'll find it under **Tools** in the left sidebar.

## Inputs

| Field | Description |
|---|---|
| **OG** | Your target original gravity |
| **Batch Size (L)** | Volume of wort going into the fermenter |
| **Pitch Rate** | Target in millions of cells per mL per degree Plato (M cells/mL/°P) |
| **Yeast Pack Cells (billions)** | The cell count listed by the manufacturer for your yeast pack |
| **Viability (%)** | Estimated percentage of cells that are still alive |

## Outputs

| Output | Description |
|---|---|
| **Required Cell Count** | Total cells needed for this batch, in billions |
| **Starter Volume** | How large a starter you need to build, in liters (0 means no starter needed) |

## How it works

### Pitch rate targets

Pitch rate is measured in millions of cells per mL of wort per degree Plato. The standard targets are:

- **0.75 M/mL/°P**: ales, most homebrewing situations
- **1.5 M/mL/°P**: lagers, which ferment at cold temperatures and need more yeast to get going

Higher-gravity worts can also benefit from a slightly higher pitch rate. Some brewers use 1.0 for high-gravity ales.

### Yeast pack cell count

Most liquid yeast packs from Wyeast and White Labs contain approximately 100 billion cells when produced. Check the manufacturer's spec sheet for the exact number; some newer format packs ship with more.

### Viability

Yeast cells die off over time, even refrigerated. A pack produced last week might be at 95% viability. A pack that's two months old might be down to 65-70%. Brewski uses your stated viability to estimate how many living cells are actually available in the pack.

### Starter volume

If the viable cell count from your pack falls short of the required cell count, Brewski calculates the starter volume needed to grow the deficit. The calculation assumes approximately 100 billion new cells per liter of starter wort on a stir plate.

**If starter volume shows 0:** your pack already has enough viable cells and no starter is needed.

## Tips

**Dry yeast usually doesn't need a starter.** Most dry yeast packets contain 200 billion cells or more, and dry yeast stays viable for much longer than liquid. For standard homebrew batch sizes, a single dry yeast packet is usually sufficient without a starter.

**Build your starter 24-48 hours before brew day.** Give it time to ferment out and cold crash before pitching. You can decant most of the spent starter wort and just pitch the concentrated slurry.

**Re-pitching from a previous batch.** This calculator assumes a fresh commercial pack. If you're re-pitching from a slurry, you'll need to estimate your cell count separately based on yeast cake volume and density.
