use crate::utils::{self, value_mapper};
use builder::SelectBuilder;
use join::Join;

mod builder;
pub mod join;

pub struct Select {
    table_name: String,
    attrs: Vec<String>,
    when: Vec<(String, String)>,
    joins: Vec<Join>,
}

impl Select {
    pub fn new() -> SelectBuilder {
        SelectBuilder::default()
    }

    pub fn exec(&self) -> String {
        let mut query = String::new();

        // "SELECT {attrs} FROM `{table}` {joins} WHERE ({when})"

        query.push_str(&format!("SELECT {}", self.get_merged_attr_string()));

        query.push(' ');
        query.push_str(&format!("FROM `{}`", self.table_name));

        if let Some(joins) = self.get_join_string() {
            query.push(' ');
            query.push_str(&joins);
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

    fn get_attr_string(&self) -> Option<String> {
        utils::attr_to_string(&self.table_name, &self.attrs)
    }

    fn get_merged_attr_string(&self) -> String {
        let mut values = Vec::new();

        if let Some(attr_str) = self.get_attr_string() {
            values.push(attr_str);
        }

        self.joins.iter().for_each(|join| {
            if let Some(attr_str) = join.get_attr_string() {
                values.push(attr_str);
            }
        });

        let value = values.join(", ");

        if value.is_empty() {
            panic!("ERROR: get_merged_attr_string is empty.");
        }

        value
    }

    fn get_join_string(&self) -> Option<String> {
        let value = self
            .joins
            .iter()
            .map(|join| join.exec())
            .collect::<Vec<String>>()
            .join(" ");

        if value.is_empty() {
            return None;
        }

        Some(value)
    }
}

#[cfg(test)]
mod tests {
    use super::join::JoinType;
    use super::*;

    #[test]
    fn test_select_simple() {
        let select_one = Select::new()
            .table_name("users")
            .attr("name")
            .attr("email")
            .build()
            .exec();

        assert_eq!(
            select_one,
            ["SELECT `users`.`name`, `users`.`email`", "FROM `users`"].join(" ")
        );

        let select_two = Select::new()
            .table_name("users")
            .attr("name")
            .attr("email")
            .when("id", "1")
            .when("email", "sql@mail.com")
            .build()
            .exec();

        assert_eq!(
            select_two,
            [
                "SELECT `users`.`name`, `users`.`email`",
                "FROM `users`",
                "WHERE (`users`.`id` = 1 AND `users`.`email` = \"sql@mail.com\")"
            ]
            .join(" ")
        );
    }

    #[test]
    fn test_select_simple_with_when_and_raw_when() {
        let select_one = Select::new()
            .table_name("users")
            .attr("name")
            .attr("email")
            .when("id", "1")
            .raw_when("`users`.`email`", "sql@mail.com")
            .build()
            .exec();

        assert_eq!(
            select_one,
            [
                "SELECT `users`.`name`, `users`.`email`",
                "FROM `users`",
                "WHERE (`users`.`id` = 1 AND `users`.`email` = \"sql@mail.com\")"
            ]
            .join(" ")
        );
    }

    #[test]
    fn test_select_join() {
        let select_one = Select::new()
            .table_name("students")
            .attr("name")
            .join(
                Join::new()
                    .join_type(JoinType::Left)
                    .table_name("schools_students")
                    .attr("school_id")
                    .on("student_id", "`students`.`id`"),
            )
            .when("id", "1")
            .build()
            .exec();

        assert_eq!(
            select_one,
            [
                "SELECT `students`.`name`, `schools_students`.`school_id`",
                "FROM `students`",
                "LEFT JOIN `schools_students`",
                "ON (`schools_students`.`student_id` = `students`.`id`)",
                "WHERE (`students`.`id` = 1)"
            ]
            .join(" ")
        );
    }
}
