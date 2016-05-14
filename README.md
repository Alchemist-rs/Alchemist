<H1 align="center">Alchemist</H1>
<p align="center">
  <img src=https://avatars3.githubusercontent.com/u/19353789?v=3&s=200>
</p>
<H2 align="center">Unix Platform Agnostic Installation Tool</H2>


**Master:**

[![Build Status](https://travis-ci.org/Alchemist-rs/Alchemist.svg?branch=master)](https://travis-ci.org/Alchemist-rs/Alchemist)

**Dev:**

[![Build Status](https://travis-ci.org/Alchemist-rs/Alchemist.svg?branch=dev)](https://travis-ci.org/Alchemist-rs/Alchemist)

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
Due to certain needed features we've pegged a specific version of the rust nightly compiler
Currently using:

```
nightly-2016-05-08
```

###Native Library Dependencies

For diesel:

```
Sqlite3
```

###Dev Setup
This will setup all the tools you need to get hacking away.

####Arch Linux
To get setup for development run:

```
sh scripts/arch_setup.sh
```
