ALTER TABLE packages ADD COLUMN void TEXT NOT NULL DEFAULT ''; UPDATE packages SET void = 'sudo' WHERE id = '1' --Keep repeating the UPDATE statment until all packages are added
