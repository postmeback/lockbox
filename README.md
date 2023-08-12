# Lockbox
<img src="https://i.imgur.com/ZV550bh.jpg" alt="Lockbox" width="100%">


Lockbox is a command-line tool for generating and managing passwords. It uses strong encryption algorithms to securely store your passwords, so you can be sure that your data is safe.

[![codecov](https://codecov.io/gh/SonuBardai/lockbox/branch/main/graph/badge.svg?token=XV39653NA5)](https://codecov.io/gh/SonuBardai/lockbox)

### Usage
- To use Lockbox, first make sure you have [Rust installed](https://www.rust-lang.org/tools/install) on your system.
- Then, clone this repository with `git clone git@github.com:SonuBardai/lockbox.git`.
- You can run it using `cargo run`. Here’s an overview of the available commands:

<img src="https://i.imgur.com/PIj6o1h.png" alt="Lockbox" width="100%">

```rust
Usage: lockbox <COMMAND>

Commands:
  add       Add a new password to the password manager
  generate  Generate a random password.
  list      List all passwords in the password manager
  remove    Remove a password from the password manager
  show      Show a specific password in the password manager
  repl      Start an interactive REPL session
  help      Print this message or the help of the given subcommand(s)
```

- You can directly trigger the lockbox REPL by running `cargo run`
```rust
Welcome to L🦀CKBOX!

Please enter the master password
>> 

Enter [1] add password [2] generate random password [3] list passwords [4] remove password [5] show password [6] exit
>> 1
[1] generate random password [2] enter your own password [3] cancel
>> 1
5NqTd62DSpthlwOO (Copied to clipboard)
Please enter the service name
>> github     
Please enter the username (Optional)
>> MyAwesomeGithubProfile
Password added successfully

Enter [1] add password [2] generate random password [3] list passwords [4] remove password [5] show password [6] exit
>> show
Please enter the service name
>> github
Please enter the username (Optional)
>> MyAwesomeGithubProfile
Password: 5NqTd62DSpthlwOO

Enter [1] add password [2] generate random password [3] list passwords [4] remove password [5] show password [6] exit
>> exit
```

### Working
[Store](./src/store/README.md)

### Contributing
Contributions are welcome! If you’d like to contribute, please feel free to open an issue or submit a pull request. Checkout our [CONTRIBUTING](CONTRIBUTING.md) file for details on how to contribute.

### License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
