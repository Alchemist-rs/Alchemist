CREATE TABLE packages (
  id int PRIMARY KEY NOT NULL,
  arch TEXT NOT NULL,
  aur TEXT NOT NULL,
  ubuntu TEXT NOT NULL,
  ubuntu_dev TEXT NOT NULL
);

INSERT INTO packages(id,arch, aur, ubuntu, ubuntu_dev) VALUES (1,'sudo','','sudo','');
