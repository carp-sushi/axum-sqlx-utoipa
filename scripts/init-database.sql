-- create db
CREATE DATABASE sqlx_todos;
GRANT ALL ON DATABASE sqlx_todos to postgres;

-- create schema ( \c sqlx_todos first )
CREATE SCHEMA IF NOT EXISTS sqlx_todos;
GRANT ALL ON SCHEMA sqlx_todos to postgres;
