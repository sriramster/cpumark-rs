# CpuMark Rust 

A CPU benchmarking tool in native-rust inspired from [[https://github.com/sergev/duremark][duremark]] or CoreMark. 
The advantage is the precision with clock ticks the resolution of CLOCKS_PER_SEC is not guranteed by the Operating system. But 
Rust gurantee's this value. 

# Kernels
- integer kernels
- memory kernels (L1, L2, DRAM)
- Branch prediction
- List operations

TODO
- Matrix
- Math ops


# To build 

```bash
$ cargo build 
$ ./target/debug/cpumark-rs 

CpuMark starting...
Config: Config { duration_secs: 5.0, mem_size: 33554432, seed: 305419896 }
Iterations   : 7891
Time (s)     : 5.000
Checksum     : 0x69a994ab2a
CpuMark-rust Score: 1578.06


```

# LICENSE

This project is licensed under the MIT License. See the [[LICENSE][LICENSE]] file for details.
