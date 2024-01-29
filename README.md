My first rust project. A library that is able to parse treasury quotes. For example
```rust
let q = String::from("102'18'5");
match parse(&q, Style::ShortNoteFuture) {
  Ok(quote) => println!("{} => {}", q, quote),
  Err(e) => println!("Could not parse quote {}, err: {:?}", q, e),
};
```

which prints as result: <code>102'18'5 => 102.578125.</code> This is the decimal price of the /ZT treasury future.


Another way to use the parser is to directly call parse from a String/str. Like so:
```rust
let result: f64 = "102'18'5".parse::<Quote>().unwrap().into();

println!("The parsed result is {}", result);
```
Which prints <code>The parsed result is 102.578125
