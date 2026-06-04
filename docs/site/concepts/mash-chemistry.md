# Mash chemistry basics

The mash is where raw grain becomes fermentable wort. You're soaking crushed grain in hot water and letting naturally occurring enzymes break down starches into sugars. Getting the temperature, pH, and duration right determines how fermentable your wort is and how much sugar you extract.

## The two key enzymes

Two enzymes do most of the work in a typical single-infusion mash:

**Beta-amylase** works best between 63-66°C (145-151°F). It clips sugars off the end of starch chains one at a time, producing mostly maltose, which yeast ferments readily. A mash at the lower end of the range gives you highly fermentable wort: the beer will finish drier and have more alcohol.

**Alpha-amylase** works best between 68-72°C (154-162°F). It chops up starch chains in the middle, producing a mix of fermentable sugars and longer-chain dextrins that yeast can't ferment. A mash at the higher end leaves more body and residual sweetness in the finished beer.

Most single-infusion mashes target 65-68°C (149-154°F) to get useful activity from both enzymes. Lean lower for a drier, crisper beer. Lean higher for a fuller-bodied result.

Both enzymes denature (stop working) at higher temperatures. Above about 75°C (167°F), the enzymatic conversion stops, which is exactly why mashout heats the wort before sparging.

## Mash pH

**Mash pH** is the acidity of your mash liquid. The ideal range is 5.2-5.4. Within that range, both enzymes work efficiently and you extract clean, fermentable sugars. Outside of it, enzyme activity drops off and you start extracting harsh tannins from the grain husks, especially at high pH.

Dark malts naturally lower pH. Pale malts are close to neutral. Most tap water is pH 7 or higher, which would push your mash pH too high without adjustment. Mineral additions like calcium and acids like lactic or phosphoric acid bring it down. Brewski's water chemistry tools help you calculate these adjustments.

## Mash efficiency

**Mash efficiency** is how much of the available sugar in your grain you actually extract into the wort. If your grain bill has 100 gravity points available and you extract 75 of them, your efficiency is 75%.

Typical homebrew efficiency ranges from 65-80%. Several things improve it:

- **Crush quality.** Finer crush exposes more starch to the water. A good mill makes a real difference. Aim for cracked husks with the inner starch exposed, not flour.
- **Mash time.** Conversion is usually complete in 45-60 minutes, but a longer mash gives slower-working enzymes more time.
- **Water-to-grain ratio.** Thinner mashes tend to be slightly more efficient and easier to work with.
- **Mash temperature consistency.** If your mash cools significantly during the rest, enzyme activity slows.

## Strike temperature

**Strike temperature** is the water temperature you need to hit your target mash temperature. Cold grain absorbs heat from the water, so you need to start hotter than your target. Brewski calculates this using:

```
strike_temp = (0.41 / ratio) × (target_temp - grain_temp) + target_temp
```

where `ratio` is water volume in quarts per pound of grain, `target_temp` is your mash temperature target, and `grain_temp` is the current temperature of your grain. Room-temperature grain at 20°C typically requires strike water about 4-7°C above your mash target.
