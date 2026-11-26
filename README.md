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

## what it does

`rust_minifier` removes comments and redundant whitespace from source code.
It is not limited to Rust: it supports 90 text-based file types, including
web files (HTML, CSS, JavaScript, TypeScript), Android and JVM files
(Java, Kotlin, Gradle, AIDL, ProGuard, Smali), data files (JSON, YAML, TOML,
SQL, GraphQL), build files (Dockerfile, Makefile, CMake, HCL, Nix) and many
scripting and systems languages.

Comments are always removed first. Strings, raw strings, regular expressions,
heredocs, YAML block scalars and similar constructs are protected, so their
contents are copied verbatim. Formats where a single line would change the
meaning (Python, Markdown, indentation-based Sass, and others) are skipped on
purpose.

Output is a single line by default where the syntax allows it. Use
`--preserve-line-breaks` to keep the original line structure. Line based
formats such as YAML, shell scripts and Makefiles always keep their lines.

### `--help`

```text
comment and whitespace minifier for Rust, web, Android, JVM, data, build and scripting text files;
fast parallel CLI.

Usage: rust_minifier.exe [OPTIONS] [FILES]...

Arguments:
  [FILES]...
          Input files or directories to minify.

          Can be individual supported text files, or directories to recursively search. If omitted,
          reads from stdin.

Options:
  -t, --type <LANGUAGE>
          Force a language for every input instead of detecting it.

          Accepts a language id or alias from `--list-types`, for example `html`, `typescript`,
          `kotlin` or `yaml`.

      --preserve-line-breaks
          Keep original line breaks where the format allows it.

          By default output is collapsed to a single line. Line based formats already keep their
          line structure.

      --list-types
          List every supported and excluded file type, then exit

  -s, --suffix <SUFFIX>
          Suffix to append to minified output filenames.

          For example, with suffix `_minified`, `main.rs` becomes `main_minified.rs`. Only used when
          writing individual files (not with `--concat-to`).

          [default: _minified]

      --concat-to <DESTINATION>
          Destination for concatenated output.

          Can be A file path (writes minified content to that file), a directory path (creates a
          timestamped file in that directory), `STDOUT` (or empty - prints to standard output,
          default).

          When specified, individual minified files are not written to disk.

          [default: STDOUT]

      --include-file-path
          Include source file paths as comments in output.

          When enabled, each minified file is prefixed with a comment containing its full path.
          Useful for tracking which source files contributed to concatenated output.

      --no-include-file-path
          Do not prefix output with the source file path.

          This is the inverse of `--include-file-path` and wins when both are set.

      --exclude <EXCLUDE>
          Exclude files matching these patterns (non-case-sensitive substring match on filename).

          Files whose names contain any of these patterns are skipped during processing. Matching is
          case-insensitive and applied only to the filename, not the full path.

          for example: rust_minifier src/ --exclude test --exclude mock will exclude files like:
          `test_helper.rs`, `mock_data.toml`, `my_test.rs`

      --soft-split <SOFT_SPLIT>
          Try to limit maximum number of characters per file in --concat-to result, by generating
          multiple files each with a '_part_XX_of_YY' suffix

          [default: unlimited]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

Repository:    https://github.com/eladkarako/rust_minifier
Homepage:      https://github.com/eladkarako/rust_minifier
Use --list-types to see every supported file type.
```

### supported file types

The authoritative list is always available from the program:

```text
rust_minifier --list-types
```

It prints every supported type grouped by category, followed by every excluded
type and the reason it is skipped. The table below is a short overview; the
program output also lists every extension and special file name.

| Category | Supported types |
| --- | --- |
| Rust | Rust (`.rs`) |
| Web | HTML, SVG, Vue, Svelte, Astro, Razor/CSHTML, JSP, Twig, Handlebars/Mustache, EJS/ERB/Liquid/Nunjucks, JavaScript, JSX, TypeScript, TSX, PHP |
| Android | AIDL, ProGuard/R8 rules, Smali |
| JVM | Java, Kotlin, Groovy/Gradle, Scala |
| Systems | C, C++, C#, Objective-C, Go, Swift, Dart, Zig, D, Nim, Crystal, Solidity, CUDA, Assembly, Verilog/SystemVerilog, VHDL |
| Scripting | Shell, PowerShell, Windows batch, Lua, Perl, Ruby, R, Julia, Elixir, Erlang, Clojure, Haskell, OCaml, Pascal/Delphi, Visual Basic, Fortran, Tcl, AWK, Vim script |
| Data | Prisma, JSON/JSONC/JSON5, YAML, TOML, Protocol Buffers, Thrift, FlatBuffers, Avro IDL, Gettext PO, WebAssembly text |
| Config | Java properties/`.env`, INI/config, ignore files, Browserslist, Procfile |
| Build | Dockerfile/Containerfile, Makefile, CMake, HCL/Terraform, Nix, Just, Ninja |
| Markup | XML (including Android resources, project files and XAML) |
| Stylesheet | CSS, SCSS, LESS |
| Query | SQL, GraphQL |
| Shader | GLSL/HLSL/WGSL/Metal, OpenCL/ISPC |
| Other | Apple `.strings`, QML |

<details><summary><h4>full <code>--list-types</code> output</h4></summary>

```text
Supported types (90):

[rust]
  rust             Rust                         .rs

[web]
  html             HTML                         .html, .htm, .shtml, .shtm, .xhtml, .xht, .xhtm, .hta
  svg              SVG                          .svg
  vue              Vue single file component    .vue, .nvue
  svelte           Svelte component             .svelte
  astro            Astro component              .astro
  razor            Razor / CSHTML               .razor, .cshtml
  jsp              JSP                          .jsp, .jspx, .jspf
  twig             Twig template                .twig
  handlebars       Handlebars / Mustache        .hbs, .handlebars, .mustache
  ejs              EJS                          .ejs, .erb, .liquid, .njk
  javascript       JavaScript                   .js, .mjs, .cjs
  jsx              JavaScript JSX               .jsx
  typescript       TypeScript                   .ts, .mts, .cts
  tsx              TypeScript TSX               .tsx
  php              PHP                          .php, .phtml, .php3, .php4, .php5, .php7, .phps

[android]
  aidl             Android AIDL                 .aidl
  proguard         ProGuard / R8 rules          .pro, .proguard
  smali            Smali                        .smali

[jvm]
  java             Java                         .java
  kotlin           Kotlin                       .kt, .kts
  groovy           Groovy / Gradle DSL          .groovy, .gradle, .gvy, .gy, .gsh, Jenkinsfile
  scala            Scala                        .scala, .sc

[systems]
  c                C                            .c, .h
  cpp              C++                          .cpp, .cxx, .cc, .c++, .hpp, .hxx, .hh, .h++, .ipp, .inl, .tpp, .txx
  csharp           C#                           .cs, .csx
  objectivec       Objective-C                  .m, .mm
  go               Go                           .go
  swift            Swift                        .swift
  dart             Dart / Flutter               .dart
  zig              Zig                          .zig, .zon
  d                D                            .d, .di
  nim              Nim                          .nim, .nims
  crystal          Crystal                      .cr
  solidity         Solidity                     .sol
  cuda             CUDA                         .cu, .cuh, .hip
  assembly         Assembly                     .asm, .s, .S
  verilog          Verilog / SystemVerilog      .v, .sv, .svh, .vh
  vhdl             VHDL                         .vhd, .vhdl

[scripting]
  shell            Shell                        .sh, .bash, .zsh, .fish, .ksh, .bashrc, .bash_profile, .zshrc, .profile, .bash_aliases
  powershell       PowerShell                   .ps1, .psm1, .psd1
  batch            Windows batch                .bat, .cmd
  lua              Lua                          .lua
  perl             Perl                         .pl, .pm, .t, .pod
  ruby             Ruby                         .rb, .rake, .gemspec, .ru, Rakefile, Gemfile, Podfile, Fastfile, Appfile, Matchfile, Scanfile, Gymfile, Snapfile, Deliverfile, Pluginfile, Dangerfile, Steepfile, Guardfile, Capfile, Thorfile, Berksfile, Cheffile, Bowerfile, Caskfile, Puppetfile, Vagrantfile, Brewfile
  r                R                            .r, .R, .rmd
  julia            Julia                        .jl
  elixir           Elixir                       .ex, .exs, mix.exs
  erlang           Erlang                       .erl, .hrl
  clojure          Clojure                      .clj, .cljs, .cljc, .edn
  haskell          Haskell                      .hs, .lhs
  ocaml            OCaml                        .ml, .mli
  pascal           Pascal / Delphi              .pas, .pp, .dpr, .dpk, .lpr
  visualbasic      Visual Basic                 .vb, .vbs, .bas, .cls, .frm
  fortran          Fortran                      .f, .f77, .f90, .f95, .f03, .f08, .for, .ftn
  tcl              Tcl                          .tcl, .tk, .itcl
  awk              AWK                          .awk, .gawk, .mawk
  vim              Vim script                   .vim, .vimrc, .gvimrc, .exrc, .ideavimrc

[data]
  prisma           Prisma schema                .prisma
  json             JSON / JSONC / JSON5         .json, .jsonc, .json5, .webmanifest, .map, .har, .geojson, .topojson, .ipynb, .avsc, .jsonld, .sarif, .babelrc, .eslintrc, .prettierrc, .stylelintrc, .swcrc, .commitlintrc, .lintstagedrc, .nycrc
  yaml             YAML                         .yaml, .yml
  toml             TOML                         .toml, Cargo.lock, poetry.lock, Pipfile
  proto            Protocol Buffers             .proto
  thrift           Thrift                       .thrift
  flatbuffers      FlatBuffers schema           .fbs
  avro             Avro IDL                     .avdl
  po               Gettext PO                   .po, .pot
  wat              WebAssembly text             .wat, .wast

[config]
  properties       Java properties / .env       .properties, .env, .env.local, .env.development, .env.production
  ini              INI / config                 .ini, .cfg, .conf, .cnf, .desktop, .service, .timer, .socket, .mount, .path, .target, .slice, .scope, .automount, .swap, .device, .network, .netdev, .link, .dnssd, .nspawn, .editorconfig, .gitconfig, .gitmodules, .npmrc, .yarnrc, .pylintrc, .flake8, .coveragerc, .mypy.ini, .pypirc, .hgrc
  gitignore        Ignore files                 .gitignore, .dockerignore, .npmignore, .eslintignore, .prettierignore, .stylelintignore, .vscodeignore, .hgignore, .cvsignore, .slugignore, .cfignore, .helmignore, .gcloudignore, .vercelignore, .netlifyignore, .nowignore, .firebaseignore, .amplifyignore, .serverlessignore, .terraformignore, .pulumiignore, .cdkignore, .sstignore, .wranglerignore, .denoignore, .rgignore, .fdignore, .agignore, .ignore
  browserslist     Browserslist                 .browserslistrc
  procfile         Procfile                     Procfile

[build]
  dockerfile       Dockerfile                   .dockerfile, .containerfile, Dockerfile, Containerfile
  makefile         Makefile                     .mk, .make, Makefile, makefile, GNUmakefile
  cmake            CMake                        .cmake, CMakeLists.txt
  hcl              HCL / Terraform              .tf, .tfvars, .hcl, .nomad
  nix              Nix                          .nix
  just             Just                         .just, justfile, Justfile, .justfile
  ninja            Ninja                        .ninja, build.ninja

[markup]
  xml              XML                          .xml, .xsd, .xsl, .xslt, .wsdl, .rss, .atom, .plist, .storyboard, .xib, .resx, .csproj, .vbproj, .fsproj, .props, .targets, .nuspec, .config, .manifest, .xaml, .axaml, .ui, .glade, .xul, .rdf, .owl, .bpmn, .dmn, .wxs, .wxi, .wxl, .settings, .kml, .gpx, .gml, .tcx, .opml, .proj, .projitems, .shproj, .vcxproj, .vcproj, .xproj, .dtsx, .wsf

[stylesheet]
  css              CSS                          .css, .pcss, .postcss, .wxss, .acss, .qss
  scss             SCSS                         .scss
  less             LESS                         .less

[query]
  sql              SQL                          .sql, .ddl, .dml, .psql, .pgsql, .mysql, .sqlite
  graphql          GraphQL                      .graphql, .gql, .graphqls

[shader]
  glsl             GLSL                         .glsl, .vert, .frag, .geom, .tesc, .tese, .comp, .hlsl, .wgsl, .metal
  opencl           OpenCL / ISPC                .cl, .ispc

[other]
  strings          Apple .strings               .strings
  qml              QML                          .qml
```

</details>

### excluded file types

These are detected but intentionally not minified. The program skips them and
`--list-types` prints the same reasons.

| Excluded | Examples | Why |
| --- | --- | --- |
| Python | `.py`, `.pyw`, `.pyi`, `.pyx`, `.pxd` | indentation is syntax; single-line output would change meaning |
| Markdown | `.md`, `.markdown`, `.mdx`, `.mdown`, `.mkd` | line structure and spacing carry the formatting |
| reStructuredText / AsciiDoc | `.rst`, `.adoc`, `.asciidoc`, `.org` | line structure and indentation carry the formatting |
| Indented Sass / Stylus | `.sass`, `.styl`, `.sss` | indentation is syntax; use `.scss` or `.css` instead |
| Pug / Haml / Slim | `.pug`, `.jade`, `.haml`, `.slim` | indentation is syntax |
| F# / Elm / PureScript | `.fs`, `.fsi`, `.fsx`, `.elm`, `.purs` | indentation is syntax |
| Starlark / Bazel / Meson | `.bzl`, `.bazel`, `.star` | Python-like indentation is syntax |
| LaTeX / TeX | `.tex`, `.latex`, `.ltx`, `.sty`, `.cls` | whitespace and line breaks are semantically significant |
| CSV / TSV / line-delimited JSON | `.csv`, `.tsv`, `.ndjson`, `.jsonl` | record boundaries and whitespace are data |
| Diff / patch / logs | `.diff`, `.patch`, `.log` | line structure is the content |
| Binary and archive formats | `.png`, `.zip`, `.pdf`, `.apk`, `.exe`, `.wasm`, ... | binary content is out of scope |

### language overrides and line breaks

- Detection uses the file extension, exact file name and, for a few ambiguous
  names (`.config`, `.m`, `.inc`, `.lock`), the file contents.
- `--type <LANGUAGE>` or `-t <LANGUAGE>` forces one language for every input.
  This is how you minify stdin or a file with an unusual extension:
  `type page.txt | rust_minifier -t html`.
- Without `--type`, stdin is treated as Rust.
- `--preserve-line-breaks` keeps newlines for engines that can safely do so.
  Line based formats (YAML, shell, INI, Makefile, ...) already keep their lines.
- `--no-include-file-path` removes the path comment that is normally placed
  before each minified file.
- `--list-types` prints the full supported and excluded lists, including every
  extension and special file name.

### uses
- drag and drop any supported file over the executable as an argument to generate a minified file.
- run `rust_minifier . --concat-to all.txt` to recursively walk all files and folders (in parallel - highly advised! very fast!) and join everything after minification to `all.txt` in the current folder.
- use `--type` to minify stdin: `type index.html | rust_minifier -t html > index.min.html`.
- use it in your own batch files and first recursively collect the paths to all supported files in your project, before executing this program ONCE with all those as arguments; it works in parallel. Then walk through to join all the files with `_minified` in them; they already include their fully qualified path as the first line (by default), so you can easily create one file that documents your project's structure.
- if your A.I. code agent is chat-based, you can split a long output into multiple files based on a maximum character length limit using `--soft-split`; it is only effective when used with `--concat-to` to a file.
- use `--exclude test --exclude mock` to keep test and mock files out of the result.

### use cases
- pass your code to an A.I. code generator for code review more easily.
- strip away all your old comments and rustdoc before creating new ones.
- prepare a project for `rustfmt.toml` and `cargo fmt --verbose --message-format human` when you truly want to clean up old styles.
- bundle an entire small web app (HTML, CSS, JS, JSON, SVG) into a single file for inspection or embedding.
- strip comments from Android resources and Gradle files before packaging or review.

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
