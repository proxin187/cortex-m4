<div align="center">

# Cortez

![GitHub License](https://img.shields.io/badge/license-MIT-red?style=for-the-badge&logo=none)
![dependencies](https://deps.rs/repo/github/proxin187/cortez/status.svg?style=for-the-badge)

An easy-to-use nRF52833/cortex-m4 emulator that that makes debugging and developing for cortex-m4 chips easier.

</div>

> [!WARNING]
> Cortez is still a work-in-progess project, only a small subset of the armv7m instruction set is implemented.


# Features
- [x] ELF Loader
- [x] Two Interfaces: Interactive/Minimal
- [x] Disassembly viewer
- [x] Easy-to-use
- [ ] Configurable peripherals and memory regions (coming soon)


# Usage

```
Usage: cortex-m4 [OPTIONS] <COMMAND>

Commands:
  interactive  a interactive emulator interface
  minimal      a minimal emulator interface with no tui
  help         Print this message or the help of the given subcommand(s)

Options:
  -d, --debug
  -h, --help     Print help
  -V, --version  Print version
```

# License

Cortez is licensed under the MIT-License.

# Sources
[nRF52833](https://infocenter.nordicsemi.com/pdf/nRF52833_PS_v1.3.pdf)
[ARM Cortex-M4 Processor](https://www.engr.scu.edu/~dlewis/book3/docs/Cortex-M4%20Proc%20Tech%20Ref%20Manual.pdf)
[ARM v7-M Architecture Reference Manual](https://web.eecs.umich.edu/~prabal/teaching/eecs373-f10/readings/ARMv7-M_ARM.pdf)


