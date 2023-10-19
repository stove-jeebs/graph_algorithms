# Graph Analysis Assignment in Rust

This assignment involves various graph algorithms implemented in Rust, including finding the diameter of a graph and converting between different graph representations.

## Prerequisites

- Rust and Cargo: Ensure you have Rust and Cargo installed. If not, install them from [rust-lang.org](https://rust-lang.org/).

## Building the Project

To build the project, navigate to the project's root directory and run:

```bash
cargo build --release
```

This will create an optimized executable in the `./target/release/` directory.

## Running the Program

The program accepts various command-line arguments to perform different tasks:

```bash
./target/release/rust --<argument> < input_file > output.out
```

## List of arguments

- Task 1: `--size`
- Task 2: `--component_order`
- Task 3: `--matrix`
- Task 4: `--diameter`
- Task 5: `--girth`

### Checking Outputs

To check if the output is correct, you can use the `vim` diff tool:

```bash
vim -d <desired_output_file> output.out
```

Replace `desired_output_file` with the path to the expected output in the `sample_output` folder

More details of the assigment can be found inside [A4.pdf](https://github.com/stove-jeebs/graph_algorithms/blob/main/A4.pdf)
