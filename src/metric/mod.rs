extern crate time;
extern crate regex;

use std::collections::HashMap;
use super::database::Matrix;

mod template;

#[derive(Debug, PartialEq)]
pub struct Metric {
    pub name: String,
    pub value: i64,
    pub timestamp: u32,
}

impl Metric {
    pub fn new<T: AsRef<str>>(name: T, value: i64, time: &time::Timespec) -> Metric {
        Metric {
            name: name.as_ref().to_string(),
            value,
            timestamp: time.to_millis(),
        }
    }
}

pub trait ToMetrics {
    fn to_metrics<T: AsRef<str>>(&self, pattern: T, time: &time::Timespec) -> Vec<Metric>;
}

impl ToMetrics for Matrix {
    fn to_metrics<T: AsRef<str>>(&self, pattern: T, time: &time::Timespec) -> Vec<Metric> {
        fn build_value_columns(column_names: &Vec<String>, key_columns: &Vec<String>) -> Vec<String> {
            column_names.iter()
                .filter(|&col| !key_columns.contains(col))
                .map(|col| col.clone())
                .collect()
        }
        let pattern_template = template::Template::new(&pattern);

        let key_columns = &pattern_template.placeholders;

        if let Some(missings) = key_columns.missing_from(&self.column_names) {
            panic!(format!("Metric template use missing columns {:?}", missings))
        }

        let value_columns = build_value_columns(&self.column_names, key_columns);

        if value_columns.is_empty() { panic!("No value column to produce metric") };

        let mut metrics = Vec::new();
        for row in &self.rows {
            let key_prefix = pattern_template.evaluate(&row).unwrap();
            for value_column in &value_columns {
                let name = format!("{}.{}", &key_prefix, &value_column);
                let str_val = row.get(value_column).unwrap();
                let value = str_val.parse::<i64>().expect(format!("Value column {} contains a non-numeric value {}", value_column, str_val).as_ref());
                metrics.push(Metric::new(name, value, time));
            }
        }

        metrics
    }
}

pub trait ToMillis {
    fn to_millis(&self) -> u32;
}

impl ToMillis for time::Timespec {
    fn to_millis(&self) -> u32 {
        (self.sec + self.nsec as i64 / 1000 / 1000) as u32
    }
}

trait MissingFrom<T> {
    fn missing_from(&self, sub: &Vec<T>) -> Option<Vec<&T>>;
}

impl<T: PartialEq> MissingFrom<T> for Vec<T> {
    fn missing_from(&self, sub: &Vec<T>) -> Option<Vec<&T>> {
        let mut missings = Vec::new();
        for key in self {
            if !sub.contains(key) {
                missings.push(key);
            }
        }
        if missings.is_empty() {
            None
        } else {
            Some(missings)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_matrix_is_converted_to_empty_set_of_metrics() {
        let matrix = Matrix {
            column_names: vec![String::from("SOME"), String::from("THING"), String::from("WEIRD")],
            rows: Vec::new(),
        };

        let metrics = matrix.to_metrics("ANY.{SOME}.OTHER.{THING}", &time::get_time());
        assert_eq!(metrics, vec![]);
    }

    #[test]
    fn matrix_with_one_row_and_no_fragment_column_is_converted_to_a_metric_singleton() {
        let mut row_hash_map = HashMap::new();
        row_hash_map.insert(String::from("SOME"), String::from("42"));
        let matrix = Matrix {
            column_names: vec![String::from("SOME")],
            rows: vec![row_hash_map],
        };

        let time = time::get_time();


        let metrics = matrix.to_metrics("ANY", &time);
        assert_eq!(metrics, vec![Metric::new("ANY.SOME", 42, &time)]);
    }

    #[test]
    fn placeholder_in_key_template_are_resolved() {
        let mut row_hash_map = HashMap::new();
        row_hash_map.insert(String::from("SOME"), String::from("SOME_1"));
        row_hash_map.insert(String::from("THING"), String::from("THING_2"));
        row_hash_map.insert(String::from("WEIRD"), String::from("43"));
        let matrix = Matrix {
            column_names: vec![String::from("SOME"), String::from("THING"), String::from("WEIRD")],
            rows: vec![row_hash_map],
        };

        let time = time::get_time();


        let metrics = matrix.to_metrics("ANY.{SOME}.OTHER.{THING}", &time);
        assert_eq!(metrics, vec![Metric::new("ANY.SOME_1.OTHER.THING_2.WEIRD", 43, &time)]);
    }
}
