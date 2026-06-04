# Hydrometer Correction

The **Hydrometer Correction** calculator adjusts a gravity reading taken at a temperature other than your hydrometer's calibration temperature. You'll find it under **Tools** in the left sidebar.

## Inputs

| Field | Description |
|---|---|
| **Measured SG** | The gravity reading directly from your hydrometer |
| **Sample Temperature** | The temperature of your sample when you took the reading, in °C |
| **Calibration Temperature** | The temperature your hydrometer is calibrated for, in °C |

## Output

| Output | Description |
|---|---|
| **Corrected SG** | The actual gravity of your sample, adjusted for temperature |

## How it works

Hydrometers are calibrated to read accurately at one specific temperature, most commonly 20°C (68°F). When your sample is warmer than that, the liquid is less dense and your hydrometer sinks a bit too far, giving you a reading that's lower than the true gravity. When your sample is cooler, the opposite happens and you read higher than actual.

The calculator applies a polynomial correction formula based on the viscosity of water at different temperatures. You put in what your hydrometer said and what temperature the sample was, and you get back what the gravity actually is.

## Where to find your calibration temperature

Check the label or packaging that came with your hydrometer. Most hydrometers sold for brewing are calibrated to 20°C (68°F). Some older or inexpensive instruments use 15°C (59°F) or 60°F. If you're unsure, 20°C is a safe assumption for most modern hydrometers.

## Tips

**Take readings as close to room temperature as you can.** The correction is reliable near typical calibration temperatures, but the further your sample strays from the calibration point, the less precise the correction becomes. For wort right off the boil, cool a small sample first.

**Don't skip the correction.** A wort sample at 60°C can read 10+ gravity points lower than it actually is. Skipping the correction will give you a false OG, which will throw off your ABV and attenuation calculations.
