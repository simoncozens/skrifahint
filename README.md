# tilvisan

`tilvisan` is a tool for autohinting TrueType fonts. It began life as a port of `ttfautohint` to Rust, but now uses the autohint implementation in `skrifa` instead. It is designed to be used both as a library and as a command-line tool.

It supports the `ttfautohint` command-line interface and control file mechanism. The control file mechanism allows users to specify custom hinting instructions for specific glyphs, which can be useful for fine-tuning the hinting of a font.

## Installing and using

You can install Tilvisan in two ways:

* From binary: Download the latest release from https://github.com/simoncozens/tilvisan/releases or use [`cargo binstall`](https://github.com/cargo-bins/cargo-binstall)
* From source: If you have a rust installation already, use `cargo install tilvisan`. If not, go to https://rustup.rs to get one!

Using `tilvisan` should be very familiar to users of `ttfautohint`. The command-line interface is the same, and the control file mechanism is also supported. You can run `tilvisan --help` to see a full list of options. The most basic usage:

```bash
tilvisan font-unhinted.ttf font-hinted.ttf
```

## Name

Following the [fontations](https://github.com/googlefonts/fontations) Old Norse naming scheme, [*til-vísan*](https://cleasby-vigfusson-dictionary.vercel.app/word/til-visan) is the Old Norse word for "guidance", "direction" or "instruction".

## License

`tilvisan` is licensed under the same terms as Rust (MIT + Apache-2.0). See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE) for details.
