# Catalog searcher

A demo product catalog searcher written in Rust

## How to build?

### Requirements

- rustc
- cargo

### Building - Development

For executing the app in the development mode you can use the following command:

```sh
$ cargo run
```

### Building - Release

To build the project, run the command bellow:

```sh
$ cargo build --release
```

And then execute the binary

```sh
$ ./target/release/search-test $CATALOG_FILENAME
```

## Executing the app

The app recieves only one command, which is the source file to index the products. It must be a JSON containing a list of products with:

- id
- name
- price

Whereas all of the attributes are strings, below you can se what a product must look like

```json
{ "id": "1", "name": "Foo", "price": "10.24" }
```

### Arguments

| argument | type   |
| -------- | ------ |
| filename | string |

### Example

```sh
$ cargo run data/dump.json # Running in development
```

## Author

| ![Eder Lima](https://github.com/asynched.png?size=100) |
| ------------------------------------------------------ |
| [Eder Lima](https://github.com/asynched)               |
