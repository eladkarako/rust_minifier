# rust_minifier

![](resources/logos/app.png)

<hr/>

<details><summary><h3>👇  download</h3></summary>

### download (direct URLs - latest release)

#### Windows
- [![](resources/logos/64.png) ![](resources/logos/windows.png) x86_64-pc-windows-msvc.zip](https://github.com/eladkarako/ram_report/releases/latest/download/x86_64-pc-windows-msvc.zip)
- [![](resources/logos/64.png) ![](resources/logos/windows.png) aarch64-pc-windows-msvc.zip](https://github.com/eladkarako/ram_report/releases/latest/download/aarch64-pc-windows-msvc.zip)
- [![](resources/logos/32.png) ![](resources/logos/windows.png) i686-pc-windows-msvc.zip](https://github.com/eladkarako/ram_report/releases/latest/download/i686-pc-windows-msvc.zip)

#### Android (NDK `v30.0.16248370`, SDK `v21`)
- [![](resources/logos/64.png) ![](resources/logos/android.png) aarch64-linux-android.zip](https://github.com/eladkarako/ram_report/releases/latest/download/aarch64-linux-android.zip)
- [![](resources/logos/32.png) ![](resources/logos/android.png) armv7-linux-androideabi.zip](https://github.com/eladkarako/ram_report/releases/latest/download/armv7-linux-androideabi.zip)
- [![](resources/logos/64.png) ![](resources/logos/android.png) x86_64-linux-android.zip](https://github.com/eladkarako/ram_report/releases/latest/download/x86_64-linux-android.zip)
- [![](resources/logos/32.png) ![](resources/logos/android.png) i686-linux-android.zip](https://github.com/eladkarako/ram_report/releases/latest/download/i686-linux-android.zip)

#### Linux (gnu)
- [![](resources/logos/64.png) ![](resources/logos/linux.png) x86_64-unknown-linux-gnu.zip](https://github.com/eladkarako/ram_report/releases/latest/download/x86_64-unknown-linux-gnu.zip)
- [![](resources/logos/32.png) ![](resources/logos/linux.png) i686-unknown-linux-gnu.zip](https://github.com/eladkarako/ram_report/releases/latest/download/i686-unknown-linux-gnu.zip)
- [![](resources/logos/64.png) ![](resources/logos/linux.png) aarch64-unknown-linux-gnu.zip](https://github.com/eladkarako/ram_report/releases/latest/download/aarch64-unknown-linux-gnu.zip)
- [![](resources/logos/32.png) ![](resources/logos/linux.png) armv7-unknown-linux-gnueabi.zip](https://github.com/eladkarako/ram_report/releases/latest/download/armv7-unknown-linux-gnueabi.zip)
- [![](resources/logos/32.png) ![](resources/logos/linux.png) armv7-unknown-linux-gnueabihf.zip](https://github.com/eladkarako/ram_report/releases/latest/download/armv7-unknown-linux-gnueabihf.zip)

#### Linux (musl)
- [![](resources/logos/64.png) ![](resources/logos/linux.png) x86_64-unknown-linux-musl.zip](https://github.com/eladkarako/ram_report/releases/latest/download/x86_64-unknown-linux-musl.zip)
- [![](resources/logos/32.png) ![](resources/logos/linux.png) i686-unknown-linux-musl.zip](https://github.com/eladkarako/ram_report/releases/latest/download/i686-unknown-linux-musl.zip)
- [![](resources/logos/64.png) ![](resources/logos/linux.png) aarch64-unknown-linux-musl.zip](https://github.com/eladkarako/ram_report/releases/latest/download/aarch64-unknown-linux-musl.zip)
- [![](resources/logos/32.png) ![](resources/logos/linux.png) armv7-unknown-linux-musleabi.zip](https://github.com/eladkarako/ram_report/releases/latest/download/armv7-unknown-linux-musleabi.zip)
- [![](resources/logos/32.png) ![](resources/logos/linux.png) armv7-unknown-linux-musleabihf.zip](https://github.com/eladkarako/ram_report/releases/latest/download/armv7-unknown-linux-musleabihf.zip)

### other
- [changelog.txt](https://github.com/eladkarako/ram_report/releases/latest/download/changelog.txt)
- [version.txt](https://github.com/eladkarako/ram_report/releases/latest/download/version.txt)

</details>

<hr/>

### `--help`

```
whitespace-based rust-code minification, with comment and rustdoc removing. and TOML
minification, parallel processing.

Usage: rust_minifier.exe [OPTIONS] [FILES]...

Arguments:
  [FILES]...  Files to minify (leave empty for stdin)

Options:
  -s, --suffix <SUFFIX>       Output suffix for minified files [default: _minified]
      --concat-output <PATH>  Concatenate all output to this file
      --concat-to-stdout      Write to stdout instead of files as each thread finishes
                              [default: false]
      --include-file-path     file's path as a comment in the minified file's head [default:
                              true]
  -h, --help                  Print help
  -V, --version               Print version
```

### uses
- drag and drop any `.rs` or `.toml` over the execute as an argument to generate a minified file.
- you can use it in your own batch files and first recursively collect the paths to all your `.rs`/`.toml` files in your project, before executing this program ONCE with all those as arguments, it works in parallel. then walk through to join all the files with `_minified` in it, they already include their fully qualified path as the first line in them (by default) so you can easily create one use file of all your project's structure.

### use cases.
- pass your code to A.I. code-generator for code-review more easily.
- strip away all your old comments and rustdoc before creating new-ones.
- a step before updating `rustfmt.toml` and running `cargo fmt --verbose --message-format human`, when you truly want to clean up old styles.

<hr/>

this is a free and open-source program that works across all OS,  
built with the assist of `GitHub's Copilot` and `Claude Haiku 4.5`,  
logos and infographics generated with `GPT Image 2`.  
written in Rust, with JetBrains RustRover IDE Community.  

enjoy!

<br/>
<hr/>

<a href="https://paypal.me/31adkarak0" target="_blank" rel="noopener noreferrer"><img src="resources/logos/paypal1.svg"><br /><img src="resources/logos/paypal2.png"></a>




<hr/>
