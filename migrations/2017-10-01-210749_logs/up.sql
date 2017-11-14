CREATE TABLE log_infos (
  id BIGSERIAL PRIMARY KEY,
  taskid BIGINT NOT NULL,
  category varchar(50),
  what varchar(50),
  details varchar(2000)
);
CREATE TABLE log_warnings (
  id BIGSERIAL PRIMARY KEY,
  taskid BIGINT NOT NULL,
  category varchar(50),
  what varchar(50),
  details varchar(2000)
);
CREATE TABLE log_errors (
  id BIGSERIAL PRIMARY KEY,
  taskid BIGINT NOT NULL,
  category varchar(50),
  what varchar(50),
  details varchar(2000)
);
CREATE TABLE log_fatals (
  id BIGSERIAL PRIMARY KEY,
  taskid BIGINT NOT NULL,
  category varchar(50),
  what varchar(50),
  details varchar(2000)
);
CREATE TABLE log_invalids (
  id BIGSERIAL PRIMARY KEY,
  taskid BIGINT NOT NULL,
  category varchar(50),
  what varchar(50),
  details varchar(2000)
);

create index log_infos_taskid on log_infos(taskid);
create index log_warnings_taskid on log_warnings(taskid);
create index log_errors_taskid on log_errors(taskid);
create unique index log_fatals_taskid on log_fatals(taskid);
create unique index log_invalids_taskid on log_invalids(taskid);

-- Note: to avoid a sequential scan on log fors all the report pages, the following indexes are crucial:
create index log_infos_index on log_infos(category,what,taskid);
create index log_warnings_index on log_warnings(category,what,taskid);
create index log_errors_index on log_errors(category,what,taskid);
create index log_fatals_index on log_fatals(category,what,taskid);
create index log_invalids_index on log_invalids(category,what,taskid);
