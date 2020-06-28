# exemd

> exemd is a markdown executor, it will make markdown executable. 

[![Build Status](https://travis-ci.org/phodal/exemd.svg?branch=master)](https://travis-ci.org/phodal/exemd)

![build](https://github.com/phodal/exemd/workflows/build/badge.svg)

## Development

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

## LICENSE

origin code inspired by [mask](https://github.com/jakedeichert/mask) with MIT LICENSE;

[![Phodal's Idea](http://brand.phodal.com/shields/idea-small.svg)](http://ideas.phodal.com/)

@ 2020 A [Phodal Huang](https://www.phodal.com)'s [Idea](http://github.com/phodal/ideas).  This code is distributed under the MPL license. See `LICENSE` in this directory.
