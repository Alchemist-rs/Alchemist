#Shaper
##Unix Platform Agnostic Installation Tool

Master: [![Build Status](https://travis-ci.org/mgattozzi/shaper.svg?branch=master)](https://travis-ci.org/mgattozzi/shaper)

Dev: [![Build Status](https://travis-ci.org/mgattozzi/shaper.svg?branch=dev)](https://travis-ci.org/mgattozzi/shaper)

###Inspiration
I was tired of having to search for what packages I have to use for what
distribution. Some I had to download two separate ones others I just had
to download one of them. The naming between distributions was different.
My setup on one computer was hard to replicate on another. What if I had
just one tool that would be able to read a configuration file, figure
out what distribution it was on and then be able to download the proper
tools it needed? What about instructions that only list Ubuntu packages
in the README? What if you could pass those package names to Shaper and
have it install the proper packages for you without you having to figure
out the proper mapping for your distro due to undocumented instructions
or unsupported distributions. This is what Shaper hopes to accomplish
and make it easy to install things regardless of your package manager.

###What Shaper Will Do
- Install Packages for you based off whatever name is passed
- Work on any Unix Based System (Sorry Windows)

###Contributing
- Please see contributing.md under docs. A basic summary is below:
  - Simply testing Shaper on various distributions will help!
    I only have so many distributions I can test, and I don't plan
    on buying a Mac anytime soon.
  - Documentation! Throw in doc based comments that rustdoc can pick up
    or providing man pages. Also commenting internally on non public
    facing data structures or functions for new maintainer or
    contributors
  - Contribute code! Provide mappings for packages, make something more
    idiomatic, fixing basic things that might be caught with Clippy, or
    design a whole new feature we didn't even know we wanted. The choice
    is yours and anything helps!
  - Write tests! Unit tests and integration tests that can be run on a
    CI instance is a big plus and adds better code coverage!

###Roadmap
- v0.10
  - [ ] Create a mapping structure
  - [ ] Create a configuration file
    - [ ] Parse Configuratin
    - [ ] Configuration file structure
  - [ ] Basic Arch Linux Support (No AUR)
