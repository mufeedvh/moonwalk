<div align="center">
  <h1><code>moonwalk</code></h1> 
  <p><strong><em>Cover your tracks during Linux Exploitation / Penetration Testing by leaving zero traces on system logs and filesystem timestamps.</em></strong></p>
  <img height="90" width="90" src="https://user-images.githubusercontent.com/26198477/146671442-78bb6781-b283-4f43-8754-d1d3b62ae627.gif">
  <img height="90" width="90" src="https://user-images.githubusercontent.com/26198477/146671305-5ffc26b4-1e0e-4436-9a1e-1e0dfc81f40e.gif">
</div>

---

## 📖 Table of Contents

- [Introduction](#%E2%84%B9%EF%B8%8F-introduction)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Contribution](#contribution)
- [License](#license)

## ℹ️ Introduction

**moonwalk** is a 400 KB single-binary executable that can clear your traces while penetration testing a **Unix** machine. It saves the state of system logs pre-exploitation and reverts that state including the filesystem timestamps post-exploitation leaving zero traces of a _ghost in the shell_.

⚠️ **NOTE:** This tool is open-sourced to assist solely in [**Red Team**](https://en.wikipedia.org/wiki/Red_team) operations and in no means is the author liable for repercussions caused by any prohibited use of this tool. Only make use of this in a machine you have permission to test.

## Features

- **Small Executable:** Get started quickly with a `curl` fetch to your target machine.
- **Fast:** Performs all session commands including logging, trace clearing, and filesystem operations in under 5 milliseconds.
- **Reconnaissance:** To save the state of system logs, `moonwalk` finds a world-writable path and saves the session under a dot directory which is removed upon ending the session.
- **Shell History:** Instead of clearing the whole history file, `moonwalk` reverts it back to how it was including the invokation of `moonwalk`.
- **Filesystem Timestamps:** Hide from the Blue Team by reverting the access/modify timestamps of files back to how it was using the [`GET`](#usage) command.

## Installation

```
$ curl -L https://github.com/mufeedvh/moonwalk/releases/download/v1.0.0/moonwalk_linux -o moonwalk
```

(`AMD x86-64`)

**OR**

Download the executable from [**Releases**](https://github.com/mufeedvh/moonwalk/releases) OR Install with `cargo`:

    $ cargo install --git https://github.com/mufeedvh/moonwalk.git
    
[Install Rust/Cargo](https://rust-lang.org/tools/install)

## Build From Source

**Prerequisites:**

* [Git](https://git-scm.org/downloads)
* [Rust](https://rust-lang.org/tools/install)
* Cargo (Automatically installed when installing Rust)
* A C linker (Only for Linux, generally comes pre-installed)

```
$ git clone https://github.com/mufeedvh/moonwalk.git
$ cd moonwalk/
$ cargo build --release
```

The first command clones this repository into your local machine and the last two commands enters the directory and builds the source in release mode.

## Usage

<div align="center">
  <table>
    <tr>
      <td><img height="300" width="400" src="https://user-images.githubusercontent.com/26198477/146672354-9db1e7e5-bb8a-43e5-8b64-b2d1bbea547e.png"></td>
    </tr>
  </table>
</div>

Once you get a shell into the target Unix machine, start a moonwalk session by running this command:

    $ moonwalk start
    
While you're doing recon/exploitation and messing with any files, get the `touch` timestamp command of a file beforehand to revert it back after you've accessed/modified it:

    $ moonwalk get ~/.bash_history
    
Post-exploitation, clear your traces and close the session with this command:

    $ moonwalk finish
    
That's it!

## Contribution

Ways to contribute:

- Suggest a feature
- Report a bug
- Fix something and open a pull request
- Help me document the code
- Spread the word
- Find something I missed which leaves any trace!

## License

Licensed under the MIT License, see <a href="https://github.com/mufeedvh/moonwalk/blob/master/LICENSE">LICENSE</a> for more information.

## Liked the project?
Support the author by buying him a coffee!

<a href="https://www.buymeacoffee.com/mufeedvh" target="_blank"><img src="https://cdn.buymeacoffee.com/buttons/default-orange.png" alt="Buy Me A Coffee" height="51px" width="217px"></a>

----

***Support this project by starring ⭐, sharing 📲, and contributing 👩‍💻! :heart:***

----
