#[rustfmt::skip]
pub mod story {
    pub const FETCH: &str = r#"
        | SELECT id, name
        | FROM stories
        | WHERE id = $1"#;

    pub const LIST: &str = r#"
        | SELECT id, name
        | FROM stories
        | WHERE id <= $1
        | ORDER BY id desc
        | LIMIT $2"#;

    pub const CREATE: &str = r#"
        | INSERT INTO stories (name)
        | VALUES ($1)
        | RETURNING id, name"#;

    pub const UPDATE: &str = r#"
        | UPDATE stories
        | SET name = $1
        | WHERE id = $2
        | RETURNING id, name"#;

    pub const DELETE: &str = r#"
        | DELETE FROM stories WHERE id = $1"#;

    pub const EXISTS: &str = r#"
        | SELECT EXISTS(
        |   SELECT 1 FROM stories WHERE id = $1
        | )"#;
}

#[rustfmt::skip]
pub mod task {
    pub const FETCH: &str = r#"
        | SELECT id, story_id, name, status
        | FROM tasks
        | WHERE id = $1"#;

    pub const LIST: &str = r#"
        | SELECT id, story_id, name, status
        | FROM tasks
        | WHERE story_id = $1
        | ORDER BY id LIMIT $2"#;

    pub const CREATE: &str = r#"
        | INSERT INTO tasks (story_id, name)
        | VALUES ($1, $2)
        | RETURNING id, story_id, name, status"#;

    pub const UPDATE: &str = r#"
        | UPDATE tasks
        | SET name = $1, status = $2
        | WHERE id = $3
        | RETURNING id, story_id, name, status"#;

    pub const DELETE: &str = r#"
        | DELETE FROM tasks WHERE id = $1"#;

    pub const DELETE_BY_STORY: &str = r#"
        | DELETE FROM tasks WHERE story_id = $1"#;

    pub const EXISTS: &str = r#"
        | SELECT EXISTS(
        |   SELECT 1 FROM tasks WHERE id = $1
        | )"#;
}
