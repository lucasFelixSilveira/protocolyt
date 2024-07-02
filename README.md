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

  // Starts the OPTION inside memory.
  const opt: option[asciz] = option.init::[asciz]();
  
  // Prints the type corresponding to OPTION, so that we 
  // can see the change between the initial and final types.
  stdout.debug(opt);
  
  // Starts a new String in memory and sets it to the literal content of "Hello, world!"
  // Sets that OPTION now responds to the SOME type. 
  //
  // The variable placed within the SOME function below has its memory
  // copied into OPTION, and as soon as there is an UNWRAP, the memory
  // is recovered accessible again.
  const val: asciz = asciz.init().from("Hello, world!").unwrap();
  opt.Some(val);

  // Print the current type of UNION.
  stdout.debug(opt);
  
  // Test whether the OPTION is working properly by printing the result
  // and seeing if it was as expected
  stdout.println(opt.unwrap());
}
```