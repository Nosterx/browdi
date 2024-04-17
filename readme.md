# BrowDi (Browser Dispatcher)

BrowDi (short for Browser Dispatcher) is an utility application that acts as a browser manager. It allows users to choose which web browser to use when opening links clicked outside of a browser context, such as in messengers, PDF viewers, and other applications. BrowDi is written in rust with Relm4 and LibAdwaita.

## Features

- **Ability to set default browser for a domain**: 
- **Shortcuts for every action**: 
- <H> to show/hide shortcuts
- <S> to show/hide full url
- <M> to open menu
- <Q> to quit
- One of other letter will be assigned to every browser button

## Installation

To install BrowDi, follow these steps:

1. Make sure you have the Rust toolchain installed. If not, you can download and install it from the official Rust website: [Install Rust](https://www.rust-lang.org/tools/install).

2. Install the `cargo-deb` extension by running the following command in your terminal:

    ```bash
    cargo install cargo-deb
    ```

3. Clone the BrowDi repository to your local machine:

    ```bash
    git clone https://github.com/Nosterx/browdi.git
    ```

4. Navigate to the BrowDi directory:

    ```bash
    cd browdi
    ```

5. Build and install BrowDi using Cargo and the `cargo-deb` extension:

    ```bash
    cargo deb --install
    ```

6. BrowDi is now installed on your system and ready to use as a customizable default browser manager.

## Usage
- Set as default browser in your system.
- Click link.
- Choose browser to open.

## License

BrowDi is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
