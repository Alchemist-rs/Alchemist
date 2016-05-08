#!/bin/sh

#Install depdendencies and set them up
#sqlite is only included to get diesel_cli working
sudo pacman -S postgresql postgresql-libs
echo "Enabling postgresql"
sudo systemctl enable postgresql
echo "Starting postgresql"
sudo systemctl start postgresql

#Install multirust
curl -sf https://raw.githubusercontent.com/brson/multirust/master/blastoff.sh | sh
#Install nightly
multirust update nightly

#Set Directory as nightly
multirust override nightly

#Install diesel_cli and db
createdb alchemist
cargo install diesel_cli
~/.cargo/bin/diesel migration run
echo "Add .cargo/bin to your PATH if you want to use diesel_cli everywhere"

echo "The project is all setup!"
