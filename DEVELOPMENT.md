# Developing ts_deplint

```sh
# Run on a TypeScript repository.
cargo run lint ~/loop/backend/src

# Run unit tests.
cargo test

# Bump version.
fastmod 0\.0\.7 0.0.8 

# Release.
git tag -d v0.0.8
git push origin :refs/tags/v0.0.8

git tag v0.0.8
git push origin --tags
```
