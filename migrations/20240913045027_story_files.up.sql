create table story_files (
    id uuid default gen_random_uuid() primary key,
    story_id uuid references stories(id) not null,
    storage_id uuid not null,
    name text not null,
    size bigint not null default 0,
    content_type text not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create index story_files_story_id_index ON story_files USING btree(story_id);
