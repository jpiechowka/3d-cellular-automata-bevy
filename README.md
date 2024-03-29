# 3D Cellular Automata

3D Cellular Automata in Rust and Bevy 0.12

<!-- TOC -->
* [3D Cellular Automata](#3d-cellular-automata)
  * [Features](#features)
  * [Learning resources](#learning-resources)
  * [Running the debug build](#running-the-debug-build)
  * [Building](#building)
    * [Using `RUSTFLAGS` env variable](#using-rustflags-env-variable)
  * [Gallery](#gallery)
  * [License](#license)
  * [Contributions](#contributions)
<!-- TOC -->

## Features

TODO: Fill this list

* Assets and icons created using `Aseprite` (https://github.com/aseprite/aseprite). To build from source see this
  guide: https://gist.github.com/luciopaiva/6a1f870f932a5f54011cc869c4d558a8
* Additional debug console logging and `bevy-inspector-egui` can be enabled using the `debug` feature
  (see: [Running the debug build section](#running-the-debug-build))
* Played a little bit with [JetBrains AI assistant](https://www.jetbrains.com/ai/) for documentation, code generation
  and commit messages completion

## Learning resources

* https://softologyblog.wordpress.com/2019/12/28/3d-cellular-automata-3/
* https://github.com/ManevilleF/bevy_life
* https://youtu.be/63qlEpO73C4
* https://youtu.be/jkHqrkcEHRc
* https://bevyengine.org/learn/book/getting-started/
* https://bevy-cheatbook.github.io/
* https://nnethercote.github.io/perf-book/introduction.html

## Running the debug build

You can run the debug build using `debug` feature flag with:

```
git clone https://github.com/jpiechowka/3d-cellular-automata-bevy.git
cd 3d-cellular-automata-bevy
cargo run --package cellular-automata --bin 3d-cellular-automata-bevy --features debug
```

Logging can be configured using the `RUST_LOG` environment
variable (https://bevy-cheatbook.github.io/fundamentals/log.html#environment-variable)

## Building

Install Rust (https://www.rust-lang.org/tools/install), then run the commands below:

```
git clone https://github.com/jpiechowka/3d-cellular-automata-bevy.git
cd 3d-cellular-automata-bevy
cargo build --release
```

### Using `RUSTFLAGS` env variable

If you do not care that much about the compatibility of your binary on older (or other types of) processors, you can
tell the compiler to generate the newest (and potentially fastest) instructions specific to a certain CPU architecture
by using `RUSTFLAGS`environment
variable (https://nnethercote.github.io/perf-book/build-configuration.html#cpu-specific-instructions)

```
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

The full command to clone and build will be:

```
git clone https://github.com/jpiechowka/3d-cellular-automata-bevy.git
cd 3d-cellular-automata-bevy
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

On Windows you need to follow this pattern: https://superuser.com/a/1049433

## Gallery

TODO: Provide some pictures or video of the final app

## License

3D Cellular Automata is free, open source and permissively licensed! Except where noted (below and/or in individual
files),
all code in this repository is dual-licensed under either:

* MIT License (`LICENSE-MIT` file or http://opensource.org/licenses/MIT)
* Apache License, Version 2.0 (`LICENSE-APACHE` file or http://www.apache.org/licenses/LICENSE-2.0)

at your option. This means you can select the license you prefer! This dual-licensing approach is the de-facto standard
in the Rust ecosystem.

## Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
