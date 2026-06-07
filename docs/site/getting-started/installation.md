# Installation

Brewski is available for Mac, Windows, Linux, iOS, and Android. Download the latest release from the [GitHub releases page](https://github.com/shanehead/brewski/releases), then follow the instructions for your platform below.

## macOS

Download the `.dmg` file. Open it and drag Brewski into your **Applications** folder.

The first time you launch it, macOS might block it with a Gatekeeper warning. If that happens, right-click the app icon and choose **Open** instead of double-clicking. You'll only need to do this once.

## Windows

Download the `.msi` installer. Run it and follow the prompts. Brewski installs like any other Windows app.

## Linux

Download the `.AppImage` file. Before you can run it, you need to make it executable:

```bash
chmod +x Brewski.AppImage
```

Then double-click it in your file manager, or run it directly from the terminal:

```bash
./Brewski.AppImage
```

## iOS

Brewski is coming to the App Store soon. Until then, download the `.ipa` file from the [GitHub releases page](https://github.com/shanehead/brewski/releases) and install it using AltStore, Sideloadly, or your preferred sideloading tool.

## Android

Brewski is coming to Google Play soon. Until then, download the `.apk` file from the [GitHub releases page](https://github.com/shanehead/brewski/releases). On your device, go to **Settings → Apps → Special app access → Install unknown apps**, allow your file manager or browser, then open the `.apk` to install.

## First launch

When Brewski opens for the first time, it creates a local SQLite database in your system's app data folder. There's nothing to configure. You're ready to go.

If you ever want to know exactly where that database lives, or if you'd like to move it to a shared folder for cloud sync, go to **Settings → Database Location**. See [Cloud Sync](/guides/cloud-sync) for a full walkthrough.

::: tip Next up: your first recipe
Once Brewski is running, head to [Your first recipe](/getting-started/first-recipe) to start building something worth brewing.
:::
