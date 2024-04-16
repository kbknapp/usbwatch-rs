# usbwatch-rs

![Rust Version][rustc-image]
[![crates.io][crate-image]][crate-link]
[![Documentation][docs-image]][docs-link]
[![Dependency Status][deps-image]][deps-link]

Monitor USB events and execute actions based on rules.


<!-- vim-markdown-toc GFM -->

* [Example](#example)
    * [Defining Devices](#defining-devices)
    * [Defining Rules](#defining-rules)
    * [Running](#running)
* [Contributing](#contributing)
* [License](#license)
        * [Contribution](#contribution)

<!-- vim-markdown-toc -->

`usbwatch` works by monitoring UDEV events for a specific "device" or "port" (or
combination), and executing actions based off user defined "rules."

At runtime, you tell `usbwatch` which rule sets to utilize.

# Example

In this example we use `usbwatch` to execute an `echo` command depending on if
the device was plugged in (`add` event), or unplugged (`remove` event). In a
real world scenario, you may want to run a shell script instead of just a
simple `echo` command.

## Defining Devices

First, we need a file describing either the *port* or *device* we'd like to act
on. We do this by creating YAML files with the relevant details to match on. In
this example we'll only be matching on the *device*.

If we don't already have the device details handy, we can use `usbwatch
listen` to display events and record the appropriate details.

The `usbwatch listen` command will show all device and port events as they
happen, but we're only interested in one device, so we'll add the
`--num-events=1` argument will tell the `usbwatch listen` command to exit after
it sees the first event.

It's best to start with the target USB device already unplugged, as the
`remove` event carries very few details about the device that was unplugged.

> **Note**
> Internally `usbwatch` can utilize the more detailed "add" information even on
> a "remove" event because it keeps state of which devices are plugged
> in to which ports, however it can only record this state if it sees the
> initial `add` event for the device).

Once we tell `usbwatch` to listen, we plug in the target device to cause an
event.

If we include the `--output` argument then `usbwatch` will save our device
info. Here we use `ex1.yml`. The name of the file is arbitrary and just serves
to reference this device by when writing rules.

> **NOTE**
> In more advanced rules we can have human readable names that serve as
> identifies to individual devices and ports by manually adding a `name: foo`
> key to either the port or device definition. This example does not do that
> for simplicity.

```sh
$ usbwatch listen \
    --only devices \
    --output ex1.yml \
    --num-events 1

  Listening for device events...
```

Now either plug in, or unplug the device.

```sh
  Listening for udev events...
  Received 1 event...
  Writing output to ex1.yml
$ ls
ex1.yml
```

This is what our devices file looks like:

```yaml
---
devices:
  - ID_MODEL: 'Ultra_Fit'
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

We could place other devices in the same file if we wanted to match on *any* of
the devices in that file against a rule.

Additionally, we could omit or remove any of the fields in the devices field, if
we wished to be *less* specific. I.e. if we wanted to match *all* `SanDisk`
devices, we could remove all fields except `ID_VENDOR`.

## Defining Rules

Now we can create a rule using the device we enumerated above. We can either
write the rule in a text document by hand, or use the CLI. In this example
we'll use the CLI, but then show what the plain text rule looks like.

```sh
$ usbwatch create-rule \
  --name "Example Cruzer Connect"
  --devices-file ex1.yml \
  --on add \
  --execute 'echo "Example was plugged in!" >> usb.log' > ex1_connect.yml
```

We can view the rule, and even make changes by hand if we needed to.

```yaml
---
rules:
  - name: "Example Cruzer Connect"
    command: "echo 'Example was plugged in!' >> usb.log"
    match:
      on: add
      devices:
      - include_devices: ex1.yml
```

> **Warning**
> If you have `usbwatch` running as `root` in a service such as via systemd,
> the `.yml` rules files should only be writable by `root` (permissions `0600`
> owned by `root:root`), otherwise you're giving `root` access to anyone who
> can write to these files and cause a USB event to occur.

In the rule file, the device information is pulled from the file we created
earlier, however we could also have included the device information inline
instead.

> **NOTE**
> Our example matches just the single device when connected to *any* port.
> However, had we created a ports file in a similar manner to the devices file,
> we could use the `--ports` flag on `usbwatch create-rule` to limit which
> ports are matched. This would have added a `ports:` key to the YAML file.

## Running

Now that we've defined the *devices* and the *rules* we can pass these to the
`usbwatch run` command via thier respective arguments.

```sh
$ usbwatch run --rules ex1_connect.yml --devices ex1.yml
```

You should now see a new file created `usb.log` with a new line each time you
plug in that device to any port. You should *not* see a new line appended when
you plug in any other device.

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
