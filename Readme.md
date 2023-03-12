<div align="left">
  <img title="title" src="./assets/icon.svg" alt="icon" align="left" width="75" style="padding-right: 1ch">
  <h1>pt</h1>
  <p><strong>Utility library with tools for point arithmetic</strong></p>
</div>

[![CI Jobs](https://github.com/KGuz/pt/actions/workflows/ci.yml/badge.svg)](https://github.com/KGuz/pt/actions/workflows/ci.yml)
[![Project Status](https://www.repostatus.org/badges/latest/wip.svg)](https://www.repostatus.org/#wip)
[![License MIT](https://img.shields.io/badge/license-MIT-blue)](#license)

## About

This Rust library was created to provide easy-to-use utility structures for 2D and 3D points, along with basic arithmetic operations for these structures. The library was designed to simplify geometric computations in Rust programs, by providing a set of simple and robust data structures.

The library is ideal for use in games, simulations, and other applications that require efficient and accurate geometric computations. By providing a set of generic point structures that can be used with both integer and floating-point coordinates, the library offers a flexible and powerful toolset for developers.

Whether you're working on a complex simulation or a simple game, this library can help simplify your code and improve performance, by providing a set of reliable and optimized point structures that are easy to use and integrate into your programs.

## Getting Started

To get started using this library, simply add the following line to your **Cargo.toml** file

```toml
pt = { git = "https://github.com/KGuz/pt.git" }
```

## Examples

Here are some examples of basic arithmetic operations using the pt library

```rust
use pt::prelude::*;

let a = pt!(1., 2.);
let b = pt!(3., 4.);

// Addition
let sum = a + b;        // Pt2 { x: 4.0, y: 6.0 }

// Subtraction
let difference = a - b; // Pt2 { x: -2.0, y: -2.0 }

// Multiplication
let product = a * 2.;   // Pt2 { x: 2.0, y: 4.0 }

// Division
let quotient = b / 2.;  // Pt2 { x: 1.5, y: 2.0 }

```

## Contributing

See the [contributing](Contributing.md) guide for detailed instructions on how to get started with this project.

## License

The project is made available under the MIT license. See the [license](License.md) file for more information.
