create table stories (
    id int generated always as identity primary key,
    name text not null
);

alter table stories
  add constraint check_story_name_length check (char_length(name) <= 100);
