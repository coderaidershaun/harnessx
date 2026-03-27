# Autorun Command

Launches a fully autonomous Claude operator session in the current workspace. Requires a `harnessx/` directory to exist (i.e. `harnessx init` must have been run first).

Runs:

```
claude --dangerously-skip-permissions -p "/hx:operator" --output-format json
```

## `autorun`

```bash
harnessx autorun
```

Spawns the Claude operator with full permissions, inheriting stdin, stdout, and stderr. The process exit code is forwarded from the `claude` process.

### Error: no harnessx directory

If `harnessx/` does not exist in the current working directory:

```json
{
  "success": false,
  "error": "no harnessx/ directory found — run `harnessx init` first"
}
```

Exit code: `1`

## `autorun -- [extra args...]`

Any trailing arguments are forwarded directly to the `claude` command.

```bash
harnessx autorun -- --verbose
```
