# lcs-image-diff
Image diff library and tool with LCS algorithm. rust port of [murooka/go-diff-image](https://github.com/murooka/go-diff-image)

[![](http://meritbadge.herokuapp.com/lcs-image-diff)](https://crates.io/crates/lcs-image-diff)
[![CircleCI](https://circleci.com/gh/bokuweb/lcs-image-diff-rs/tree/master.svg?style=svg)](https://circleci.com/gh/bokuweb/lcs-image-diff-rs/tree/master)

## Requirements
- latest Rust (recommend [rustup](https://www.rustup.rs/))

## Library

### Usage

```toml
# Cargo.toml
[dependencies]
image = "0.20"
lcs-image-diff = { version = "0.1", default-features = false }
```

```rust
use lcs_image_diff::compare;

let mut before = image::open("before.png")?;
let mut after = image::open("after.png")?;

let diff = compare(&mut before, &mut after, 100.0 / 256.0)?;

before.save("marked_before.png")?;
after.save("marked_after.png")?;
diff.save("diff.png")?;
```

## Binary

### Installation

``` bash
cargo install lcs-image-diff
```

### Usage

```
lcs-image-diff path/to/before.png path/to/after.png path/to/diff.png
```

## Example

| before.png        | after.png          | diff.png                 |
| --------------- |---------------| -------------------- |
| ![](https://github.com/bokuweb/lcs-image-diff-rs/blob/master/test/images/before.png?raw=true) | ![](https://github.com/bokuweb/lcs-image-diff-rs/blob/master/test/images/after.png?raw=true) |![](https://github.com/bokuweb/lcs-image-diff-rs/blob/master/test/images/diff.png?raw=true)|

`lcs-image-diff` outputs marked before and after images too.

| marked_before.png        | marked_after.png          |
| --------------- |---------------|
| ![](https://github.com/bokuweb/lcs-image-diff-rs/blob/master/test/images/marked_before.png?raw=true) | ![](https://github.com/bokuweb/lcs-image-diff-rs/blob/master/test/images/marked_after.png?raw=true) |

## LICENSE

The MIT License (MIT)

Copyright (c) 2018 @bokuweb

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
