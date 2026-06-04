# FAQ

## Is Brewski free?

Yes. Brewski is free and MIT-licensed. There's no subscription, no account, no credit card, and no features locked behind a paywall. If it's in the app, it's yours.

## Does Brewski send my data anywhere?

No. Your data lives in a local SQLite database on your machine and goes nowhere by default. Cloud sync is optional, and if you use it, you're syncing to your own Google Drive, iCloud, or Dropbox account using your own credentials.

## Can I import recipes from Brewfather or BeerSmith?

Yes. Both tools can export BeerXML, and Brewski imports BeerXML 1.0. In the recipe list sidebar, hit **Import BeerXML**, pick your `.xml` file, and you're done. In Brewfather, export via **Recipe → Export to BeerXML** to get the file.

## Why doesn't my imported recipe look right after import?

A few things don't translate perfectly from every tool. The most common one is the equipment profile: BeerXML includes equipment data, but Brewski uses your own saved profiles for volume and efficiency calculations. After importing, open the recipe and pick the right equipment profile for your setup. Water chemistry and custom ingredients are worth a quick review too. If an ingredient doesn't match anything in Brewski's library, it comes in as a custom entry with the original numbers intact.

## What platforms does Brewski run on?

Brewski runs on Mac, Windows, and Linux. It's a desktop app.

## Can I use Brewski on my phone?

Not yet. Brewski is currently desktop-only (Mac, Windows, Linux). A mobile app is planned for the future.

## Where is my database stored?

Open **Settings** and look for the **Database Location** section. The current path is shown there. You can move it from the same section.

## How do I back up my recipes?

The easiest way is to move your database to a cloud-synced folder like Google Drive, iCloud, or Dropbox. Go to **Settings → Database Location** and click **Move here** next to your provider. If you'd rather have portable files, you can also export individual recipes as BeerXML using the **Export** button in the recipe header.

## How do I switch between metric and imperial?

Go to **Settings → Units** and change the units there.

## What gravity units does Brewski support? How do I switch?

Brewski supports SG (specific gravity) and Plato. To switch, go to **Settings → Units → Gravity Unit** and pick the one you want.

## My refractometer FG reading seems off. What do I do?

Refractometers need a correction after fermentation starts because alcohol affects the reading. Use **Tools → Refractometer** in Brewski to enter your original gravity and your refractometer reading, and Brewski will calculate the corrected FG for you.

## How are IBUs calculated?

Brewski uses the Tinseth formula for IBU calculations.

## The tooltips are in the way. Can I turn them off?

Yes. Go to **Settings → Help** and toggle off **Show tooltips**.

## Can I share recipes with other Brewski users?

Yes. Use the **Export** button in the recipe header to export a recipe as BeerXML, then send the file to whoever you want. They can import it into Brewski using **Import BeerXML** in the recipe list sidebar. Any homebrewing tool that supports BeerXML 1.0 can read Brewski exports too.

## Do I need an internet connection to use Brewski?

No. Brewski is fully offline. Everything runs locally on your machine. You only need internet if you're syncing your database to a cloud storage provider.

## I found a bug or have a feature request. Where do I report it?

Open an issue on [GitHub](https://github.com/shanehead/brewski). That's the best place to report bugs and suggest features.
