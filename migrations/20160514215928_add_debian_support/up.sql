ALTER TABLE packages ADD COLUMN debian TEXT NOT NULL DEFAULT '';
UPDATE packages SET debian = 'sudo' WHERE id = '1' --Keep repeating the UPDATE statment until all packages are added