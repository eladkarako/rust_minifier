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
  [FILES]...
          Input files or directories to minify.
          
          Can be individual `.rs` or `.toml` files, or directories to recursively search. If
          omitted, reads from stdin.

Options:
  -s, --suffix <SUFFIX>
          Suffix to append to minified output filenames.
          
          For example, with suffix `_minified`, `main.rs` becomes `main_minified.rs`. Only
          used when writing individual files (not with `--concat-to`).
          
          [default: _minified]

      --concat-to <DESTINATION>
          Destination for concatenated output.
          
          Can be A file path (writes minified content to that file), a directory path (creates
          a timestamped file in that directory), `STDOUT` (or empty - prints to standard
          output, default).
          
          When specified, individual minified files are not written to disk.
          
          [default: STDOUT]

      --include-file-path
          Include source file paths as comments in output.
          
          When enabled, each minified file is prefixed with a comment containing its full
          path. Useful for tracking which source files contributed to concatenated output.

      --exclude <EXCLUDE>
          Exclude files matching these patterns (non-case-sensitive substring match on
          filename).
          
          Files whose names contain any of these patterns are skipped during processing.
          Matching is case-insensitive and applied only to the filename, not the full path.
          
          for example: rust_minifier src/ --exclude test --exclude mock will exclude files
          like: `test_helper.rs`, `mock_data.toml`, `my_test.rs`

      --soft-split <SOFT_SPLIT>
          Try to re-distribute long content by maximum amount of characters you wish to allow
          in a file. it generates multiple output files each having '_part_XX_of_YY' suffix
          
          [default: unlimited]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

### uses
- drag and drop any `.rs` or `.toml` over the execute as an argument to generate a minified file.
- run `rust_minifier . --concat-to all.txt` to let rust_minifier recursively dig-though all your files and folders (in parallel - highly advised! very fast!) and join everything after minification to `all.txt` in current folder.
- you can use it in your own batch files and first recursively collect the paths to all your `.rs`/`.toml` files in your project, before executing this program ONCE with all those as arguments, it works in parallel. then walk through to join all the files with `_minified` in it, they already include their fully qualified path as the first line in them (by default) so you can easily create one use file of all your project's structure.
- if your A.I. code agent is chat-based, you can try to split a long output to multiple files based on maximum character length limit using `--soft-split`, it only effective when using with `--concat-to` to a file.

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
