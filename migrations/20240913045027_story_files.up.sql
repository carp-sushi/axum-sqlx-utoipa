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

alter table story_files
    add constraint story_files_name_length_check check (char_length(name) <= 100);

alter table story_files
    add constraint story_files_size_check check (size > 0);
