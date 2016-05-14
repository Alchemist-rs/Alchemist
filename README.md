#Alchemist
##Unix Platform Agnostic Installation Tool


Master:

[![Build Status](https://travis-ci.org/mgattozzi/Alchemist.svg?branch=master)](https://travis-ci.org/mgattozzi/Alchemist)

Dev:

[![Build Status](https://travis-ci.org/mgattozzi/Alchemist.svg?branch=dev)](https://travis-ci.org/mgattozzi/Alchemist)

###Inspiration
I was tired of having to search for what packages I have to use for what
distribution. Some I had to download two separate ones others I just had
to download one of them. The naming between distributions was different.
My setup on one computer was hard to replicate on another. What if I had
just one tool that would be able to read a configuration file, figure
out what distribution it was on and then be able to download the proper
tools it needed? What about instructions that only list Ubuntu packages
in the README? What if you could pass those package names to Alchemist and
have it install the proper packages for you without you having to figure
out the proper mapping for your distro due to undocumented instructions
or unsupported distributions. This is what Alchemist hopes to accomplish
and make it easy to install things regardless of your package manager.

###What Alchemist Will Do
- Install Packages for you based off whatever name is passed
- Work on any Unix Based System (Sorry Windows)

###Contributing
- Please see contributing.md under docs. A basic summary is below:
  - Simply testing Alchemist on various distributions will help!
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

###Slack
We communicate using Slack using it as a central way to track issues
etc. If you'd like to join us send an email to mgattozzi@gmail.com
with the Subject Line [Slack Alchemist] and you'll get an invite.

###Compiler Version
Due to the nature of the Diesel library needing nightly
that's the versionthat will be needed. At some point in
the future I'll work on creating an installer to get it
working on stable using Syntex. As of now though since
this is far from production ready nightly is fine.

###Dependencies

For diesel:

```
Sqlite3
```

###Setup
There is more to do here but this should just be as simple as run and
done for the end user to get all setup. For now there are distribution
specific setup scripts.

####Arch Linux
To get setup for development run:

```
sh scripts/arch_setup.sh
```

###Roadmap
- v0.1.0
  - [x] Create a mapping structure
  - [x] Create Ubuntu to Arch Mappings
  - [x] Create a db configuration file
    - [x] Parse Configuratin
    - [x] Configuration file structure
  - [x] Arch Linux Support (including AUR)
