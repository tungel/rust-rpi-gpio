This program controls the fan on my Raspberry Pi 4 when it gets hot.

Cross build for rpi4:

```
cross build --release --target aarch64-unknown-linux-musl --verbose

# copy over:
scp target/aarch64-unknown-linux-musl/release/raspberrypi-gpio root@x.x.x.x:/tmp/
```

Run it:

```
(/tmp/raspberrypi-gpio  >/dev/null 2>&1 )&
```

