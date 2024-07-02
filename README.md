# Protocolyt

![](https://imgur.com/7J6IpPC.png)

# Install
### You need:
- Rust **(installed)** `Only for compilation`
- GIT **(installed)** `For files download`

### And run
- `.ps1` file if you use Windows
- `.sh` file if you use Linux or Mac Os

# Exemples
- Hello, world!
```rust
const vec := @import("vector");
const asciz := @import("asciz");
const io := @import("io");
const stdout := io.writer().unwrap();

fn main(args: vec[asciz]) @void {
  stdout.println("Hello, world!");
}
```

- Printing type
```rust
const vec := @import("vector");
const asciz := @import("asciz");
const io := @import("io");
const stdout := io.writer().unwrap();

fn main(args: vec[asciz]) @void {
  stdout.debug(args);
  // or using the TYPES module
  const types := @import("types");
  stdout.println(types.from(args).unwrap());
}
```

- Option union
```rust
const vec := @import("vector");
const io := @import("io");
const asciz := @import("asciz").ASCIz;
const option := @import("uncertain").Option;

fn main(args: vec[asciz]) @void {
  const stdout := io.writer().unwrap();

  const opt: option[asciz] = option.init::[asciz]();
  stdout.debug(t);
  const val: asciz = asciz.init().from("Hello, world!").unwrap();
  opt.Some(val);
  stdout.debug(opt);
  stdout.println(opt.unwrap());
}
```