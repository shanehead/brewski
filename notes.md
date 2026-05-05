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
- [ ] Schema first development, split up OpenAPI file
- [ ] Versioned API
- [ ] C4 Diagrams of architecture
- [ ] Simpler architecture diagram (e.g. UI -> invoke) (is this MVC??)
- [ ] OSS License
- [ ] Test coverage
- [ ] Microsoft Rust best practices checks
- [ ] Bundle SQLite for one single deployable bundle
- [ ] Separate release bundles using SemVer for each OS
- [x] Update Git commits to shane.head@gmail.com
- [ ] Allow media attachments to recipes and batches
- [ ] Images next to additions (or any other UI element)
- [ ] Cloud syncing (Google Drive? Dropbox? iCloud?)

# Questions
- [ ] Is the rust decimal conversion stuff really needed or just boilerplate/bloat? Lots of `from_dec` stuff hanging around
- [x] Why are hops, yeast, etc. all `Additions` with one repository? Should have a repository for each model. `create_water` and `update_hop` should not be in the same interface [FIXED]
- [x] Why does `recipe.rs` go through SeaORM `Entity`'s directly instead of through their respective `Repository`'s? [FIXED]

# Ideas
- Markdown support and rendering in notes tab