<H1 align="center">Alchemist</H1>
<p align="center">
  <img src=https://avatars3.githubusercontent.com/u/19353789?v=3&s=200>
</p>
<H2 align="center">Unix Platform Agnostic Installation Tool</H2>


**Master:**

[![Build Status](https://travis-ci.org/mgattozzi/alchemist.svg?branch=master)](https://travis-ci.org/mgattozzi/alchemist)

### Project Status
Currently the project is not dead. Being busy with my final year of CS
Classes and with a job as well makes keeping up with projects really hard.
I would love to continue this project and make it grow further when
I have the time. I may be working on it for my engineering class but
that is uncertain. Contributions are of course always welcome and are easy
to review as I have enough time to do that itself.

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

###Compiler Support
Stable 1.15

###Native Library Dependencies

For diesel:

```
Sqlite3
```

###Dev Setup
This will setup all the tools you need to get hacking away. If you
already have cargo and sqlite setup just run:

```
cargo build
```

The build script automatically installs the `diesel_cli` tool if you don't
have it and runs any available migrations for the database for you each build.

####Arch Linux
To get setup for development run:

```
sh scripts/arch_setup.sh
```

####Void Linux
 To get setup for development run:

 ```
 sh scripts/void_setup.sh
 ```

## Get in contact
Join us on [Discord](https://discord.gg/3w9JhWy) if you need help or want to help contribute!

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
