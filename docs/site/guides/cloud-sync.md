# Cloud sync

By default, Brewski stores your database in your app data folder on your local machine. Cloud sync is optional. It works by moving that database file into a folder your cloud provider already monitors, so your recipes, batches, and settings stay backed up and available on other devices.

## Setting it up

Go to **Settings** and find the **Database Location** section. If Brewski detects Google Drive, iCloud, or Dropbox on your system, it shows each one with a **Move here** button. Click it and Brewski moves your database to that folder. That's all there is to it.

If you'd rather use a different location, enter a custom path in the text field and move it manually.

## Using Brewski on a second device

Install Brewski on the second device. Open **Settings**, go to **Database Location**, and point it at the same file in your cloud-synced folder. Brewski will use that database going forward, with all your recipes and batches intact.

## One device at a time

Here's the one rule you need to follow: don't have Brewski open on two devices at the same time. Brewski uses a SQLite database, and SQLite isn't built for simultaneous writes from multiple sources. If both devices are writing at the same time, you can end up with a corrupted or out-of-sync database.

The safe workflow is simple: finish using Brewski on one device, close it, let your cloud provider sync the file, then open it on the other device. The sync usually takes just a few seconds.

## Backing up without a second device

Even if you're only using one device, moving the database to a cloud folder is a good idea. It gives you an automatic offsite backup. If your machine dies, your recipes are safe.
