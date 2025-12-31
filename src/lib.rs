use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum SkimpleError {
    #[error("Unable to find needle in haystack")]
    NeedleNotFoundError,

    // This *won't* happen, but I'd rather not panic
    #[error("Needle disappeared from haystack")]
    NeedleDisappearedError,
}

pub struct SkimpleMatcher {
    matcher: SkimMatcherV2,
}

impl SkimpleMatcher {
    pub fn default() -> Self {
        SkimpleMatcher {
            matcher: SkimMatcherV2::default(),
        }
    }

    pub fn new(matcher: SkimMatcherV2) -> Self {
        SkimpleMatcher { matcher }
    }

    pub fn fuzzy<'a, 'b>(
        &self,
        haystack: &'a [&'a str],
        needle: &'b str,
    ) -> Result<&'a str, SkimpleError> {
        let results: Vec<i64> = haystack
            .iter()
            .map(|item| self.matcher.fuzzy_match(item, needle).unwrap_or(0))
            .collect();

        if results.iter().sum::<i64>() == 0 {
            return Err(SkimpleError::NeedleNotFoundError);
        }

        let result: &str = haystack[results
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(index, _)| index)
            .ok_or(SkimpleError::NeedleDisappearedError)?];

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        let matcher = SkimpleMatcher::default();
        let haystack = ["Mort", "Sourcery", "Wyrd Sisters", "Pyramids", "Guards! Guards!"];
        let needle = "gards";

        let result = matcher.fuzzy(&haystack, &needle);
        assert_eq!(result, Ok("Guards! Guards!"));
    }

    #[test]
    fn not_found() {
        let matcher = SkimpleMatcher::default();
        let haystack = ["Mort", "Sourcery", "Wyrd Sisters", "Pyramids", "Guards! Guards!"];
        let needle = "Going Postal";

        let result = matcher.fuzzy(&haystack, &needle);
        assert_eq!(result, Err(SkimpleError::NeedleNotFoundError));
    }
}
