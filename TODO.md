- [x] strings
- [x] constants, chars, strings
- [x] allow labels/symbols in directive declarations e.g. `.u8 foo, bar`
- [x] add ports
- [x] add emulation
- [x] reg of token enum needs to be generic
- [x] relative and absolute addressing
- [x] make the project generic over the ISA
- [x] add support for graphics
- [x] opcode for token enum should be generic
- [x] anonymous labels
- [ ] macros
- [ ] Step simulation / debugger
- [ ] Function scopes with `{}`
- [ ] Output to logisim
- [ ] implement forwarding/hazard logic
    - how to do this in a generic way?
- [ ] fixed-point constants (i0.8, i5.3, i8.16, i32.32, etc.)
- [ ] include files
- [ ] evaluate math expressions e.g. `table + arr[2+str]`
- [ ] implement multi-instruction pseudo-ops for some architectures as macros
- [ ] implement symbols for ports which need to be CPU-defined
- [ ] CPU interrupts


```
cargo clippy -- -W clippy::all -W clippy::pedantic -W clippy::restriction -W clippy::nursery -A clippy::implicit_return -A clippy::unseparated_literal_suffix -A clippy::pub_use -A clippy::std_instead_of_core
```