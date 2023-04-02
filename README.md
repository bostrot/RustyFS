# RustyFS - would be a great name for a filesystem

RustyFS is a minimal multithreading file server written in Rust, inspired by the Rust example. It allows for easy and efficient file sharing between clients and servers through its multi-threaded architecture. With RustyFS, users can transfer files with ease while enjoying the speed and safety provided by the Rust programming language.

## Getting Started

To get started with RustyFS, you'll need to have Rust installed on your machine. Once you have Rust installed, you can install RustyFS by running the following command in your terminal:

## From releases

```bash
./rustyfs --path /path/to/directory --port 7878 --threads 4
```

## From source

Once RustyFS is downloaded, you can start the file server by running the following command:

```bash
cargo run -- --path /path/to/directory --port 7878 --threads 4
```

To download a file from RustyFS, you can use a web browser to navigate to http://localhost:7878/path/to/file.

To list the files in the current directory of RustyFS, you can use a web browser to navigate to http://localhost:7878/.

## Contributing

If you'd like to contribute to RustyFS, please feel free to open an issue or submit a pull request on GitHub. We welcome all contributions!

## License

RustyFS is open source software licensed under the MIT license. See the LICENSE file for more information.
