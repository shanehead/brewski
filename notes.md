# Entities

## Recipes
- Ingredients and amounts w/ metadata (e.g. 10m boil, or whirlpool)
- Versionable (later)
### Ingredients
#### Fermentables
#### Hops
#### Yeast
#### Water

## Profiles
### Equipment

## Batches
Each instance of brewing a Recipe is a Batch with it's own notes
Have a good way to tie a versioned recipe to its batches

# Updates Needed
- [x] Use repository pattern for DB access with SeaORM
- [x] Use better variable names
- [x] Simpler architecture diagram (e.g. UI -> invoke) (is this MVC??)
- [x] OSS License
- [x] Update Git commits to shane.head@gmail.com
- [x] Test coverage
- [ ] Schema first development, split up OpenAPI file
- [ ] Versioned API
- [ ] C4 Diagrams of architecture
- [ ] Microsoft Rust best practices checks
- [ ] Separate release bundles using SemVer for each OS
- [ ] Allow media attachments to recipes and batches
- [ ] Icons next to additions (or any other UI element)
- [ ] Unit testing for Frontend & Tauri IPC layer
- [ ] Recipe versions
- [ ] Hop types (cryo, quantum, etc..)
- [ ] SQLite Database location in release
- [ ] Bundle SQLite for one single deployable bundle
- [ ] Cloud syncing (Google Drive? Dropbox? iCloud? S3?)
- [ ] SQLite FKs and check constraints
- [ ] SeaORM Migrations seem complex (multiple touch points to make a change)
- [ ] Tools tab to expose calculations

# Data
- [ ] Hop database (abstrax, yakima valley, etc..)
- [ ] Fermentables database (import from online db)
- [ ] Yeast database (import from online db)
- [ ] Recipe Database (import from online dbs, then allow people to share eventually)
- [ ] Equipment Database (import from online dbs, major brands and types of equipment, plus common types like BIAB)

# Questions
- [x] Is the rust decimal conversion stuff really needed or just boilerplate/bloat? Lots of `from_dec` stuff hanging around
- [x] Why are hops, yeast, etc. all `Additions` with one repository? Should have a repository for each model. `create_water` and `update_hop` should not be in the same interface [FIXED]
- [x] Why does `recipe.rs` go through SeaORM `Entity`'s directly instead of through their respective `Repository`'s? [FIXED]

# Ideas
- [ ] Markdown support and rendering in notes tab