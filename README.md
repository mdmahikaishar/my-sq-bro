# My Sq Bro

![Rust](https://img.shields.io/badge/Rust-DD3515?style=for-the-badge&logo=rust&logoColor=white)

Rust SQL query builder.


## Examples

### Insert

```rs
let insert = Insert::new()
  .table_name("users")
  .data("name", "SQL Bro")
  .data("email", "sql@mail.com")
  .build()
  .exec();

assert_eq!(
    insert,
    "INSERT INTO `users` (`name`, `email`) VALUES (\"SQL Bro\", \"sql@mail.com\")"
);
```

### Select

#### Simple Select

```rs
let select = Select::new()
  .table_name("users")
  .attr("name")
  .attr("email")
  .when("id", "1")
  .when("email", "sql@mail.com")
  .build()
  .exec();

assert_eq!(
  select,
  "SELECT `users`.`name`, `users`.`email` FROM `users` WHERE (`users`.`id` = 1 AND `users`.`email` = \"sql@mail.com\")"
)
```

#### Join Select

```rs
let select = Select::new()
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
  select,
  "SELECT `students`.`name`, `schools_students`.`school_id` FROM `students` LEFT JOIN `schools_students` ON (`schools_students`.`student_id` = `students`.`id`) WHERE (`students`.`id` = 1)"
);
```



### Update 

```rs
let update = Update::new()
  .table_name("users")
  .data("name", "Guest User")
  .data("email", "guest@mail.com")
  .when("id", "1")
  .build()
  .exec();

assert_eq!(
  update,
  "UPDATE `users` SET (`name` = \"Guest User\", `email` = \"guest@mail.com\") WHERE (`users`.`id` = 1)"
);
```

### Delete

```rs
let delete = Delete::new()
  .table_name("users")
  .when("id", "1")
  .when("email", "sql@mail.com")
  .build()
  .exec();

assert_eq!(
  delete,
  "DELETE FROM `users` WHERE (`users`.`id` = 1 AND `users`.`email` = \"sql@mail.com\")"
);
```

## Contributing

Contributions are welcome! I would like you to contribute in this project.

## Roadmap

This project is in its early stages, and there are many missing features that need implementation. Check the [Issues](https://github.com/mdmahikaishar/my_sq_bro/issues) section for a list of features, enhancements, and bug fixes that are planned.

## License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/mdmahikaishar/my_sq_bro/LICENSE) file for details.