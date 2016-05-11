#!/bin/sh

#Install db depdendency
sudo pacman -S sqlite3

#Install multirust
curl -sf https://raw.githubusercontent.com/brson/multirust/master/blastoff.sh | sh
#Install nightly
multirust update nightly

#Set Directory as nightly
multirust override nightly

#Install diesel_cli and db
cargo install diesel_cli
~/.cargo/bin/diesel setup
echo "Add .cargo/bin to your PATH if you want to use diesel_cli everywhere"

echo "The project is all setup!"
