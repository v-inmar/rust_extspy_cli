# Extension Spy CLI (extspy cli)

_Rust Learning Journey_

A command line app for checking out all the files within a directory and grabbing their extensions and size.

It recursively check child directories for files and adds all the bytes based on their extension.

## Run Locally

Clone the project

```bash
    git clone https://github.com/v-inmar/rust_extspy_cli
```

Go to the project directory

```bash
    cd rust_extspy_cli
```

Install dependencies

```bash
    cargo build
```

Run the app

```bash
    cargo run /path/to/the/directory/
```

Example output

```bash
    Total bytes: 11165728917
    +------------------------------------------------------------------+------------+----------+
    | Extension                                                        | Bytes      |          |
    +------------------------------------------------------------------+------------+----------+
    | ape                                                              | 200708     | 0.002 %  |
    +------------------------------------------------------------------+------------+----------+
    | go                                                               | 295420698  | 2.646 %  |
    +------------------------------------------------------------------+------------+----------+
    | APACHE                                                           | 137006     | 0.001 %  |
    +------------------------------------------------------------------+------------+----------+
```
