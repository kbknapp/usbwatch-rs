# usbwatch-rs

Monitor USB events and execute actions

`usbwatch` works by monitor for a specific "device", and executing actions based
off "rules." Devices can also be grouped together to form "device lists" and
used in rules to reduce duplication.

The rules allow flexible conditions such as, "any device in ..." or "any device
exept ..." or "only device ...". Rules can be further grouped into "rule sets."
At runtime, you tell `usbwatch` which rule sets to utilize.

# Example

In this example we use `usbwatch` to create a *device*, and execute an `echo`
command depending on if the device was plugged in, or unplugged. In a real world
scenario, you may want to run a shell script instead of just a simple `echo`
command.

First, if we don't already have the device details handy, we can use `usbwatch`
to listen for the device and record the appropriate details.

You can start with the USB device attached or detached. Once we tell `usbwatch`
to listen, we either plug in or unplug the device to cause an event. We also
need to provide a file to save our device info to, here we use `ex1.device`. The
name is arbitrary, and just serves as a human readable ID which can use later to
reference this device by.

``` sh
$ usbwatch create-device --output ex1.usbdevice
  Listening for device events...
```

Now eithr plug in, or unplug the device.

``` sh
  Listening for device events...
  Found device ID abcdef
  Saving information to ex1.usbdevice
$ ls
ex1.device
```

Now we can create a rule using this device. We can either write the rule in a
text document by hand, or use the CLI. In this example we'll use the CLI, but
then show what the plain text rule looks like.

``` sh
$ usbwatch create-rule --device ex1.usbdevice --on connect --execute "echo 'Example was plugged in!'" > ex1_connect.yml
```

We can view the rule, and even make changes by hand if we needed to. 

``` sh
$ cat ex1_connect.yml
rule:
  device: abc123 
  on: CONNECT 
  command: "echo 'Example was plugged in!'"
```

**SECURITY**: If you have `usbwatch` running as `root` in a service such as via
systemd, the `.yml` rules files should only be writable by `root` (permissions
`600` owned by `root:root`), otherwise you're giving `root` access to anyone who
can write to these files and cause a USB event to occur.

Rules are evaluated in top down order.

Rules can be created by hand, or using the `usbwatch` CLI.
