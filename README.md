# mtr-rust

A learning-focused Rust implementation of `mtr`.

The current step is a small, educational `v0.5`:

1. Build ICMP Echo Request packets in Rust.
2. Send repeated IPv4 ICMP probes with a raw socket on macOS.
3. Walk TTL values one hop at a time.
4. Print a simple `mtr`-style summary table with per-hop packet loss and RTT
   statistics.

It is still intentionally limited:

1. IPv4 only.
2. No reverse DNS yet.
3. No full-screen TUI yet.
4. No long-running live refresh loop yet.

## Build

```bash
cargo build
```

## Version

```bash
./target/debug/mtr-rust --version
```

## Run

Raw sockets usually require elevated privileges on macOS, so run the binary
with `sudo`:

```bash
sudo ./target/debug/mtr-rust 8.8.8.8
```

By default, the program sends `10` probes per hop:

```bash
sudo ./target/debug/mtr-rust 8.8.8.8
```

You can choose a different probe count:

```bash
sudo ./target/debug/mtr-rust 8.8.8.8 --count 5
```

If socket creation fails, the program prints the operating system error so you
can see whether it is a permissions issue or something else. If no reply
arrives before the receive timeout, that probe counts as lost for the hop.

Example output:

```text
Hop  Host            Loss%  Sent  Recv  Last   Avg   Best   Wrst
1    192.168.1.1      0.0%    10    10    2.1    2.3    1.8    4.9
2    10.0.0.1        10.0%    10     9    8.2    9.1    7.8   13.4
```

## Roadmap

1. `v0.1`: Create and close a macOS ICMP raw socket.
2. `v0.2`: Build ICMP Echo Request packets and test checksum logic.
3. `v0.3`: Send one Echo Request and receive one reply.
4. `v0.4`: Add a tiny `--version` command.
5. `v0.5`: Add basic repeated probing and per-hop statistics.
6. Next: Refresh the table continuously instead of printing it once.
7. Later: Add reverse DNS lookups as an optional display feature.
8. Later: Grow that into a small, readable `mtr` implementation.
