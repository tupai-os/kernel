# Kernel

---

This is Tupai's kernel.

## Building

**NOTE: The kernel is designed to be built from the root repository. The following instructions may be outdated.**

To build the kernel, run the following command in the root directory of the repository:

```
make all TARGET=<target>
```

where `<target>` is one of `x64`, `i386` or `rpi2`.

## TODO

- [ ] Clean up all 'TODO's in source code
- [ ] Investigate deadlock problems associated with the use of `Weak` references in the premptive scheduler (`src/thread/preempt.rs`)
