ALTER TABLE packages ADD COLUMN freebsd TEXT NOT NULL DEFAULT '';
UPDATE packages SET freebsd = 'sudo' WHERE id = '1' --Keep repeating the UPDATE statment until all packages are added
