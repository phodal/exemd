# exemd

> exemd is  a markdown executor, make markdown executable. support for Kotlin, Java, Golang, Rust, Ruby, Python, JavaScript, TypeScript, PHP, Bash....  

[![Build Status](https://travis-ci.org/phodal/exemd.svg?branch=master)](https://travis-ci.org/phodal/exemd)
[![build](https://github.com/phodal/exemd/workflows/build/badge.svg)](https://github.com/phodal/exemd/actions)
[![crates.io badge](https://img.shields.io/crates/v/exemd.svg)](https://crates.io/crates/exemd)<br/>
[![codecov](https://codecov.io/gh/phodal/exemd/branch/master/graph/badge.svg)](https://codecov.io/gh/phodal/exemd)

## Todo

 - [ ] shell
   - [x] bash
 - [ ] script language
   - [x] Ruby
   - [x] Javascript
   - [x] Python
     - [x] single file
     - [x] dependency
   - [x] PHP
   - [x] TypeScript by [deno](https://deno.land/)
 - [ ] compiled language
   - [x] Rust
     - [x] single file
     - [x] dependency
   - [x] Java
     - [x] single file
     - [x] dependency
       - [x] Gradle
   - [x] Go
     - [x] single file
   - [X] Kotlin
     - [x] single file
 
## Examples

see in [fixtures](_fixtures)

config list:

 - exemd-deps
 - exemd-name
 - exemd-filename

deps format: `[name];version=[version]`

```
colored;version=1.8.0
```

## Usage

````
```rust
// exemd-deps: colored;version=1.8.0 
extern crate colored;

use colored::*;

fn main() {
    println!("{} {} !", "it".green(), "works".blue().bold());
}
```
````

```bash
exemd run _fixtures/sample.md
```

## Development

Install Rust and Cargo
 
```bash
curl https://sh.rustup.rs -sSf | sh
``` 

1. clone

```
git clone https://github.com/phodal/exemd
```

2. run

## LICENSE

origin code inspired by [mask](https://github.com/jakedeichert/mask) with MIT LICENSE;

[![Phodal's Idea](http://brand.phodal.com/shields/idea-small.svg)](http://ideas.phodal.com/)

@ 2020 A [Phodal Huang](https://www.phodal.com)'s [Idea](http://github.com/phodal/ideas).  This code is distributed under the MPL license. See `LICENSE` in this directory.
