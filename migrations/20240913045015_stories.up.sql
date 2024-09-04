create table stories (
    id uuid default gen_random_uuid() primary key,
    name text not null,
    seqno bigint generated always as identity,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create index stories_seqno_index on stories using btree(seqno);

alter table stories
    add constraint check_story_name_length check (char_length(name) <= 100);
