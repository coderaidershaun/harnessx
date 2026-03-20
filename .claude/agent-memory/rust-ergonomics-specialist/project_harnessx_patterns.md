---
name: harnessx CLI codebase patterns
description: Rust ergonomic patterns and conventions established in the harnessx CLI crate after refactoring
type: project
---

The harnessx CLI uses a consistent architectural pattern across all subcommands:

- **`exit_with` boundary function** in `output.rs` collapses all `Ok`/`Err` -> JSON response -> process::exit logic into one call. Command handlers delegate to helper functions returning `ParserResult<T>` and call `exit_with(result)`.

- **Helper functions per subcommand**: Complex match arms (Next, Complete, Update) are extracted into named functions (`next_incomplete_section`, `complete_stage`, `update_stage`) that use `?` propagation internally. This replaced 3-level nested match blocks.

- **`Status::FromStr`** impl on the `Status` enum for parsing CLI string arguments, replacing manual match blocks.

- **`Status::is_completed()`** predicate replaces `!= Status::Completed` comparisons.

- **Import convention**: std first, external crates second, `crate::` imports third, separated by blank lines.

- **Error variant reuse**: `ParserError` variants are used with `.ok_or_else()` on `Option` types rather than separate match arms.

**Why:** The original codebase had ~20 instances of `match result { Ok(x) => Response::ok(x).print_and_exit(), Err(e) => ErrorResponse::fail(e.to_string()).print_and_exit() }` which obscured the business logic. The refactoring made the `run()` method of each command a clean dispatch table.

**How to apply:** Any new subcommand should follow the same pattern: enum variant in the `Command`, a `run()` arm calling `exit_with(helper_fn())`, where the helper returns `ParserResult<T: Serialize>`.
