# BrowDi

BrowDi (short for Browser Dispatcher) is an utility application that acts as a browser manager. It allows users to choose which web browser to use when opening links clicked outside of a browser context, such as in messengers, PDF viewers, and other applications. BrowDi is written in rust with Relm4 and LibAdwaita.

## Features
**Ability to set default browser for a domain**: 
+ Pic
![изображение](https://github.com/Nosterx/browdi/assets/4470993/c1c2cefa-2d2f-49b1-b273-4df8fdaee5dd)

**Shortcuts for every action**: 
+ `<H>` to show/hide shortcuts
+ `<S>` to show/hide full url
+ `<M>` to open menu
+ `<Q>` to quit
+ '<D>' to set browser as a default for a domain of current url
+ One of other letter will be assigned to every browser button
![изображение](https://github.com/Nosterx/browdi/assets/4470993/9c4eba60-15ae-4192-9710-06dc1bb23ba1)


## Installation

### Debian/Ubuntu
1. Download package from the releases [page](https://github.com/Nosterx/browdi/releases).
2. Install package with
   ```bash
   sudo dpkg -i ~/Downloads/browdi_0.1.1-1_amd64.deb
   ```
3. Set BrowDi as a default browser![image](https://github.com/Nosterx/browdi/assets/4470993/ccc3bd3e-86ac-466e-8fc9-8cb951d33dca)


### To build from source follow this steps:

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
+ Set as default browser in your system.
+ Click link.
+ Choose browser to open.

## Kudos
[Junction](https://github.com/sonnyp/Junction)

## TODO
- [ ] Autoset as default browser
- [ ] Option to show/hide browser names on/under buttons
- [ ] Add ability to display buttons on serveral rows

## FAQ
### How to make [QuteBrowser](https://github.com/qutebrowser/qutebrowser/tree/main) profiles generated with [QBPM](https://github.com/pvsr/qbpm/) (QuteBrowser Profile Manager) to be shown in the list of browsers displayed by BrowDi?
- Copy/move profile's desktop file from `~/.local/share/applications/qbpm/PROFILE_NAME.desktop` to `~/.local/share/applications/PROFILE_NAME.desktop`
- Add profile's desktop file into `~/.config/mimeapps.list`
```
x-scheme-handler/http=firefox_nightly.desktop;google-chrome.desktop;firefox_firefox.desktop;browdi.desktop;PROFILE_NAME.desktop;
x-scheme-handler/https=firefox_nightly.desktop;google-chrome.desktop;firefox_firefox.desktop;browdi.desktop;PROFILE_NAME.desktop;
```

## License

BrowDi is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
