CREATE ROLE root WITH LOGIN;
CREATE TABLE packages (
  id SERIAL PRIMARY KEY,
  arch TEXT NOT NULL,
  aur TEXT NOT NULL,
  ubuntu TEXT NOT NULL,
  ubuntu_dev TEXT NOT NULL
);

GRANT SELECT ON packages TO root;

INSERT INTO packages(arch, aur, ubuntu, ubuntu_dev) VALUES ('sudo','','sudo','');
