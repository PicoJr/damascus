# Damascus

Interleave 2 byte streams with default payload fallback when one stream is shorter than the other.

## Example

with files:

```text
echo -ne "AA" > /tmp/as
echo -ne "BBBB" > /tmp/bs
echo -ne "CC" > /tmp/default
cargo run --release /tmp/as /tmp/bs /tmp/default
# writes "AABBCCBB" to stdout
```

> here payload size = 2 bytes

## Performance

with unix named pipes:

```text
mkfifo /tmp/fifo1
mkfifo /tmp/fifo2
head -c 1K < /dev/zero > /tmp/default
seq 10000000 > /tmp/fifo1 &
seq 10000000 > /tmp/fifo2 &
```

> payload size = 1KB

```text
./target/release/damascus /tmp/fifo1 /tmp/fifo2 /tmp/default | pv > /dev/null
[1]  - 37875 done       seq 10000000 > /tmp/fifo2
[2]  + 37899 done       seq 10000000 > /tmp/fifo1
 150MiB 0:00:00 [1.35GiB/s] [   <=>
```

> CPU: AMD Ryzen 7 3700X (16) @ 3.600GHz

Note: payload size (i.e. `default` size) massively impacts throughput:

| payload size      | throughput |
|-------------------|-----------:|
| 2 bytes           |   ~50MiB/s |
| 32 bytes          |   612MiB/s |
| 64 bytes          |  1018MiB/s |
| 500 bytes         |  1.25GiB/s |
| 1KB               |  1.35GiB/s |