# Understanding gravity

Gravity is how brewers measure sugar concentration in wort. More dissolved sugar means higher gravity, which means more food for yeast and more potential alcohol in the finished beer. Understanding gravity is the foundation for understanding almost everything else in brewing.

## What gravity actually measures

When you dissolve sugar in water, the liquid becomes denser. Gravity measurements track that density. Before fermentation, your wort is thick with fermentable sugars. After fermentation, yeast has eaten most of those sugars and converted them to alcohol and CO2, so the liquid is less dense. The difference between the two readings is how you calculate ABV.

## OG and FG

**OG (original gravity)** is the gravity of your wort before you pitch yeast. You measure it on brew day, after the boil and chill, before fermentation starts. It tells you how strong your beer has the potential to be.

**FG (final gravity)** is the gravity after fermentation is complete. It tells you how much sugar the yeast left behind. A lower FG means the yeast ate more sugar, so you get drier beer with more alcohol. A higher FG means more residual sweetness and body, and less alcohol.

Water is 1.000 SG. A light session beer might have an OG around 1.040. A standard American pale ale sits around 1.050-1.055. A barleywine can push 1.100 or higher. Normal FG readings are in the 1.008-1.020 range depending on the recipe and yeast.

## Three scales, same measurement

Brewers use three different scales to express the same thing: how much sugar is dissolved.

**SG (specific gravity)** is the ratio of the wort's density to water. 1.050 means the wort is 5% denser than water. This is the most common scale in homebrewing.

**Plato** expresses the sugar content as a percentage by weight. 12°P means roughly 12g of sugar per 100g of liquid. Commercial breweries and most of Europe use Plato. Brewski converts between them using a cubic polynomial: `-616.868 + 1111.14×SG - 630.272×SG² + 135.997×SG³`.

**Brix** is similar to Plato and is measured by refractometers. It's common in winemaking and useful for quick gravity readings during the mash or boil. Brix and Plato are close enough that many brewers treat them as interchangeable at low values, though Brewski converts precisely using `1.0 + Brix / (258.6 - (Brix / 258.2) × 227.1)`. Note that once fermentation starts, alcohol throws off refractometer readings, so Brewski applies a correction formula for FG readings from a refractometer.

## Gravity points

"Gravity points" is a handy shorthand. Just drop the "1.0" and use the decimal as a whole number. 1.050 becomes "50 points." 1.012 becomes "12 points." This makes mental math a lot easier: adding 10 points of gravity to a recipe, or figuring out how much a batch is off target.

## See also

- [Gravity Conversions calculator](/reference/calculators/gravity-conversions): convert between SG, Plato, and Brix instantly
- [ABV & Calories calculator](/reference/calculators/abv-calories): calculate ABV and estimated calories from your OG and FG
