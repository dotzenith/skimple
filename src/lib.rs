//! # skimple
//!
//! `skimple` is a *simple* interface for [skim fuzzy-matcher](https://crates.io/crates/fuzzy-matcher)
//!
//!
//! ### Best Match
//! ```rust
//! use skimple::SkimpleMatcher;
//!
//! let matcher = SkimpleMatcher::default();
//! let haystack = ["Mort", "Sourcery", "Wyrd Sisters", "Pyramids", "Guards! Guards!"];
//! let needle = "gards";
//!
//! let result = matcher.fuzzy_best(&haystack, &needle);
//! assert_eq!(result, Ok("Guards! Guards!"));
//! ```
//!
//! ### All Matches
//! ```rust
//! use skimple::SkimpleMatcher;
//!
//! let matcher = SkimpleMatcher::default();
//! let haystack = ["Mort", "Sourcery", "Wyrd Sisters", "Pyramids", "Guards! Guards!"];
//! let needle = "yr";
//!
//! let result = matcher.fuzzy_all(&haystack, &needle);
//! assert_eq!(result, Ok(vec!["Wyrd Sisters", "Pyramids"]));
//! ```

use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use thiserror::Error;

/// Possible errors that can occur during a fuzzy search
#[derive(Error, Debug, PartialEq)]
pub enum SkimpleError {
    /// There were no matches for the search string
    #[error("Unable to find needle in haystack")]
    NeedleNotFoundError,

    /// This is not a case that will realistically occur in normal usage
    ///
    /// It's defensive variant to represent an error while trying
    /// to get the element with the highest score in the fuzzy matching step
    #[error("Needle disappeared from haystack")]
    NeedleDisappearedError,
}

/// A simple (skimple?) struct to wrap around `SkimMatcherV2`
pub struct SkimpleMatcher {
    matcher: SkimMatcherV2,
}

impl SkimpleMatcher {
    /// The default constructor for `SkimpleMatcher`
    ///
    /// Embeds `SkimMatcherV2::default()`
    pub fn default() -> Self {
        SkimpleMatcher {
            matcher: SkimMatcherV2::default(),
        }
    }

    /// Creates a new `SkimpleMatcher` with a user supplied `SkimMatcherV2`
    pub fn new(matcher: SkimMatcherV2) -> Self {
        SkimpleMatcher { matcher }
    }

    /// Fuzzily search through a list of strings, returning the best match
    ///
    /// ## Examples
    /// ```rust
    /// use skimple::SkimpleMatcher;
    ///
    /// let matcher = SkimpleMatcher::default();
    /// let haystack = ["Mort", "Sourcery", "Wyrd Sisters", "Pyramids", "Guards! Guards!"];
    /// let needle = "gards";
    ///
    /// let result = matcher.fuzzy_best(&haystack, &needle);
    /// assert_eq!(result, Ok("Guards! Guards!"));
    /// ```
    pub fn fuzzy_best<'a, T>(&self, haystack: &'a [T], needle: &str) -> Result<&'a str, SkimpleError>
    where
        T: AsRef<str>,
    {
        let results: Vec<i64> = haystack
            .iter()
            .map(|item| self.matcher.fuzzy_match(item.as_ref(), needle).unwrap_or(0))
            .collect();

        if results.iter().sum::<i64>() == 0 {
            return Err(SkimpleError::NeedleNotFoundError);
        }

        let result: &str = haystack[results
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(index, _)| index)
            .ok_or(SkimpleError::NeedleDisappearedError)?]
        .as_ref();

        Ok(result)
    }

    /// Fuzzily search through a list of strings, returning all matches sorted by match score
    ///
    /// ## Examples
    /// ```rust
    /// use skimple::SkimpleMatcher;
    ///
    /// let matcher = SkimpleMatcher::default();
    /// let haystack = ["Mort", "Sourcery", "Wyrd Sisters", "Pyramids", "Guards! Guards!"];
    /// let needle = "yr";
    ///
    /// let result = matcher.fuzzy_all(&haystack, &needle);
    /// assert_eq!(result, Ok(vec!["Wyrd Sisters", "Pyramids"]));
    /// ```
    pub fn fuzzy_all<'a, T>(&self, haystack: &'a [T], needle: &str) -> Result<Vec<&'a str>, SkimpleError>
    where
        T: AsRef<str>,
    {
        let results: Vec<i64> = haystack
            .iter()
            .map(|item| self.matcher.fuzzy_match(item.as_ref(), needle).unwrap_or(0))
            .collect();

        if results.iter().sum::<i64>() == 0 {
            return Err(SkimpleError::NeedleNotFoundError);
        }

        let mut matches: Vec<(&str, i64)> = haystack
            .iter()
            .map(|item| item.as_ref())
            .zip(results.into_iter())
            .filter(|(_, score)| *score != 0)
            .collect();

        matches.sort_by(|(_, a), (_, b)| a.cmp(b));

        Ok(matches.into_iter().map(|(item, _)| item).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuzzy_search_best_slice_of_str() {
        let matcher = SkimpleMatcher::default();
        let haystack = ["Mort", "Sourcery", "Wyrd Sisters", "Pyramids", "Guards! Guards!"];
        let needle = "gards";

        let result = matcher.fuzzy_best(&haystack, &needle);
        assert_eq!(result, Ok("Guards! Guards!"));
    }

    #[test]
    fn test_fuzzy_search_best_vec_of_string() {
        let matcher = SkimpleMatcher::default();
        let haystack = vec![
            "Mort".to_string(),
            "Sourcery".to_string(),
            "Wyrd Sisters".to_string(),
            "Pyramids".to_string(),
            "Guards! Guards!".to_string(),
        ];
        let needle = "gards";

        let result = matcher.fuzzy_best(&haystack, &needle);
        assert_eq!(result, Ok("Guards! Guards!"));
    }

    #[test]
    fn test_fuzzy_search_search_best_string_not_found() {
        let matcher = SkimpleMatcher::default();
        let haystack = ["Mort", "Sourcery", "Wyrd Sisters", "Pyramids", "Guards! Guards!"];
        let needle = "Going Postal";

        let result = matcher.fuzzy_best(&haystack, &needle);
        assert_eq!(result, Err(SkimpleError::NeedleNotFoundError));
    }

    #[test]
    fn test_fuzzy_search_search_all() {
        let matcher = SkimpleMatcher::default();
        let haystack = ["Mort", "Sourcery", "Wyrd Sisters", "Pyramids", "Guards! Guards!"];
        let needle = "yr";

        let result = matcher.fuzzy_all(&haystack, &needle);
        assert_eq!(result, Ok(vec!["Wyrd Sisters", "Pyramids"]));
    }
}
