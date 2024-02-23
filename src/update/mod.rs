use crate::utils::value_mapper;
use builder::UpdateBuilder;

mod builder;

pub struct Update {
    table_name: String,
    data: Vec<(String, String)>,
    when: Vec<(String, String)>,
}

impl Update {
    pub fn new() -> UpdateBuilder {
        UpdateBuilder::default()
    }

    ///
    ///
    /// ```sql
    /// UPDATE {table} SET ({set}) WHERE ({when});
    /// ````
    pub fn exec(&self) -> String {
        let mut query = String::new();

        query.push_str(&format!("UPDATE `{}`", self.table_name));

        if let Some(set) = self.get_set_string() {
            query.push(' ');
            query.push_str(&format!("SET ({})", set));
        }

        if let Some(when) = self.get_where_string() {
            query.push(' ');
            query.push_str(&format!("WHERE ({when})"));
        }

        query
    }

    fn get_where_string(&self) -> Option<String> {
        let value = self
            .when
            .iter()
            .map(|(key, value)| format!("{} = {}", key, value_mapper(value)))
            .collect::<Vec<String>>()
            .join(" AND ");

        if value.is_empty() {
            return None;
        }

        Some(value)
    }

    fn get_set_string(&self) -> Option<String> {
        let value = self
            .data
            .iter()
            .map(|(key, value)| format!("`{}` = {}", key, value_mapper(value)))
            .collect::<Vec<String>>()
            .join(", ");

        if value.is_empty() {
            return None;
        }

        Some(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_simple() {
        let update_one = Update::new()
            .table_name("users")
            .data("name", "Guest User")
            .data("email", "guest@mail.com")
            .when("id", "1")
            .build()
            .exec();

        assert_eq!(
            update_one,
            [
                "UPDATE `users`",
                "SET (`name` = \"Guest User\", `email` = \"guest@mail.com\")",
                "WHERE (`users`.`id` = 1)"
            ]
            .join(" ")
        );
    }
}
