pub fn attr_to_string(table_name: &str, attrs: &[String]) -> Option<String> {
    let value = attrs
        .iter()
        .map(|attr| format!("`{table_name}`.`{attr}`"))
        .collect::<Vec<String>>()
        .join(", ");

    if value.is_empty() {
        return None;
    }

    Some(value)
}

pub fn value_mapper(value: &str) -> String {
    let tild_count = value
        .chars()
        .filter(|i| i.eq(&'`'))
        .collect::<Vec<_>>()
        .len();

    if value == "$" || value.parse::<isize>().is_ok() || (tild_count >= 2 && tild_count % 2 == 0) {
        return value.to_string();
    }

    format!("\"{}\"", value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attr_to_string() {
        assert_eq!(
            attr_to_string("users", &["name".to_string(), "email".to_string()]),
            Some("`users`.`name`, `users`.`email`".to_string())
        );
    }
}
