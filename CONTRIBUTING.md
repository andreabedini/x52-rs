# Contributing

## Formatting

CI runs a format check (`cargo fmt --all --check`), so **every commit must be
formatted**. Run formatting before committing:

```bash
cargo fmt --all
```

### Automatic formatting with `jj fix`

If you use [Jujutsu](https://github.com/jj-vcs/jj), you can have formatting
applied automatically with `jj fix`. Add this to your repo config
(`.jj/repo/config.toml`):

```toml
[fix.tools.rustfmt]
command = ["rustfmt", "--emit", "stdout", "--edition", "2024"]
patterns = ["glob:\"**/*.rs\""]
```

`jj fix` runs each file through a command that reads it on stdin and writes the
result to stdout. `cargo fmt` does not work that way, but
`rustfmt --emit stdout` does. `--edition 2024` matches the workspace edition
(rustfmt otherwise defaults to 2015).

Then run `jj fix` (or `jj fix -s <revset>` to format a whole stack) before
pushing.

## Before submitting

```bash
cargo fmt --all --check   # Formatting
cargo clippy --all        # Lints
cargo test --all          # Tests
```
