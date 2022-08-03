## rust-plasma-simulation-examples

This repository contains a set of basic plasma physics simulations implemented based on Lubos Brieda's *Plasma Simulations by Example*.

The author of the book provides simulation code written in C++, but here the simulations have been re-implemented in Rust. I undertook this project to challenge myself and to better internalize the computational physics concepts outlined in the book.

### Book

*Plasma Simulations by Example* by Lubos Brieda can be found [here](https://www.amazon.com/gp/product/1138342327). As a disclaimer, I am not associated with the author and I have not been paid to promote this book. I find the text to be a useful resource for understanding the basics of computational plasma physics. The author is duly credited, as this repository represents a fair use derivative work of the author's original C++ simulation code.

![Cover of Plasma Simulations by Example](https://images-na.ssl-images-amazon.com/images/I/41sqjn4babL._SY291_BO1,204,203,200_QL40_ML2_.jpg)

### Execution

The plasma simulations can be run with the following Docker commands:

```
docker build -t plasma .
docker run -it --rm -v $(pwd):/usr/src/plasma-simulation --name plasma plasma
```

Or by executing the `run.sh` script, if you are in a Unix based environment.

If Rust is installed on your machine, simply run the following command:

```
cargo run
```

### Testing

Tests can be run via Docker using the following command:

```
docker run -it --rm -v $(pwd):/usr/src/plasma-simulation --name plasma plasma cargo test
```

Tests can be run outside of Docker as follows:

```
cargo test
```

### Code Formatting

All Rust code must be formatted by applying the `rustfmt` command.
