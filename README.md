# Starlink CLI

A command line interface (CLI) tool to interact with your Starlink dish. This tool allows you to get device info, obstruction stats, device state, and speeds, as well as device alerts.

## Installation

Clone the repo:

```bash
git clone https://github.com/yourusername/starlink-cli.git
cd starlink-cli
```

To build the project:
```bash
cargo build --release
```

The executable will be located in target/release/starlink-cli.

## Usage

After building the project, you can run the tool with the following command:

./target/release/starlink-cli [SUBCOMMAND]

The available subcommands are:

    * alerts: Get alerts.
    * info: Get device info.
    * obstruction: Get obstruction stats.
    * state: Get device state.
    * speed: Get speed.

For example, to get device info, you would run:

```bash
./target/release/starlink-cli info
```

## Contributing
Contributions are welcome! Please feel free to submit a pull request.