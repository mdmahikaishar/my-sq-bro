use crate::utils::value_mapper;
use builder::DeleteBuilder;

mod builder;

pub struct Delete {
    table_name: String,
    when: Vec<(String, String)>,
}

impl Delete {
    pub fn new() -> DeleteBuilder {
        DeleteBuilder::default()
    }

    pub fn exec(&self) -> String {
        let mut query = String::new();

        // DELETE FROM `{}` WHERE ({});

        query.push_str(&format!("DELETE FROM `{}`", self.table_name));

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete() {
        let delete_one = Delete::new()
            .table_name("users")
            .when("id", "1")
            .when("email", "sql@mail.com")
            .build()
            .exec();

        assert_eq!(
            delete_one,
            [
                "DELETE FROM `users`",
                "WHERE (`users`.`id` = 1 AND `users`.`email` = \"sql@mail.com\")"
            ]
            .join(" ")
        );
    }
}
