//! Data-driven registry of every file type this program knows about.
//!
//! The registry is intentionally a plain table of [`Language`] records. Adding
//! a new file type should usually be a one-line change here, not a new engine.
//! Each record names the engine that knows how to minify the syntax and the
//! comment style used when a source path is embedded in concatenated output.

use std::path::Path;

/// The family of minification logic used for a language.
///
/// Several languages share an engine. For example JavaScript, Kotlin and Java
/// all use [`EngineKind::CStyle`] with different profiles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EngineKind {
    /// Existing token based Rust minifier.
    Rust,
    /// Existing line oriented TOML minifier.
    Toml,
    /// HTML, XML, SVG and single file component formats.
    Markup,
    /// C-like languages with configurable comment and string syntax.
    CStyle,
    /// CSS, SCSS and LESS.
    Css,
    /// JSON and JSON-with-comments dialects.
    Json,
    /// SQL dialects.
    Sql,
    /// GraphQL.
    GraphQl,
    /// Formats whose line structure must be preserved (YAML, shell, INI...).
    LineBased,
    /// Mixed HTML/PHP documents.
    Php,
}

impl EngineKind {
    /// Short human readable engine name used by `--list-types`.
    pub fn name(self) -> &'static str {
        match self {
            EngineKind::Rust => "rust-token",
            EngineKind::Toml => "toml-lines",
            EngineKind::Markup => "markup",
            EngineKind::CStyle => "c-like",
            EngineKind::Css => "css",
            EngineKind::Json => "json",
            EngineKind::Sql => "sql",
            EngineKind::GraphQl => "graphql",
            EngineKind::LineBased => "line-based",
            EngineKind::Php => "php-mixed",
        }
    }
}

/// How a source path comment is rendered for a language.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommentStyle {
    /// A single line comment, for example `// path` or `# path`.
    Line(&'static str),
    /// A block comment, for example `<!-- path -->`.
    Block(&'static str, &'static str),
    /// The format has no safe comment syntax.
    None,
}

impl CommentStyle {
    /// Render a path comment. Returns `None` when the format cannot carry one.
    pub fn render(self, text: &str) -> Option<String> {
        match self {
            CommentStyle::Line(prefix) => {
                Some(format!("{} {}", prefix, text))
            }
            CommentStyle::Block(open, close) => {
                Some(format!("{} {} {}", open, text, close))
            }
            CommentStyle::None => None,
        }
    }
}

/// Broad grouping used for documentation and deterministic output ordering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Category {
    Rust,
    Web,
    Android,
    Jvm,
    Systems,
    Scripting,
    Data,
    Config,
    Build,
    Markup,
    Stylesheet,
    Query,
    Shader,
    Other,
}

impl Category {
    pub fn name(self) -> &'static str {
        match self {
            Category::Rust => "rust",
            Category::Web => "web",
            Category::Android => "android",
            Category::Jvm => "jvm",
            Category::Systems => "systems",
            Category::Scripting => "scripting",
            Category::Data => "data",
            Category::Config => "config",
            Category::Build => "build",
            Category::Markup => "markup",
            Category::Stylesheet => "stylesheet",
            Category::Query => "query",
            Category::Shader => "shader",
            Category::Other => "other",
        }
    }
}

/// One supported file type.
#[derive(Debug, Clone, Copy)]
pub struct Language {
    /// Canonical identifier accepted by `--type`.
    pub id: &'static str,
    /// Human readable name shown by `--list-types`.
    pub display_name: &'static str,
    /// Extensions without the leading dot, lower case.
    pub extensions: &'static [&'static str],
    /// Exact file names (case insensitive) such as `Dockerfile`.
    pub filenames: &'static [&'static str],
    /// File name prefixes such as `Dockerfile.` for `Dockerfile.dev`.
    pub filename_prefixes: &'static [&'static str],
    /// Engine used to minify this language.
    pub engine: EngineKind,
    /// Comment syntax used for embedded source paths.
    pub comment: CommentStyle,
    /// Documentation grouping.
    pub category: Category,
    /// Whether the engine is expected to emit a single line.
    pub single_line: bool,
    /// Extra identifiers accepted by `--type`.
    pub aliases: &'static [&'static str],
    /// Short caveat shown in documentation.
    pub notes: &'static str,
}

/// A file type that is deliberately not minified.
#[derive(Debug, Clone, Copy)]
pub struct ExcludedType {
    pub name: &'static str,
    pub extensions: &'static [&'static str],
    pub reason: &'static str,
}

const fn language(
    id: &'static str,
    display_name: &'static str,
    engine: EngineKind,
    comment: CommentStyle,
    category: Category,
    single_line: bool,
    extensions: &'static [&'static str],
    filenames: &'static [&'static str],
    filename_prefixes: &'static [&'static str],
    aliases: &'static [&'static str],
    notes: &'static str,
) -> Language {
    Language {
        id,
        display_name,
        extensions,
        filenames,
        filename_prefixes,
        engine,
        comment,
        category,
        single_line,
        aliases,
        notes,
    }
}

/// Every supported language. Detection walks this table in order, so put more
/// specific records before more general ones.
pub static LANGUAGES: &[Language] = &[
    // ---------------------------------------------------------------- Rust
    language(
        "rust",
        "Rust",
        EngineKind::Rust,
        CommentStyle::Line("//"),
        Category::Rust,
        true,
        &["rs"],
        &[],
        &[],
        &["rs"],
        "token based; comments and rustdoc removed",
    ),
    // ---------------------------------------------------------------- Web
    language(
        "html",
        "HTML",
        EngineKind::Markup,
        CommentStyle::Block("<!--", "-->"),
        Category::Web,
        true,
        &["html", "htm", "shtml", "shtm", "xhtml", "xht", "xhtm", "hta"],
        &[],
        &[],
        &["htm"],
        "comments removed; pre/textarea preserved; script/style minified",
    ),
    language(
        "xml",
        "XML",
        EngineKind::Markup,
        CommentStyle::Block("<!--", "-->"),
        Category::Markup,
        true,
        &[
            "xml", "xsd", "xsl", "xslt", "wsdl", "rss", "atom", "plist",
            "storyboard", "xib", "resx", "csproj", "vbproj", "fsproj",
            "props", "targets", "nuspec", "config", "manifest", "xaml",
            "axaml", "ui", "glade", "xul", "rdf", "owl", "bpmn", "dmn",
            "wxs", "wxi", "wxl", "settings", "kml", "gpx", "gml", "tcx",
            "opml", "proj", "projitems", "shproj", "vcxproj", "vcproj",
            "xproj", "dtsx", "wsf",
        ],
        &[],
        &[],
        &["xsd", "svg"],
        "comments removed; whitespace between tags collapsed",
    ),
    language(
        "svg",
        "SVG",
        EngineKind::Markup,
        CommentStyle::Block("<!--", "-->"),
        Category::Web,
        true,
        &["svg"],
        &[],
        &[],
        &[],
        "XML markup with embedded style/script minification",
    ),
    language(
        "vue",
        "Vue single file component",
        EngineKind::Markup,
        CommentStyle::Block("<!--", "-->"),
        Category::Web,
        true,
        &["vue", "nvue"],
        &[],
        &[],
        &["vue-sfc"],
        "template minified; script/style blocks minified by language",
    ),
    language(
        "svelte",
        "Svelte component",
        EngineKind::Markup,
        CommentStyle::Block("<!--", "-->"),
        Category::Web,
        true,
        &["svelte"],
        &[],
        &[],
        &[],
        "template minified; script/style blocks minified",
    ),
    language(
        "astro",
        "Astro component",
        EngineKind::Markup,
        CommentStyle::Block("<!--", "-->"),
        Category::Web,
        true,
        &["astro"],
        &[],
        &[],
        &[],
        "frontmatter preserved as TypeScript; template minified",
    ),
    language(
        "razor",
        "Razor / CSHTML",
        EngineKind::Markup,
        CommentStyle::Block("<!--", "-->"),
        Category::Web,
        true,
        &["razor", "cshtml"],
        &[],
        &[],
        &["cshtml"],
        "HTML minified; C# blocks left intact",
    ),
    language(
        "jsp",
        "JSP",
        EngineKind::Markup,
        CommentStyle::Block("<!--", "-->"),
        Category::Web,
        true,
        &["jsp", "jspx", "jspf"],
        &[],
        &[],
        &[],
        "HTML minified; Java scriptlets left intact",
    ),
    language(
        "twig",
        "Twig template",
        EngineKind::Markup,
        CommentStyle::Block("<!--", "-->"),
        Category::Web,
        true,
        &["twig"],
        &[],
        &[],
        &[],
        "template tags preserved; HTML whitespace collapsed",
    ),
    language(
        "handlebars",
        "Handlebars / Mustache",
        EngineKind::Markup,
        CommentStyle::Block("<!--", "-->"),
        Category::Web,
        true,
        &["hbs", "handlebars", "mustache"],
        &[],
        &[],
        &["mustache"],
        "template tags preserved; HTML whitespace collapsed",
    ),
    language(
        "ejs",
        "EJS",
        EngineKind::Markup,
        CommentStyle::Block("<!--", "-->"),
        Category::Web,
        true,
        &["ejs", "erb", "liquid", "njk"],
        &[],
        &[],
        &["erb", "liquid", "nunjucks"],
        "template tags preserved; HTML whitespace collapsed",
    ),
    language(
        "javascript",
        "JavaScript",
        EngineKind::CStyle,
        CommentStyle::Line("//"),
        Category::Web,
        true,
        &["js", "mjs", "cjs"],
        &[],
        &[],
        &["js", "ecmascript"],
        "comments removed; ASI safe semicolons inserted; template literals kept",
    ),
    language(
        "jsx",
        "JavaScript JSX",
        EngineKind::CStyle,
        CommentStyle::Line("//"),
        Category::Web,
        true,
        &["jsx"],
        &[],
        &[],
        &["react"],
        "comments removed; JSX comments become empty expression containers",
    ),
    language(
        "typescript",
        "TypeScript",
        EngineKind::CStyle,
        CommentStyle::Line("//"),
        Category::Web,
        true,
        &["ts", "mts", "cts"],
        &[],
        &[],
        &["ts"],
        "comments removed; ASI safe semicolons inserted",
    ),
    language(
        "tsx",
        "TypeScript TSX",
        EngineKind::CStyle,
        CommentStyle::Line("//"),
        Category::Web,
        true,
        &["tsx"],
        &[],
        &[],
        &[],
        "comments removed; JSX and TypeScript syntax supported",
    ),
    language(
        "css",
        "CSS",
        EngineKind::Css,
        CommentStyle::Block("/*", "*/"),
        Category::Stylesheet,
        true,
        &["css", "pcss", "postcss", "wxss", "acss", "qss"],
        &[],
        &[],
        &[],
        "comments removed; selector spaces preserved; url() kept intact",
    ),
    language(
        "scss",
        "SCSS",
        EngineKind::Css,
        CommentStyle::Block("/*", "*/"),
        Category::Stylesheet,
        true,
        &["scss"],
        &[],
        &[],
        &["sass-scss"],
        "SCSS nesting supported; // and /* */ comments removed",
    ),
    language(
        "less",
        "LESS",
        EngineKind::Css,
        CommentStyle::Block("/*", "*/"),
        Category::Stylesheet,
        true,
        &["less"],
        &[],
        &[],
        &[],
        "LESS nesting supported; // and /* */ comments removed",
    ),
    language(
        "php",
        "PHP",
        EngineKind::Php,
        CommentStyle::Line("//"),
        Category::Web,
        true,
        &["php", "phtml", "php3", "php4", "php5", "php7", "phps"],
        &[],
        &[],
        &[],
        "HTML outside PHP tags and PHP code inside tags are minified separately",
    ),
    // ---------------------------------------------------------------- Android / JVM
    language(
        "java",
        "Java",
        EngineKind::CStyle,
        CommentStyle::Line("//"),
        Category::Jvm,
        true,
        &["java"],
        &[],
        &[],
        &[],
        "comments removed; text blocks preserved",
    ),
    language(
        "kotlin",
        "Kotlin",
        EngineKind::CStyle,
        CommentStyle::Line("//"),
        Category::Jvm,
        true,
        &["kt", "kts"],
        &[],
        &[],
        &["kt"],
        "comments removed; newline statements get safe semicolons",
    ),
    language(
        "groovy",
        "Groovy / Gradle DSL",
        EngineKind::CStyle,
        CommentStyle::Line("//"),
        Category::Jvm,
        true,
        &["groovy", "gradle", "gvy", "gy", "gsh"],
        &["Jenkinsfile"],
        &[],
        &["gradle-groovy"],
        "Gradle Groovy DSL supported; comments removed",
    ),
    language(
        "scala",
        "Scala",
        EngineKind::CStyle,
        CommentStyle::Line("//"),
        Category::Jvm,
        true,
        &["scala", "sc"],
        &[],
        &[],
        &[],
        "comments removed; nested block comments supported",
    ),
    language(
        "aidl",
        "Android AIDL",
        EngineKind::CStyle,
        CommentStyle::Line("//"),
        Category::Android,
        true,
        &["aidl"],
        &[],
        &[],
        &[],
        "Android interface definition language",
    ),
    language(
        "proguard",
        "ProGuard / R8 rules",
        EngineKind::LineBased,
        CommentStyle::Line("#"),
        Category::Android,
        false,
        &["pro", "proguard"],
        &[],
        &[],
        &["r8"],
        "line preserving; # comments removed",
    ),
    language(
        "smali",
        "Smali",
        EngineKind::LineBased,
        CommentStyle::Line("#"),
        Category::Android,
        false,
        &["smali"],
        &[],
        &[],
        &[],
        "line preserving; # comments removed",
    ),
    language(
        "properties",
        "Java properties / .env",
        EngineKind::LineBased,
        CommentStyle::Line("#"),
        Category::Config,
        false,
        &["properties", "env"],
        &[".env", ".env.local", ".env.development", ".env.production"],
        &[".env."],
        &["dotenv"],
        "line preserving; # and ! comments removed; backslash continuations joined",
    ),
    // ---------------------------------------------------------------- Systems
    language(
        "c",
        "C",
        EngineKind::CStyle,
        CommentStyle::Line("//"),
        Category::Systems,
        true,
        &["c", "h"],
        &[],
        &[],
        &[],
        "preprocessor lines keep their own line",
    ),
    language(
        "cpp",
        "C++",
        EngineKind::CStyle,
        CommentStyle::Line("//"),
        Category::Systems,
        true,
        &[
            "cpp", "cxx", "cc", "c++", "hpp", "hxx", "hh", "h++", "ipp",
            "inl", "tpp", "txx",
        ],
        &[],
        &[],
        &["c++"],
        "preprocessor lines keep their own line",
    ),
    language(
        "csharp",
        "C#",
        EngineKind::CStyle,
        CommentStyle::Line("//"),
        Category::Systems,
        true,
        &["cs", "csx"],
        &[],
        &[],
        &["c#"],
        "verbatim/interpolated strings supported; # directives keep their line",
    ),
    language(
        "objectivec",
        "Objective-C",
        EngineKind::CStyle,
        CommentStyle::Line("//"),
        Category::Systems,
        true,
        &["m", "mm"],
        &[],
        &[],
        &["objc"],
        "preprocessor lines keep their own line",
    ),
    language(
        "go",
        "Go",
        EngineKind::CStyle,
        CommentStyle::Line("//"),
        Category::Systems,
        true,
        &["go"],
        &[],
        &[],
        &["golang"],
        "raw backtick strings supported; newline statements get semicolons",
    ),
    language(
        "swift",
        "Swift",
        EngineKind::CStyle,
        CommentStyle::Line("//"),
        Category::Systems,
        true,
        &["swift"],
        &[],
        &[],
        &[],
        "multiline strings and nested comments supported",
    ),
    language(
        "dart",
        "Dart / Flutter",
        EngineKind::CStyle,
        CommentStyle::Line("//"),
        Category::Systems,
        true,
        &["dart"],
        &[],
        &[],
        &[],
        "raw and multiline strings supported",
    ),
    language(
        "zig",
        "Zig",
        EngineKind::CStyle,
        CommentStyle::Line("//"),
        Category::Systems,
        true,
        &["zig", "zon"],
        &[],
        &[],
        &[],
        "line comments only",
    ),
    language(
        "d",
        "D",
        EngineKind::CStyle,
        CommentStyle::Line("//"),
        Category::Systems,
        true,
        &["d", "di"],
        &[],
        &[],
        &["dlang"],
        "nested block comments supported",
    ),
    language(
        "nim",
        "Nim",
        EngineKind::LineBased,
        CommentStyle::Line("#"),
        Category::Systems,
        false,
        &["nim", "nims"],
        &[],
        &[],
        &[],
        "indentation sensitive; line preserving",
    ),
    language(
        "crystal",
        "Crystal",
        EngineKind::CStyle,
        CommentStyle::Line("#"),
        Category::Systems,
        true,
        &["cr"],
        &[],
        &[],
        &[],
        "Ruby-like syntax; safe semicolons inserted",
    ),
    language(
        "solidity",
        "Solidity",
        EngineKind::CStyle,
        CommentStyle::Line("//"),
        Category::Systems,
        true,
        &["sol"],
        &[],
        &[],
        &[],
        "comments removed",
    ),
    language(
        "cuda",
        "CUDA",
        EngineKind::CStyle,
        CommentStyle::Line("//"),
        Category::Systems,
        true,
        &["cu", "cuh", "hip"],
        &[],
        &[],
        &["hip"],
        "preprocessor lines keep their own line",
    ),
    language(
        "assembly",
        "Assembly",
        EngineKind::LineBased,
        CommentStyle::Line(";"),
        Category::Systems,
        false,
        &["asm", "s", "S"],
        &[],
        &[],
        &["asm"],
        "line preserving; ; and # comments removed",
    ),
    language(
        "verilog",
        "Verilog / SystemVerilog",
        EngineKind::CStyle,
        CommentStyle::Line("//"),
        Category::Systems,
        true,
        &["v", "sv", "svh", "vh"],
        &[],
        &[],
        &["systemverilog"],
        "comments removed; statements keep their semicolons",
    ),
    language(
        "vhdl",
        "VHDL",
        EngineKind::CStyle,
        CommentStyle::Line("--"),
        Category::Systems,
        true,
        &["vhd", "vhdl"],
        &[],
        &[],
        &[],
        "comments removed; statements keep their semicolons",
    ),
    // ---------------------------------------------------------------- Scripting
    language(
        "shell",
        "Shell",
        EngineKind::LineBased,
        CommentStyle::Line("#"),
        Category::Scripting,
        false,
        &["sh", "bash", "zsh", "fish", "ksh"],
        &[".bashrc", ".bash_profile", ".zshrc", ".profile", ".bash_aliases"],
        &[],
        &["bash", "sh"],
        "line preserving; heredocs preserved; # comments removed",
    ),
    language(
        "powershell",
        "PowerShell",
        EngineKind::LineBased,
        CommentStyle::Line("#"),
        Category::Scripting,
        false,
        &["ps1", "psm1", "psd1"],
        &[],
        &[],
        &["pwsh"],
        "line preserving; # and <# #> comments removed",
    ),
    language(
        "batch",
        "Windows batch",
        EngineKind::LineBased,
        CommentStyle::Line("REM"),
        Category::Scripting,
        false,
        &["bat", "cmd"],
        &[],
        &[],
        &["dos"],
        "line preserving; REM and :: comments removed",
    ),
    language(
        "lua",
        "Lua",
        EngineKind::CStyle,
        CommentStyle::Line("--"),
        Category::Scripting,
        true,
        &["lua"],
        &[],
        &[],
        &[],
        "long bracket strings and comments supported",
    ),
    language(
        "perl",
        "Perl",
        EngineKind::LineBased,
        CommentStyle::Line("#"),
        Category::Scripting,
        false,
        &["pl", "pm", "t", "pod"],
        &[],
        &[],
        &[],
        "line preserving; POD blocks left intact",
    ),
    language(
        "ruby",
        "Ruby",
        EngineKind::LineBased,
        CommentStyle::Line("#"),
        Category::Scripting,
        false,
        &["rb", "rake", "gemspec", "ru"],
        &[
            "Rakefile", "Gemfile", "Podfile", "Fastfile", "Appfile",
            "Matchfile", "Scanfile", "Gymfile", "Snapfile", "Deliverfile",
            "Pluginfile", "Dangerfile", "Steepfile", "Guardfile", "Capfile",
            "Thorfile", "Berksfile", "Cheffile", "Bowerfile", "Caskfile",
            "Puppetfile", "Vagrantfile", "Brewfile",
        ],
        &[],
        &["rb"],
        "line preserving; =begin/=end and # comments removed",
    ),
    language(
        "r",
        "R",
        EngineKind::LineBased,
        CommentStyle::Line("#"),
        Category::Scripting,
        false,
        &["r", "R", "rmd"],
        &[],
        &[],
        &["rstats"],
        "line preserving; # comments removed",
    ),
    language(
        "julia",
        "Julia",
        EngineKind::LineBased,
        CommentStyle::Line("#"),
        Category::Scripting,
        false,
        &["jl"],
        &[],
        &[],
        &[],
        "line preserving; # and #= =# comments removed",
    ),
    language(
        "elixir",
        "Elixir",
        EngineKind::CStyle,
        CommentStyle::Line("#"),
        Category::Scripting,
        true,
        &["ex", "exs"],
        &["mix.exs"],
        &[],
        &[],
        "comments removed; heredocs preserved",
    ),
    language(
        "erlang",
        "Erlang",
        EngineKind::CStyle,
        CommentStyle::Line("%"),
        Category::Scripting,
        true,
        &["erl", "hrl"],
        &[],
        &[],
        &[],
        "comments removed",
    ),
    language(
        "clojure",
        "Clojure",
        EngineKind::CStyle,
        CommentStyle::Line(";"),
        Category::Scripting,
        true,
        &["clj", "cljs", "cljc", "edn"],
        &[],
        &[],
        &[],
        "comments removed",
    ),
    language(
        "haskell",
        "Haskell",
        EngineKind::LineBased,
        CommentStyle::Line("--"),
        Category::Scripting,
        false,
        &["hs", "lhs"],
        &[],
        &[],
        &[],
        "layout sensitive; line preserving",
    ),
    language(
        "ocaml",
        "OCaml",
        EngineKind::CStyle,
        CommentStyle::Block("(*", "*)"),
        Category::Scripting,
        true,
        &["ml", "mli"],
        &[],
        &[],
        &[],
        "nested block comments removed",
    ),
    language(
        "pascal",
        "Pascal / Delphi",
        EngineKind::CStyle,
        CommentStyle::Line("//"),
        Category::Scripting,
        true,
        &["pas", "pp", "dpr", "dpk", "lpr"],
        &[],
        &[],
        &[],
        "//, { } and (* *) comments removed",
    ),
    language(
        "visualbasic",
        "Visual Basic",
        EngineKind::LineBased,
        CommentStyle::Line("'"),
        Category::Scripting,
        false,
        &["vb", "vbs", "bas", "cls", "frm"],
        &[],
        &[],
        &["vbnet"],
        "line preserving; ' and REM comments removed",
    ),
    language(
        "fortran",
        "Fortran",
        EngineKind::LineBased,
        CommentStyle::Line("!"),
        Category::Scripting,
        false,
        &["f", "f77", "f90", "f95", "f03", "f08", "for", "ftn"],
        &[],
        &[],
        &[],
        "line preserving; ! comments removed",
    ),
    language(
        "tcl",
        "Tcl",
        EngineKind::LineBased,
        CommentStyle::Line("#"),
        Category::Scripting,
        false,
        &["tcl", "tk", "itcl"],
        &[],
        &[],
        &[],
        "line preserving; # comments removed",
    ),
    language(
        "awk",
        "AWK",
        EngineKind::LineBased,
        CommentStyle::Line("#"),
        Category::Scripting,
        false,
        &["awk", "gawk", "mawk"],
        &[],
        &[],
        &[],
        "line preserving; # comments removed",
    ),
    language(
        "vim",
        "Vim script",
        EngineKind::LineBased,
        CommentStyle::Line("\""),
        Category::Scripting,
        false,
        &["vim"],
        &[".vimrc", ".gvimrc", ".exrc", ".ideavimrc"],
        &[],
        &["vimscript"],
        "line preserving; \" comments removed",
    ),
    language(
        "prisma",
        "Prisma schema",
        EngineKind::LineBased,
        CommentStyle::Line("//"),
        Category::Data,
        false,
        &["prisma"],
        &[],
        &[],
        &[],
        "line preserving; // comments removed",
    ),
    // ---------------------------------------------------------------- Data / query
    language(
        "json",
        "JSON / JSONC / JSON5",
        EngineKind::Json,
        CommentStyle::Line("//"),
        Category::Data,
        true,
        &[
            "json", "jsonc", "json5", "webmanifest", "map", "har",
            "geojson", "topojson", "ipynb", "avsc", "jsonld", "sarif",
        ],
        &[
            ".babelrc", ".eslintrc", ".prettierrc", ".stylelintrc",
            ".swcrc", ".commitlintrc", ".lintstagedrc", ".nycrc",
        ],
        &[],
        &["jsonc", "json5"],
        "comments and whitespace removed; single quotes accepted for JSON5",
    ),
    language(
        "yaml",
        "YAML",
        EngineKind::LineBased,
        CommentStyle::Line("#"),
        Category::Data,
        false,
        &["yaml", "yml"],
        &[],
        &[],
        &[],
        "indentation sensitive; line preserving; # comments removed",
    ),
    language(
        "toml",
        "TOML",
        EngineKind::Toml,
        CommentStyle::Line("#"),
        Category::Data,
        false,
        &["toml"],
        &["Cargo.lock", "poetry.lock", "Pipfile"],
        &[],
        &[],
        "line preserving; # comments removed",
    ),
    language(
        "sql",
        "SQL",
        EngineKind::Sql,
        CommentStyle::Line("--"),
        Category::Query,
        true,
        &["sql", "ddl", "dml", "psql", "pgsql", "mysql", "sqlite"],
        &[],
        &[],
        &[],
        "comments removed; strings and quoted identifiers preserved",
    ),
    language(
        "graphql",
        "GraphQL",
        EngineKind::GraphQl,
        CommentStyle::Line("#"),
        Category::Query,
        true,
        &["graphql", "gql", "graphqls"],
        &[],
        &[],
        &["gql"],
        "comments and whitespace removed; block strings preserved",
    ),
    language(
        "proto",
        "Protocol Buffers",
        EngineKind::CStyle,
        CommentStyle::Line("//"),
        Category::Data,
        true,
        &["proto"],
        &[],
        &[],
        &["protobuf"],
        "comments removed",
    ),
    language(
        "thrift",
        "Thrift",
        EngineKind::CStyle,
        CommentStyle::Line("//"),
        Category::Data,
        true,
        &["thrift"],
        &[],
        &[],
        &[],
        "comments removed",
    ),
    language(
        "flatbuffers",
        "FlatBuffers schema",
        EngineKind::CStyle,
        CommentStyle::Line("//"),
        Category::Data,
        true,
        &["fbs"],
        &[],
        &[],
        &[],
        "comments removed",
    ),
    language(
        "avro",
        "Avro IDL",
        EngineKind::CStyle,
        CommentStyle::Line("//"),
        Category::Data,
        true,
        &["avdl"],
        &[],
        &[],
        &[],
        "comments removed",
    ),
    language(
        "po",
        "Gettext PO",
        EngineKind::LineBased,
        CommentStyle::Line("#"),
        Category::Data,
        false,
        &["po", "pot"],
        &[],
        &[],
        &[],
        "line preserving; # comments removed",
    ),
    language(
        "wat",
        "WebAssembly text",
        EngineKind::LineBased,
        CommentStyle::Line(";;"),
        Category::Data,
        false,
        &["wat", "wast"],
        &[],
        &[],
        &["wasm-text"],
        "line preserving; ;; and (; ;) comments removed",
    ),
    // ---------------------------------------------------------------- Config / build
    language(
        "ini",
        "INI / config",
        EngineKind::LineBased,
        CommentStyle::Line(";"),
        Category::Config,
        false,
        &[
            "ini", "cfg", "conf", "cnf", "desktop", "service", "timer",
            "socket", "mount", "path", "target", "slice", "scope",
            "automount", "swap", "device", "network", "netdev", "link",
            "dnssd", "nspawn",
        ],
        &[
            ".editorconfig", ".gitconfig", ".gitmodules", ".npmrc",
            ".yarnrc", ".pylintrc", ".flake8", ".coveragerc", ".mypy.ini",
            ".pypirc", ".hgrc",
        ],
        &[],
        &["properties-ini", "config"],
        "line preserving; # and ; comments removed",
    ),
    language(
        "dockerfile",
        "Dockerfile",
        EngineKind::LineBased,
        CommentStyle::Line("#"),
        Category::Build,
        false,
        &["dockerfile", "containerfile"],
        &["Dockerfile", "Containerfile"],
        &["Dockerfile.", "Containerfile."],
        &["docker"],
        "line preserving; # comments removed; continuations kept",
    ),
    language(
        "makefile",
        "Makefile",
        EngineKind::LineBased,
        CommentStyle::Line("#"),
        Category::Build,
        false,
        &["mk", "make"],
        &["Makefile", "makefile", "GNUmakefile"],
        &["Makefile.", "makefile."],
        &["make"],
        "line preserving; leading tabs and # comments handled",
    ),
    language(
        "cmake",
        "CMake",
        EngineKind::LineBased,
        CommentStyle::Line("#"),
        Category::Build,
        false,
        &["cmake"],
        &["CMakeLists.txt"],
        &[],
        &[],
        "line preserving; # comments removed",
    ),
    language(
        "hcl",
        "HCL / Terraform",
        EngineKind::LineBased,
        CommentStyle::Line("#"),
        Category::Build,
        false,
        &["tf", "tfvars", "hcl", "nomad"],
        &[],
        &[],
        &["terraform", "terragrunt"],
        "line preserving; #, // and /* */ comments removed",
    ),
    language(
        "nix",
        "Nix",
        EngineKind::LineBased,
        CommentStyle::Line("#"),
        Category::Build,
        false,
        &["nix"],
        &[],
        &[],
        &[],
        "line preserving; # and /* */ comments removed",
    ),
    language(
        "just",
        "Just",
        EngineKind::LineBased,
        CommentStyle::Line("#"),
        Category::Build,
        false,
        &["just"],
        &["justfile", "Justfile", ".justfile"],
        &[],
        &[],
        "line preserving; # comments removed",
    ),
    language(
        "ninja",
        "Ninja",
        EngineKind::LineBased,
        CommentStyle::Line("#"),
        Category::Build,
        false,
        &["ninja"],
        &["build.ninja"],
        &[],
        &[],
        "line preserving; # comments removed",
    ),
    language(
        "gitignore",
        "Ignore files",
        EngineKind::LineBased,
        CommentStyle::Line("#"),
        Category::Config,
        false,
        &[
            "gitignore", "dockerignore", "npmignore", "eslintignore",
            "prettierignore", "stylelintignore", "vscodeignore",
            "hgignore", "cvsignore", "slugignore", "cfignore",
            "helmignore", "gcloudignore", "vercelignore", "netlifyignore",
            "nowignore", "firebaseignore", "amplifyignore",
            "serverlessignore", "terraformignore", "pulumiignore",
            "cdkignore", "sstignore", "wranglerignore", "denoignore",
            "rgignore", "fdignore", "agignore", "ignore",
        ],
        &[
            ".gitignore", ".dockerignore", ".npmignore", ".eslintignore",
            ".prettierignore", ".stylelintignore", ".vscodeignore",
            ".hgignore", ".cvsignore", ".slugignore", ".cfignore",
            ".helmignore", ".gcloudignore", ".vercelignore",
            ".netlifyignore", ".nowignore", ".firebaseignore",
            ".amplifyignore", ".serverlessignore", ".terraformignore",
            ".pulumiignore", ".cdkignore", ".sstignore", ".wranglerignore",
            ".denoignore", ".rgignore", ".fdignore", ".agignore", ".ignore",
        ],
        &[],
        &[],
        "line preserving; # comments removed",
    ),
    language(
        "browserslist",
        "Browserslist",
        EngineKind::LineBased,
        CommentStyle::Line("#"),
        Category::Config,
        false,
        &["browserslistrc"],
        &[".browserslistrc"],
        &[],
        &[],
        "line preserving; # comments removed",
    ),
    language(
        "procfile",
        "Procfile",
        EngineKind::LineBased,
        CommentStyle::Line("#"),
        Category::Config,
        false,
        &[],
        &["Procfile"],
        &[],
        &[],
        "line preserving; # comments removed",
    ),
    // ---------------------------------------------------------------- Shader
    language(
        "glsl",
        "GLSL",
        EngineKind::CStyle,
        CommentStyle::Line("//"),
        Category::Shader,
        true,
        &[
            "glsl", "vert", "frag", "geom", "tesc", "tese", "comp",
            "hlsl", "wgsl", "metal",
        ],
        &[],
        &[],
        &["hlsl", "wgsl", "metal"],
        "comments removed; preprocessor lines keep their own line",
    ),
    language(
        "opencl",
        "OpenCL / ISPC",
        EngineKind::CStyle,
        CommentStyle::Line("//"),
        Category::Shader,
        true,
        &["cl", "ispc"],
        &[],
        &[],
        &["opencl"],
        "comments removed; preprocessor lines keep their own line",
    ),
    // ---------------------------------------------------------------- Other text
    language(
        "strings",
        "Apple .strings",
        EngineKind::CStyle,
        CommentStyle::Line("//"),
        Category::Other,
        true,
        &["strings"],
        &[],
        &[],
        &[],
        "C-style comments removed; quoted pairs preserved",
    ),
    language(
        "qml",
        "QML",
        EngineKind::CStyle,
        CommentStyle::Line("//"),
        Category::Other,
        true,
        &["qml"],
        &[],
        &[],
        &[],
        "JavaScript-like comments removed",
    ),
];

/// File types that are intentionally excluded. They are listed by `--help` and
/// `--list-types` so users know why a file was skipped.
pub static EXCLUDED_TYPES: &[ExcludedType] = &[
    ExcludedType {
        name: "Python",
        extensions: &["py", "pyw", "pyi", "pyx", "pxd"],
        reason: "indentation is syntax; single-line output would change meaning",
    },
    ExcludedType {
        name: "Markdown",
        extensions: &["md", "markdown", "mdx", "mdown", "mkd"],
        reason: "line structure and spacing carry the formatting",
    },
    ExcludedType {
        name: "reStructuredText / AsciiDoc",
        extensions: &["rst", "adoc", "asciidoc", "org"],
        reason: "line structure and indentation carry the formatting",
    },
    ExcludedType {
        name: "Indented Sass / Stylus",
        extensions: &["sass", "styl", "sss"],
        reason: "indentation is syntax; use .scss or .css instead",
    },
    ExcludedType {
        name: "Pug / Haml / Slim",
        extensions: &["pug", "jade", "haml", "slim"],
        reason: "indentation is syntax",
    },
    ExcludedType {
        name: "F# / Elm / PureScript",
        extensions: &["fs", "fsi", "fsx", "elm", "purs"],
        reason: "indentation is syntax",
    },
    ExcludedType {
        name: "Starlark / Bazel / Meson",
        extensions: &["bzl", "bazel", "star"],
        reason: "Python-like indentation is syntax",
    },
    ExcludedType {
        name: "LaTeX / TeX",
        extensions: &["tex", "latex", "ltx", "sty", "cls"],
        reason: "whitespace and line breaks are semantically significant",
    },
    ExcludedType {
        name: "CSV / TSV / line-delimited JSON",
        extensions: &["csv", "tsv", "ndjson", "jsonl"],
        reason: "record boundaries and whitespace are data",
    },
    ExcludedType {
        name: "Diff / patch / logs",
        extensions: &["diff", "patch", "log"],
        reason: "line structure is the content",
    },
    ExcludedType {
        name: "Binary and archive formats",
        extensions: &[
            "png", "jpg", "jpeg", "gif", "webp", "ico", "bmp", "tif",
            "tiff", "ttf", "otf", "woff", "woff2", "eot", "mp3", "mp4",
            "wav", "ogg", "webm", "zip", "gz", "tgz", "bz2", "xz", "7z",
            "rar", "tar", "pdf", "doc", "docx", "xls", "xlsx", "ppt",
            "pptx", "exe", "dll", "so", "dylib", "a", "lib", "o", "obj",
            "class", "jar", "apk", "aab", "dex", "wasm", "keystore",
            "jks", "p12", "pfx", "der", "cer", "pem", "crt", "key",
        ],
        reason: "binary content is out of scope",
    },
];

/// Find a language by canonical id or alias, case insensitively.
pub fn find_language(name: &str) -> Option<&'static Language> {
    let needle = name.trim().to_lowercase();
    LANGUAGES.iter().find(|language| {
        language.id == needle
            || language
                .aliases
                .iter()
                .any(|alias| alias.to_lowercase() == needle)
            || language.display_name.to_lowercase() == needle
    })
}

/// Return the lower-case extension, if any.
fn path_extension(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| extension.to_lowercase())
}

/// Return the file name, lower-cased, if any.
fn path_file_name(path: &Path) -> Option<String> {
    path.file_name()
        .and_then(|file_name| file_name.to_str())
        .map(|file_name| file_name.to_lowercase())
}

/// Match a language by exact file name or file name prefix.
fn match_by_file_name(path: &Path) -> Option<&'static Language> {
    let file_name = path_file_name(path)?;
    LANGUAGES.iter().find(|language| {
        language
            .filenames
            .iter()
            .any(|candidate| candidate.to_lowercase() == file_name)
            || language
                .filename_prefixes
                .iter()
                .any(|prefix| file_name.starts_with(&prefix.to_lowercase()))
    })
}

/// Detect a language from a path without reading the file.
///
/// This is used while walking directories. Ambiguous extensions get a
/// best-effort answer; [`detect_language_from_source`] can refine them once
/// the contents are available.
pub fn detect_language(path: &Path) -> Option<&'static Language> {
    if let Some(language) = match_by_file_name(path) {
        return Some(language);
    }

    let extension = path_extension(path)?;
    let extension = extension.as_str();

    // Multi-dot names that need special handling before their final extension.
    let lower_name = path_file_name(path).unwrap_or_default();
    if lower_name.ends_with(".gradle.kts") {
        return find_language("kotlin");
    }
    if lower_name.ends_with(".d.ts")
        || lower_name.ends_with(".d.mts")
        || lower_name.ends_with(".d.cts")
    {
        return find_language("typescript");
    }

    LANGUAGES
        .iter()
        .find(|language| language.extensions.contains(&extension))
}

/// Detect a language using both the path and the file contents.
///
/// Only a few extensions are genuinely ambiguous. This function keeps the
/// registry honest without making every caller read the file twice.
pub fn detect_language_from_source(
    path: &Path,
    source: &str,
) -> Option<&'static Language> {
    let extension = path_extension(path).unwrap_or_default();
    let lower_name = path_file_name(path).unwrap_or_default();

    if extension == "config" {
        let trimmed = source.trim_start();
        if trimmed.starts_with('<') {
            return find_language("xml");
        }
        return find_language("ini");
    }

    if extension == "m" {
        let looks_like_objc = source.contains("#import")
            || source.contains("@interface")
            || source.contains("@implementation")
            || source.contains("@end");
        if looks_like_objc {
            return find_language("objectivec");
        }
        return None;
    }

    if extension == "inc" {
        if source.contains("<?php") {
            return find_language("php");
        }
        return find_language("c");
    }

    if extension == "lock" {
        return match lower_name.as_str() {
            "cargo.lock" | "poetry.lock" => find_language("toml"),
            "pipfile.lock" | "composer.lock" | "flake.lock" => {
                find_language("json")
            }
            "pnpm-lock.yaml" | "pubspec.lock" | "podfile.lock" => {
                find_language("yaml")
            }
            "yarn.lock" | "gemfile.lock" => find_language("gitignore"),
            _ => None,
        };
    }

    detect_language(path)
}

/// Render the complete supported/excluded type listing for `--list-types`.
pub fn type_listing() -> String {
    fn extensions_of(language: &Language) -> String {
        let mut parts: Vec<String> = Vec::new();
        let mut push = |value: String| {
            if !parts.contains(&value) {
                parts.push(value);
            }
        };
        for extension in language.extensions {
            push(format!(".{}", extension));
        }
        for filename in language.filenames {
            push((*filename).to_string());
        }
        if parts.is_empty() {
            "-".to_string()
        } else {
            parts.join(", ")
        }
    }

    let categories = [
        Category::Rust,
        Category::Web,
        Category::Android,
        Category::Jvm,
        Category::Systems,
        Category::Scripting,
        Category::Data,
        Category::Config,
        Category::Build,
        Category::Markup,
        Category::Stylesheet,
        Category::Query,
        Category::Shader,
        Category::Other,
    ];

    let mut output = String::new();
    output.push_str(&format!(
        "Supported types ({}):\n",
        LANGUAGES.len()
    ));
    for category in categories {
        let group: Vec<&Language> = LANGUAGES
            .iter()
            .filter(|language| language.category == category)
            .collect();
        if group.is_empty() {
            continue;
        }
        output.push_str(&format!("\n[{}]\n", category.name()));
        for language in group {
            output.push_str(&format!(
                "  {:<16} {:<28} {}\n",
                language.id,
                language.display_name,
                extensions_of(language)
            ));
        }
    }

    output.push_str(&format!(
        "\nExcluded types ({}):\n",
        EXCLUDED_TYPES.len()
    ));
    for excluded in EXCLUDED_TYPES {
        let extensions: Vec<String> = excluded
            .extensions
            .iter()
            .map(|extension| format!(".{}", extension))
            .collect();
        output.push_str(&format!(
            "  {:<28} {:<24} {}\n",
            excluded.name,
            extensions.join(", "),
            excluded.reason
        ));
    }
    output
}

/// A compact list of supported extensions, used by help text.
pub fn supported_extension_summary() -> String {
    let mut extensions: Vec<&'static str> = Vec::new();
    for language in LANGUAGES {
        for extension in language.extensions {
            if !extensions.contains(extension) {
                extensions.push(extension);
            }
        }
    }
    extensions.sort_unstable();
    extensions
        .into_iter()
        .map(|extension| format!(".{}", extension))
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
#[path = "language_tests.rs"]
mod tests;
