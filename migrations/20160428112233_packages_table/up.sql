CREATE ROLE root WITH LOGIN;
CREATE TABLE packages (
  id SERIAL PRIMARY KEY,
  arch TEXT NOT NULL,
  aur TEXT NOT NULL,
  ubuntu TEXT NOT NULL
);

GRANT SELECT ON packages TO root;

INSERT INTO packages(arch, aur, ubuntu) VALUES ('sudo','','sudo');
INSERT INTO packages(arch, aur, ubuntu) VALUES ('sudo','','hello');
