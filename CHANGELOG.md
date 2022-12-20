# Changelog

## [0.3.0] - 2022-12-19

### Added

- `review` subcommand to sum all transactions across each category.

## [0.2.0] - 2022-05-05

### Added

- Adding broader support for `Category` budgets in the `homebank_db` crate
- Display budget progress with the `budget` subcommand
  - Renders progress bars of transactions within a category and compares that against budgets set for those categories
  - Can search for categories by their name or consider specific time intervals

## [0.1.1] - 2022-04-22

### Added

- Ability to query by parent + sub-category
  - Parent categories are separated by their sub-categories with a `:`
  - It was previously impossible to distinguish between sub-categories with the same name, e.g. `Hello:There` and `WhoGoes:There`.
  - Now, you can include the entire parent + sub-category name in the query.

## [0.1.0] - 2022-04-14

- Initial release
