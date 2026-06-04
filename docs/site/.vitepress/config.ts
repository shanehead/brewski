import { defineConfig } from "vitepress";

export default defineConfig({
  title: "Brewski",
  description: "Documentation for Brewski — the free, open-source homebrewing app",
  base: "/brewski/",  // update if deploying to a custom domain (remove base entirely for apex domain)

  themeConfig: {
    nav: [
      { text: "Getting Started", link: "/getting-started/what-is-brewski" },
      { text: "Guides", link: "/guides/building-a-recipe" },
      { text: "Reference", link: "/reference/equipment-profiles" },
      { text: "Concepts", link: "/concepts/gravity" },
      { text: "FAQ", link: "/faq" },
    ],

    sidebar: [
      {
        text: "Getting Started",
        items: [
          { text: "What is Brewski?", link: "/getting-started/what-is-brewski" },
          { text: "Installation", link: "/getting-started/installation" },
          { text: "Your first recipe", link: "/getting-started/first-recipe" },
          { text: "Your first batch", link: "/getting-started/first-batch" },
          { text: "Importing a recipe (BeerXML)", link: "/getting-started/importing" },
        ],
      },
      {
        text: "Guides",
        items: [
          { text: "Building a recipe from scratch", link: "/guides/building-a-recipe" },
          { text: "Working with fermentables", link: "/guides/fermentables" },
          { text: "Working with hops", link: "/guides/hops" },
          { text: "Adding yeast", link: "/guides/yeast" },
          { text: "Dialing in your mash", link: "/guides/mash" },
          { text: "Water chemistry", link: "/guides/water-chemistry" },
          { text: "Scaling a recipe", link: "/guides/scaling" },
          { text: "Recipe versions", link: "/guides/recipe-versions" },
          { text: "Logging a brew day", link: "/guides/brew-day" },
          { text: "Tracking gravity & fermentation", link: "/guides/gravity-tracking" },
          { text: "Carbonation & packaging", link: "/guides/carbonation" },
          { text: "Using the ingredient library", link: "/guides/ingredient-library" },
          { text: "Cloud sync", link: "/guides/cloud-sync" },
        ],
      },
      {
        text: "Reference",
        items: [
          { text: "Equipment profiles", link: "/reference/equipment-profiles" },
          { text: "Styles", link: "/reference/styles" },
          { text: "Ingredient library", link: "/reference/ingredients" },
          {
            text: "Calculators",
            items: [
              { text: "ABV & calories", link: "/reference/calculators/abv-calories" },
              { text: "Hydrometer correction", link: "/reference/calculators/hydrometer-correction" },
              { text: "Refractometer", link: "/reference/calculators/refractometer" },
              { text: "Gravity conversions", link: "/reference/calculators/gravity-conversions" },
              { text: "Color conversions", link: "/reference/calculators/color-conversions" },
              { text: "Pitch rate", link: "/reference/calculators/pitch-rate" },
              { text: "Carbonation", link: "/reference/calculators/carbonation" },
              { text: "Unit conversions", link: "/reference/calculators/unit-conversions" },
            ],
          },
          { text: "Settings", link: "/reference/settings" },
          { text: "BeerXML import & export", link: "/reference/beerxml" },
        ],
      },
      {
        text: "Concepts & Glossary",
        items: [
          { text: "Understanding gravity", link: "/concepts/gravity" },
          { text: "IBU — bitterness explained", link: "/concepts/ibu" },
          { text: "SRM & EBC — color explained", link: "/concepts/color" },
          { text: "ABV & attenuation", link: "/concepts/abv-attenuation" },
          { text: "Mash chemistry basics", link: "/concepts/mash-chemistry" },
          { text: "Water ions & their effects", link: "/concepts/water-ions" },
          { text: "Hop forms & usage types", link: "/concepts/hop-forms" },
          { text: "Yeast pitch rate & starters", link: "/concepts/pitch-rate-starters" },
          { text: "Glossary A–Z", link: "/concepts/glossary" },
        ],
      },
      { text: "FAQ", link: "/faq" },
    ],

    search: {
      provider: "local",
    },

    socialLinks: [
      { icon: "github", link: "https://github.com/shanehead/brewski" },
    ],

    footer: {
      message: "Released under the MIT License.",
      copyright: "Copyright © 2026 Shane Head",
    },
  },
});
