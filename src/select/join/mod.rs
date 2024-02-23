use crate::utils::{self, value_mapper};
pub use builder::{JoinBuilder, JoinType};

mod builder;

pub struct Join {
    join_type: JoinType,
    table_name: String,
    attrs: Vec<String>,
    on: Vec<(String, String)>,
}

impl Join {
    pub fn new() -> JoinBuilder {
        JoinBuilder::default()
    }

    /// Exec
    ///
    /// "{join} `{table}` ON ({on})",
    pub fn exec(&self) -> String {
        let mut query = String::new();

        query.push_str(&self.join_type.to_string());

        query.push(' ');
        query.push_str(&format!("`{}`", self.table_name));

        if let Some(on) = self.get_on_string() {
            query.push(' ');
            query.push_str(&format!("ON ({})", on));
        }

        query
    }

    pub fn get_attr_string(&self) -> Option<String> {
        utils::attr_to_string(&self.table_name, &self.attrs)
    }

    fn get_on_string(&self) -> Option<String> {
        if self.on.is_empty() {
            // panic!("ERROR: `JoinTableBuilder` needs `on` statement.");
            return None;
        }

        let value = self
            .on
            .iter()
            .map(|(key, value)| format!("{} = {}", key, value_mapper(value)))
            .collect::<Vec<String>>()
            .join(" AND ");

        Some(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_join() {
        let join_one = Join::new()
            .join_type(JoinType::Left)
            .table_name("schools_students")
            .on("student_id", "1")
            .build()
            .exec();

        assert_eq!(
            join_one,
            [
                "LEFT JOIN `schools_students`",
                "ON (`schools_students`.`student_id` = 1)",
            ]
            .join(" ")
        );
    }

    #[test]
    fn test_join_with_on_and_raw_on() {
        let join_one = Join::new()
            .join_type(JoinType::Left)
            .table_name("schools_students")
            .on("student_id", "1")
            .raw_on("`schools_students`.`student_id`", "2")
            .build()
            .exec();

        assert_eq!(
            join_one,
            [
                "LEFT JOIN `schools_students`",
                "ON (`schools_students`.`student_id` = 1 AND `schools_students`.`student_id` = 2)",
            ]
            .join(" ")
        );
    }
}
