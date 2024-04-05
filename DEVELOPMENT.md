# Developing ts_deplint

```sh
# Run on a TypeScript repository.
cargo run lint ~/loop/backend/src

# Run unit tests.
cargo test

# Release.
git tag -d v0.0.6
git push origin :refs/tags/v0.0.6

git tag v0.0.6
git push origin --tags
```
