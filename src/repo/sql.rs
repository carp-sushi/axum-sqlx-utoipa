pub(crate) mod story {
    pub const FETCH: &str = r#"SELECT id, name, seqno, created_at, updated_at
        | FROM stories
        | WHERE id = $1"#;

    pub const LIST: &str = r#"SELECT id, name, seqno, created_at, updated_at
        | FROM stories
        | WHERE seqno >= $1
        | ORDER BY seqno, created_at
        | LIMIT $2"#;

    pub const CREATE: &str = r#"INSERT INTO stories (name)
        | VALUES ($1)
        | RETURNING id, name, seqno, created_at, updated_at"#;

    pub const UPDATE: &str = r#"UPDATE stories
        | SET name = $1, updated_at = now()
        | WHERE id = $2
        | RETURNING id, name, seqno, created_at, updated_at"#;

    pub const DELETE: &str = r#"DELETE FROM stories WHERE id = $1"#;
}

pub(crate) mod task {
    pub const FETCH: &str = r#"SELECT id, story_id, name, status, created_at, updated_at
        | FROM tasks
        | WHERE id = $1"#;

    pub const LIST: &str = r#"SELECT id, story_id, name, status, created_at, updated_at
        | FROM tasks
        | WHERE story_id = $1
        | ORDER BY created_at LIMIT $2"#;

    pub const CREATE: &str = r#"INSERT INTO tasks (story_id, name, status)
        | VALUES ($1, $2, $3)
        | RETURNING id, story_id, name, status, created_at, updated_at"#;

    pub const UPDATE: &str = r#"UPDATE tasks
        | SET name = $1, status = $2, updated_at = now()
        | WHERE id = $3
        | RETURNING id, story_id, name, status, created_at, updated_at"#;

    pub const DELETE: &str = r#"DELETE FROM tasks WHERE id = $1"#;

    pub const DELETE_BY_STORY: &str = r#"DELETE FROM tasks WHERE story_id = $1"#;
}
