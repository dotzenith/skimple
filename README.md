<h1 align="center"> ━━━━  ❖  ━━━━ </h1>

## ❖ skimple

`skimple` is a *simple* interface for [skim fuzzy-matcher](https://crates.io/crates/fuzzy-matcher)


## ❖ Examples

```rust
use skimple::SkimpleMatcher;

let matcher = SkimpleMatcher::default();
let haystack = ["Mort", "Sourcery", "Wyrd Sisters", "Pyramids", "Guards! Guards!"];
let needle = "gards";

let result = matcher.fuzzy(&haystack, &needle);
assert_eq!(result, Ok("Guards! Guards!"));
```

## ❖ What's New?

1.0.0 - Initial Release

---

<div align="center">

   <img src="https://img.shields.io/static/v1.svg?label=License&message=MIT&color=F5E0DC&labelColor=302D41&style=for-the-badge">

</div>
