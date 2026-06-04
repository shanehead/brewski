# Carbonation

The **Carbonation** calculator has two sections: **Priming Sugar** for bottle-conditioning, and **CO2 Pressure** for force carbonation in a keg. You'll find it under **Tools** in the left sidebar.

This calculator uses the same math as the carbonation section in your batch **Overview** tab. Entering the same inputs here and there will give you identical results.

## Priming Sugar

Use this section when you're bottling and need to add sugar to carbonate.

### Inputs

| Field | Description |
|---|---|
| **Target CO2 Volumes** | How carbonated you want the finished beer |
| **Batch Size (L)** | Volume of beer being primed |
| **Fermentation Temperature (°C)** | The temperature at the end of fermentation, before cold crashing |
| **Sugar Type** | Corn Sugar (dextrose), Table Sugar (sucrose), or Dry Malt Extract (DME) |

### Output

| Output | Description |
|---|---|
| **Priming Sugar** | Grams of the selected sugar to add at packaging |

### How it works

Beer retains a small amount of dissolved CO2 from fermentation, depending on how warm it was. The calculator uses your fermentation temperature to estimate this residual CO2, then figures out how much additional CO2 the priming sugar needs to contribute to hit your target.

**Use the temperature at the end of fermentation, not after cold crashing.** The residual CO2 estimate is based on the temperature at which fermentation finished, not your packaging temperature.

**DME requires more grams than pure sugars.** DME is not 100% fermentable, so you need more of it by weight to produce the same amount of CO2. The calculator accounts for this automatically based on the sugar type you select.

### CO2 volumes by style

| Style | Typical CO2 volumes |
|---|---|
| British ales | 1.8-2.2 |
| American ales | 2.3-2.6 |
| German lagers | 2.4-2.7 |
| Hefeweizens | 3.0-3.6 |
| Belgian styles | 3.0-3.8 |

## CO2 Pressure

Use this section when you're force carbonating a keg.

### Inputs

| Field | Description |
|---|---|
| **Target CO2 Volumes** | How carbonated you want the finished beer |
| **Serving Temperature (°C)** | The temperature at which the keg will be stored and served |

### Output

| Output | Description |
|---|---|
| **Serving Pressure** | Set your regulator to this pressure, in kPa |

### How it works

CO2 dissolves into beer more readily at lower temperatures. The calculator uses Henry's Law to determine the equilibrium pressure you need to maintain at your serving temperature to achieve your target carbonation level. Set your regulator to the output pressure and the beer will absorb CO2 until it reaches the target volume.

## Tips

**For priming sugar, weigh your sugar.** Volume measurements for priming sugar are imprecise. Use a kitchen scale for consistent results batch to batch.

**For force carb, give it time.** At serving pressure, carbonation typically takes 1-2 weeks at refrigerator temperatures. You can speed this up with higher pressure and agitation, but the equilibrium method is more reliable and won't over-carbonate.
