# tilvisan

`tilvisan` is a tool for autohinting TrueType fonts. It began life as a port of `ttfautohint` to Rust, but now uses the autohint implementation in `skrifa` instead. It is designed to be used both as a library and as a command-line tool.

It supports the `ttfautohint` command-line interface and control file mechanism. The control file mechanism allows users to specify custom hinting instructions for specific glyphs, which can be useful for fine-tuning the hinting of a font.

## Installing and using

Tilvisan is unpublished as of now. If you have `rustup` or a Rust toolchain installed, you can build it from source using Cargo:

```bash
cargo install --git https://github.com/simoncozens/tilvisan
```

(If you need to install rustup, you can find instructions at https://rustup.rs/.)

Alternatively, you can grab the latest build by clicking on the `Actions` tab and downloading the latest artifact from the most recent green workflow run. At the bottom of the page linked from that workflow run, you should see a section called "Artifacts" with a link to download the latest build for Apple Silicon, Apple x86, Linux or Windows.

Using `tilvisan` should be very familiar to users of `ttfautohint`. The command-line interface is the same, and the control file mechanism is also supported. You can run `tilvisan --help` to see a full list of options. The most basic usage:

```bash
tilvisan font-unhinted.ttf font-hinted.ttf
```

## Name

Following the [fontations](https://github.com/googlefonts/fontations) Old Norse naming scheme, [*til-vísan*](https://cleasby-vigfusson-dictionary.vercel.app/word/til-visan) is the Old Norse word for "guidance", "direction" or "instruction".

## License

`tilvisan` is licensed under the same terms as Rust (MIT + Apache-2.0). See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE) for details.