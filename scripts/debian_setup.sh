#!/bin/bash

#Install db depdendency
sudo aptitude update
sudo aptitude install sqlite3 libsqlite3-dev curl gcc

#Install multirust
curl https://sh.rustup.rs -sSf | sh

#Ensure rust commands run
source $HOME/.cargo/env

#Install nightly
rustup install nightly

#Set Directory as nightly
rustup override add nightly-2016-05-08

#Install diesel_cli and db
cargo install diesel_cli
~/.cargo/bin/diesel setup
echo "Add .cargo/bin to your PATH if you want to use diesel_cli everywhere"

echo "The project is all setup!"
