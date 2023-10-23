# diplopod

A reimplementation of the classic snake clone '[Tausendfuß](https://archive.org/details/Tausendfuss_1983_TCS_Computer_DE)' from the [Colour Genie EG2000](https://en.wikipedia.org/wiki/Colour_Genie)

https://user-images.githubusercontent.com/138790/224545845-c4918f03-1b6a-4e07-8425-594a7f7e32cf.mp4

## Objective

Diplopod is a typical snake game where your goal is to eat and grow.
The playing field is filled with green food and red poison.
Every time food is consumed, the diplopod grows and new food and poison appear. So the amount of food remains constant while the amount of poison increases.
When a certain amount of food has been eaten, a superfood appears in the form of a rotating star.
Every second superfood is accompanied by an antidote that makes the Diplopod immune to the poison for ten seconds. Use that time to clear the playing field.

## Installation

There are no precompiled binaries (yet) so you have to install the Rust toolchain.
Follow the instructions on [Rust's official setup page](https://www.rust-lang.org/learn/get-started).
If you are using Linux, you should also check the [official Bevy Linux dependencies page](https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md).

Once you have installed all dependencies, you can start the game (as a debug build) with

    $ cargo run --features bevy/dynamic_linking

or install a binary with

    $ cargo install --path .

## Acknowledgements

This game owes a lot of ideas to the tutorial [Creating a Snake Clone in Rust, with Bevy](https://mbuffett.com/posts/bevy-snake-tutorial/).

Sound effects were obtained from https://www.zapsplat.com

The font used is '[Allerta Stencil](https://github.com/google/fonts/tree/main/ofl/allertastencil)'. It was published under the terms and conditions of the [OFL](assets/fonts/OFL.txt).

## License

All code in this repository is licensed under the [MIT License](LICENSE-MIT).

The assets included in this repository fall under different open licenses.
