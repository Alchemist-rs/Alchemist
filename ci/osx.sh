#!/usr/bin/env bash

brew update
brew outdated sqlite | brew upgrade sqlite
