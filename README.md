# rocket-rust
restful api made in [Rust](https://www.rust-lang.org)


## Install
Clone this repo

    git clone git@github.com:HomeInside/rocket-rust.git


## Build
```bash
cargo build
```

```bash
cargo run
```

and visit http://localhost:8000 to see this application in action!


## Deploy
first, inspect the **Rocket.toml** file

## Generate a new secret_key
When manually specifying the secret key, the value should a 256-bit base64 encoded string.
Such a string can be generated with the openssl command line tool:

```bash
openssl rand -base64 32
```

now paste the generated secret_key to **Rocket.toml**

```yaml
[production]
address = "0.0.0.0"
port = 8080
workers = 2
log = "critical"
secret_key = "8Xui8SN4mI+7egV/9dlfYYLGQJeEx4+DwmSQLwDVXJg="
limits = { forms = 32768 }
```



## run the server
```bash
ROCKET_ENV=development ./target/release/rocket-rust
```

and visit http://localhost:6969 to see this application in action!


## Documentation
First read [Rocket Guide](https://rocket.rs/guide/)


## license
The full text of the license can be found in the file **LICENSE-MIT**


## Contact
[HomeInside](https://github.com/HomeInside/)
