<h1 align="center"> ━━━━  ❖  ━━━━ </h1>

## ❖ skimple

`skimple` is a *simple* interface for [skim fuzzy-matcher](https://crates.io/crates/fuzzy-matcher)


## ❖ Examples

### Best Match
```rust
use skimple::SkimpleMatcher;

let matcher = SkimpleMatcher::default();
let haystack = ["Mort", "Sourcery", "Wyrd Sisters", "Pyramids", "Guards! Guards!"];
let needle = "gards";

let result = matcher.fuzzy_best(&haystack, &needle);
assert_eq!(result, Ok("Guards! Guards!"));
```

### All Matches
```rust
use skimple::SkimpleMatcher;

let matcher = SkimpleMatcher::default();
let haystack = ["Mort", "Sourcery", "Wyrd Sisters", "Pyramids", "Guards! Guards!"];
let needle = "yr";

let result = matcher.fuzzy_all(&haystack, &needle);
assert_eq!(result, Ok(vec!["Wyrd Sisters", "Pyramids"]));
```

## ❖ What's New?

2.0.0 - Add feature to return all matches, and not just the best

---

<div align="center">

   <img src="https://img.shields.io/static/v1.svg?label=License&message=MIT&color=F5E0DC&labelColor=302D41&style=for-the-badge">

</div>
