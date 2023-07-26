-- Your SQL goes here
CREATE TABLE if not exists chat (
  id uuid not null,
  sender varchar(200) not null,
  receiver varchar(200) not null,
  created_date timestamp not null,
  content varchar(1000) not null,
  PRIMARY KEY (id)
)
