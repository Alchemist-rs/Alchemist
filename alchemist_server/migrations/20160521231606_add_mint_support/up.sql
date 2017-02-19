ALTER TABLE packages ADD COLUMN mint TEXT NOT NULL DEFAULT '';
UPDATE packages SET mint = 'sudo' WHERE id = '1' --Keep repeating the UPDATE statment until all packages are added
