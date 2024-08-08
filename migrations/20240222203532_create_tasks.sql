create table tasks (
    id int generated always as identity primary key,
    story_id int references stories(id) not null,
    name text not null,
    status text not null default 'incomplete'
);

create index tasks_story_id_index ON tasks USING btree(story_id);

alter table tasks
  add constraint check_task_name_length check (char_length(name) <= 100);

alter table tasks
  add constraint check_task_status
  check (status = 'incomplete' or status = 'complete')
;
