# DeDup CLI

Simple command line utility for removing duplicate lines from input. Like 
[`uniq`](http://man7.org/linux/man-pages/man1/uniq.1.html) but not just adjacent lines.

The standard advice for removing duplicates from a pipeline is either `sort | uniq` or just `sort -u`. That works, but 
can be suboptimal. `sort` serializes the input. On multi-cpu machines this can prevent all of the cores from being 
utilized. Also, sorting the input isn't free, imposing an O(_n_ log _n_) cost.

[`dedup`]() scans for duplicates incrementally, keeping a hash-set of lines it has seen. Hash look-up and storage is 
O(_c_). And because it scans a line at a time, `dedup` can output a new line immediately, silently dropping 
subsequent ones as they appear.

## Installation

Currently, the easiest way to install is via cargo. Checkout the project and run `cargo install`.

```bash
cargo install -path .
```

## Usage

```bash
producer | dedup | consumer
```

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License
[MIT](https://choosealicense.com/licenses/mit/)