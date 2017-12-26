extern crate regex;

use std::collections::HashMap;

lazy_static! {
    static ref PATTERN_REGEX : regex::Regex = regex::Regex::new(r"\{([^\}]+)\}").unwrap();
}

pub struct Template {
    raw: String,
    pub placeholders: Vec<String>,
}

impl Template {
    pub fn new<T: AsRef<str>>(name: T) -> Template {
        let raw = name.as_ref().to_string();
        let matches = PATTERN_REGEX.find_iter(name.as_ref());
        let placeholders: Vec<String> = matches.map(|m| {
            name.as_ref()[(m.start() + 1)..(m.end() - 1)].to_string()
        }).collect();
        Template {
            raw,
            placeholders,
        }
    }

    pub fn evaluate(&self, replacements: &HashMap<String, String>) -> Result<String, String> {
        for needed_placeholder_key in &self.placeholders {
            if !replacements.contains_key(needed_placeholder_key) {
                return Err(format!("Missing placeholder key: {}", needed_placeholder_key));
            }
        }
        let evaluated = PATTERN_REGEX.replace_all(&self.raw, |caps: &regex::Captures| {
            caps
                .get(1)
                .map(|m| {
                    replacements.get(&m.as_str().to_string()).unwrap()
                })
                .unwrap()
                .clone()
        });
        Ok(evaluated.to_string())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! str_hash {
    () => {
        HashMap::new()
    };

    ( $( ($key:expr, $value:expr) ),* ) =>
        {
            {
                let mut temp_map = HashMap::new();
                $(
                    temp_map.insert($key.to_string(), $value.to_string());
                )*
                temp_map
            }
        };
    }

    #[test]
    fn template_can_be_instantiated() {
        Template::new("");
    }

    #[test]
    fn template_without_placeholders_gives_empty_vec() {
        let template = Template::new("some.thing");
        assert_eq!(template.placeholders.len(), 0);
    }

    #[test]
    fn template_gives_correct_placeholders() {
        let template = Template::new("some.thing");
        assert_eq!(template.placeholders.len(), 0);
    }

    #[test]
    fn template_without_placeholders_evaluates_to_initial_template_string() {
        let template = Template::new("some.thing");
        let evaluation_result = template.evaluate(&str_hash!());
        assert_eq!(evaluation_result, Ok("some.thing".to_string()));
    }

    #[test]
    fn template_fails_to_evaluate_if_a_placeholder_key_is_missing() {
        let template = Template::new("part.{tok1}.part_{tok2}");
        let evaluation_result = template.evaluate(&str_hash!(("tok1", "val1")));
        assert_eq!(evaluation_result, Err("Missing placeholder key: tok2".to_string()));
    }

    #[test]
    fn template_nominal_usage_works() {
        let template = Template::new("part.{tok1}.part_{tok2}");
        let evaluation_result = template.evaluate(&str_hash!(("tok1", "val1"), ("tok2", "val2")));
        assert_eq!(evaluation_result, Ok("part.val1.part_val2".to_string()));
    }
}
