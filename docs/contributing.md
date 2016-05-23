##Contributing Guidelines
- Before submitting a PR please rebase off the latest version of the dev
  branch.
- Please be respectful of others. You can disagree but you don't have to
  be a dick about it.
- If you have an issue please file a bug report with your error, what
  you were doing right up to that point, what distribution you are on
  and what you have tried to resolve it. Saying it's broken will have
  the issue closed until a better report is filed. No exceptions. It
  will be reopened upon editing the issue and sufficient information is
  provided.
- If you have any concerns or anything regarding the project you can
  email me at mgattozzi@gmail.com

##Adding a distro:
A few things need to happen in order to add a distro to Alchemist.
Please note that anything with a variable (eg $distroname) is intended
for you to replace.

1. Open up an issue or comment on a request issue that you are adding
   support and include the following as check boxes. Comment on the issue
   as the requirements below are completed so they can be checked off to
   know where the issue stands
2. Add the distro to the Distro enum located in src/alchemy/distro.rs
3. Add the distro to the Package Struct located in src/alchemy/models.rs
4. Create a new migration using the [diesel_cli
   tool](https://github.com/diesel-rs/diesel/blob/master/diesel_cli) by
   running:
   ```
   diesel migration generate add_$distroname_support
   ```
5. Using the up.sql file just created add the following to the file:
   ```
   ALTER TABLE packages ADD COLUMN $distroname TEXT NOT NULL DEFAULT '';
   UPDATE packages SET $distroname = '$packagename' WHERE
   id = '$packageidnumber'
   --Keep repeating the UPDATE statment until all packages are added
   ```
   This will have to also include all of the names of the packages that
   have been added to the database since the beginning of the project.
   You'll have to provide all the package names in order to be
   maintained further in the future. This also makes it easier for
   releases. All we have to say is "Added $DISTRO support" w/ the
   package names being implicit.
6. You'll also need to fill the down.sql with the following in order to
   make sure a migration can be reverted. Also make sure to add the last few migrations to the CREATE TABLE and INSERT INTO.:
   ```sql
   CREATE TABLE new_packages (
     id int PRIMARY KEY NOT NULL,
     arch TEXT NOT NULL,
     aur TEXT NOT NULL,
     ubuntu TEXT NOT NULL,
     ubuntu_dev TEXT NOT NULL
   );

   INSERT INTO new_packages SELECT id, arch, aur, ubuntu, ubuntu_dev FROM packages;

   DROP TABLE IF EXISTS packages;
   ALTER TABLE  new_packages RENAME TO packages;
   ```
7. You're going to also have to provide a setup script under scripts for
   your distribution that sets up the db and gets all the dependencies
   for the user as well as installing rust nightly using rustup.rs.
   Please include on the issue an output of the script running or
   a link to a text file of some sort showing it ran as expected.
8. Add distro to the query in the package query method located in
   src/alchemy/db.rs
9. Add install, upgrade, and refresh commands for your distribution.
   Show that your additions can do each for your distribution using
   either text output or a video of some sort that can be reviewed.

##Adding a package:
This is much easier to do than adding a distro simply run the following:

```bash
#This creates a migration with the timestamp as part of the name
diesel migration generate pkgmappings
```

Open up the up.sql from the new migration and add the following:

```sql
-- After packages include each of the distros located in the
-- Packages struct in src/alchemy/models folder. You can
-- see what distros are in by checking the DB using sqlite
-- You'll also need to provide the correct id number for the db
INSERT INTO packages(id,arch, aur, ubuntu, ubuntu_dev) VALUES
(1,'postgresql',''postgresql','');

-- Add more mappings to that list one for each distro in the order that
-- you put for the first part of the statement. If no mapping exists you
-- have to put ''. You're required to have all the distros in your INSERT
-- statement
```

For the down.sql file:

```sql
DELETE FROM packages WHERE id > $ID_OF_PACKAGE_BEFORE_YOU_ADDED_NEW_ONES
```

You can find out by running:
```
sqlite3 alchemist.db
SELECT * FROM packages;
```

Look at the last id of the package that you didn't add. Use that one.

```
