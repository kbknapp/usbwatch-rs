# usbwatch-rs

![Rust Version][rustc-image]
[![crates.io][crate-image]][crate-link]
[![Documentation][docs-image]][docs-link]
[![Dependency Status][deps-image]][deps-link]

Monitor USB events and execute actions based on rules.


<!-- vim-markdown-toc GFM -->

* [Example](#example)
* [Contributing](#contributing)
* [License](#license)
        * [Contribution](#contribution)

<!-- vim-markdown-toc -->

`usbwatch` works by monitoring UDEV events for a specific "device" or "port" (or
combination), and executing actions based off user defined "rules."

At runtime, you tell `usbwatch` which rule sets to utilize.

# Example

In this example we use `usbwatch` to create a *device*, and execute an `echo`
command depending on if the device was plugged in (`add` event), or unplugged
(`remove` event). In a real world scenario, you may want to run a shell script
instead of just a simple `echo` command.

First, if we don't already have the device details handy, we can use `usbwatch
listen` to display events and manually record the appropriate details.

However, if we don't want to see all events, devices, and ports we can use a
more targeted `usbwatch create-device` command to listen for only a single
device event and write the file for us.

It's best to start with the target USB device unplugged, as the `remove` event
carries very few details about the device that was unplugged 

> **Note**
> Internally `usbwatch` can utilize the more detailed "add" information even on
> a "remove" event because it keeps state of which devices are plugged
> in to which ports, however it can only record this state if it sees the
> initial `add` event for the device). 

Once we tell `usbwatch` to listen, we plug in the device to cause an event. We
also need to provide a file to save our device info to, here we use
`ex1.device`. The name is arbitrary, and just serves as a human readable ID
which can use later to reference this device by.

``` sh
$ usbwatch create-device --output ex1.yml --name "My Cruzer"
  Listening for device events...
```

Now either plug in, or unplug the device.

``` sh
  Listening for device events...
  Found one device
  Saving information to ex1.yml
$ ls
ex1.yml
```

The devices file could be written by hand as well, as this is what our devices
file looks like:

```
---
devices:
  - name: "My Cruzer"
    ID_MODEL: 'Ultra_Fit'
    ID_MODEL_ENC: "Ultra\\x20Fit"
    ID_MODEL_FROM_DATABASE: "Ultra Fit"
    ID_MODEL_ID: "5583"
    ID_SERIAL: "SanDisk_Ultra_Fit_12330123260925119515"
    ID_SERIAL_SHORT: "12330123260925119515"
    ID_VENDOR: "SanDisk"
    ID_VENDOR_ENC: "SanDisk"
    ID_VENDOR_FROM_DATABASE: "SanDisk Corp."
    ID_VENDOR_ID: "0781"
    PRODUCT: "781/5583/100"
```

We could place other devices in the same file if we wanted to *any* of the
devices in that file against a rule.

Additionally, we could omit or remove any of the fields in the devices field, if
we wished to be *less* specific. I.e. if we wanted to match *all* `SanDisk`
devices, we could remove all fields except `ID_VENDOR`.

Now we can create a rule using this device. We can either write the rule in a
text document by hand, or use the CLI. In this example we'll use the CLI, but
then show what the plain text rule looks like.

``` sh
$ usbwatch create-rule \
  --name "Example Cruzer Connect"
  --devices-file ex1.yml \
  --on add \
  --execute "echo 'Example was plugged in!'" > ex1_connect.yml
```

We can view the rule, and even make changes by hand if we needed to. 

``` sh
$ cat ex1_connect.yml
---
rules:
  - name: "Example Cruzer Connect"
    match:
      on: add 
      devices: 
        - ex1.yml
    command: "echo 'Example was plugged in!' >> usb.log"
```

> **Warning** 
> If you have `usbwatch` running as `root` in a service such as via systemd,
> the `.yml` rules files should only be writable by `root` (permissions `600`
> owned by `root:root`), otherwise you're giving `root` access to anyone who
> can write to these files and cause a USB event to occur.

Our example matches just the single device when connected to *any* port. In the
rule file, the device information is pulled from the file we created earlier,
however we could also have included the device information inline. If `usbwatch
run --devices ex1.yml` is used to load a devices file, we could also have just
referenced the device by name `"My Cruzer"`.

If we had wanted to match just this one device on a specific port, similar steps
could be followed using the `create-port` subcommand, or manually copying the
information from the `listen` subcommand.

In order to run our rule, we use `usbwatch run --rules ex1_connect.yml`. You
should now see a new file created `usb.log` with a new line each time you plug
in that device to any port.

# Contributing

You'll need:

- A [Rust compiler][rustup]
- `libudev` header files installed (e.g. `systemd-devel` on Fedora, or
  `libudev-dev` on Ubuntu)

# License

This crate is licensed under either of

 * [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
 * [MIT license](http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly note otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[//]: # (badges)

[rustc-image]: https://img.shields.io/badge/rustc-1.59+-blue.svg
[crate-image]: https://img.shields.io/crates/u/usbwatch.svg
[crate-link]: https://crates.io/crates/usbwatch
[docs-image]: https://docs.rs/usbwatch/badge.svg
[docs-link]: https://docs.rs/usbwatch
[deps-image]: https://deps.rs/repo/github/kbknapp/usbwatch-rs/status.svg
[deps-link]: https://deps.rs/repo/github/kbknapp/usbwatch-rs

[//]: # (links)

[rustup]: https://rustup.rs
