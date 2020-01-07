# rusty_screeps

A very hacky and not-at-all good set of fooling around with [Screeps][screeps], a Javascript 
MMO Game but in Rust. This project is built using the [`screeps-game-api`] bindings from the 
[rustyscreeps] organization.

It's also recommended to use [`cargo-screeps`] for uploading the code, but the code should still
compile if using [`cargo-web`] directly instead.

Quickstart:

```bash
# clone:

git clone https://github.com/toddaaro/rusty_screeps
cd rusty_screeps

# cli dependencies:

cargo install cargo-screeps

# configure for uploading:

cp screeps-no-token.toml screeps.toml
nano screeps.toml

# release new version
./release 1.0.0 "we made it to 1.0.0!"

# build tool:

cargo screeps --help
```

[screeps]: https://screeps.com/
[`stdweb`]: https://github.com/koute/stdweb
[`cargo-web`]: https://github.com/koute/cargo-web
[`cargo-screeps`]: https://github.com/rustyscreeps/cargo-screeps/
[`screeps-game-api`]: https://github.com/rustyscreeps/screeps-game-api/
[rustyscreeps]: https://github.com/rustyscreeps/
