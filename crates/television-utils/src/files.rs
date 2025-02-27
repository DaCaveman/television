use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::{collections::HashSet, path::PathBuf};

use ignore::{overrides::Override, types::TypesBuilder, WalkBuilder};
use lazy_static::lazy_static;
use tracing::{debug, warn};

use crate::strings::{
    proportion_of_printable_ascii_characters, PRINTABLE_ASCII_THRESHOLD,
};
use crate::threads::default_num_threads;

lazy_static::lazy_static! {
    pub static ref DEFAULT_NUM_THREADS: usize = default_num_threads().into();
}

pub fn walk_builder(
    path: &Path,
    n_threads: usize,
    overrides: Option<Override>,
    ignore_paths: Option<Vec<PathBuf>>,
) -> WalkBuilder {
    let mut builder = WalkBuilder::new(path);

    // ft-based filtering
    let mut types_builder = TypesBuilder::new();
    types_builder.add_defaults();
    builder.types(types_builder.build().unwrap());

    // ignore paths
    if let Some(paths) = ignore_paths {
        builder.filter_entry(move |e| {
            let path = e.path();
            if paths.iter().any(|p| path.starts_with(p)) {
                debug!("Ignoring path: {:?}", path);
                return false;
            }
            true
        });
    }

    builder.threads(n_threads);
    if let Some(ov) = overrides {
        builder.overrides(ov);
    }
    builder
}

pub fn get_file_size(path: &Path) -> Option<u64> {
    std::fs::metadata(path).ok().map(|m| m.len())
}

#[derive(Debug)]
pub enum FileType {
    Text,
    Other,
    Unknown,
}

impl<P> From<P> for FileType
where
    P: AsRef<Path> + Debug,
{
    fn from(path: P) -> Self {
        debug!("Getting file type for {:?}", path);
        let p = path.as_ref();
        if is_known_text_extension(p) {
            return FileType::Text;
        }
        if let Ok(mut f) = File::open(p) {
            let mut buffer = [0u8; 256];
            if let Ok(bytes_read) = f.read(&mut buffer) {
                if bytes_read > 0
                    && proportion_of_printable_ascii_characters(
                        &buffer[..bytes_read],
                    ) > PRINTABLE_ASCII_THRESHOLD
                {
                    return FileType::Text;
                }
            }
        } else {
            warn!("Error opening file: {:?}", path);
        }
        FileType::Other
    }
}

pub fn is_known_text_extension<P>(path: P) -> bool
where
    P: AsRef<Path>,
{
    path.as_ref()
        .extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| KNOWN_TEXT_FILE_EXTENSIONS.contains(ext))
}

lazy_static! {
    static ref KNOWN_TEXT_FILE_EXTENSIONS: HashSet<&'static str> = [
        "ada",
        "adb",
        "ads",
        "applescript",
        "as",
        "asc",
        "ascii",
        "ascx",
        "asm",
        "asmx",
        "asp",
        "aspx",
        "atom",
        "au3",
        "awk",
        "bas",
        "bash",
        "bashrc",
        "bat",
        "bbcolors",
        "bcp",
        "bdsgroup",
        "bdsproj",
        "bib",
        "bowerrc",
        "c",
        "cbl",
        "cc",
        "cfc",
        "cfg",
        "cfm",
        "cfml",
        "cgi",
        "cjs",
        "clj",
        "cljs",
        "cls",
        "cmake",
        "cmd",
        "cnf",
        "cob",
        "code-snippets",
        "coffee",
        "coffeekup",
        "conf",
        "cp",
        "cpp",
        "cpt",
        "cpy",
        "crt",
        "cs",
        "csh",
        "cson",
        "csproj",
        "csr",
        "css",
        "csslintrc",
        "csv",
        "ctl",
        "curlrc",
        "cxx",
        "d",
        "dart",
        "dfm",
        "diff",
        "dof",
        "dpk",
        "dpr",
        "dproj",
        "dtd",
        "eco",
        "editorconfig",
        "ejs",
        "el",
        "elm",
        "emacs",
        "eml",
        "ent",
        "erb",
        "erl",
        "eslintignore",
        "eslintrc",
        "ex",
        "exs",
        "f",
        "f03",
        "f77",
        "f90",
        "f95",
        "fish",
        "for",
        "fpp",
        "frm",
        "fs",
        "fsproj",
        "fsx",
        "ftn",
        "gemrc",
        "gemspec",
        "gitattributes",
        "gitconfig",
        "gitignore",
        "gitkeep",
        "gitmodules",
        "go",
        "gpp",
        "gradle",
        "graphql",
        "groovy",
        "groupproj",
        "grunit",
        "gtmpl",
        "gvimrc",
        "h",
        "haml",
        "hbs",
        "hgignore",
        "hh",
        "hpp",
        "hrl",
        "hs",
        "hta",
        "htaccess",
        "htc",
        "htm",
        "html",
        "htpasswd",
        "hxx",
        "iced",
        "iml",
        "inc",
        "inf",
        "info",
        "ini",
        "ino",
        "int",
        "irbrc",
        "itcl",
        "itermcolors",
        "itk",
        "jade",
        "java",
        "jhtm",
        "jhtml",
        "js",
        "jscsrc",
        "jshintignore",
        "jshintrc",
        "json",
        "json5",
        "jsonld",
        "jsp",
        "jspx",
        "jsx",
        "ksh",
        "less",
        "lhs",
        "lisp",
        "log",
        "ls",
        "lsp",
        "lua",
        "m",
        "m4",
        "mak",
        "map",
        "markdown",
        "master",
        "md",
        "mdown",
        "mdwn",
        "mdx",
        "metadata",
        "mht",
        "mhtml",
        "mjs",
        "mk",
        "mkd",
        "mkdn",
        "mkdown",
        "ml",
        "mli",
        "mm",
        "mxml",
        "nfm",
        "nfo",
        "noon",
        "npmignore",
        "npmrc",
        "nuspec",
        "nvmrc",
        "ops",
        "pas",
        "pasm",
        "patch",
        "pbxproj",
        "pch",
        "pem",
        "pg",
        "php",
        "php3",
        "php4",
        "php5",
        "phpt",
        "phtml",
        "pir",
        "pl",
        "pm",
        "pmc",
        "pod",
        "pot",
        "prettierrc",
        "properties",
        "props",
        "pt",
        "pug",
        "purs",
        "py",
        "pyx",
        "r",
        "rake",
        "rb",
        "rbw",
        "rc",
        "rdoc",
        "rdoc_options",
        "resx",
        "rexx",
        "rhtml",
        "rjs",
        "rlib",
        "ron",
        "rs",
        "rss",
        "rst",
        "rtf",
        "rvmrc",
        "rxml",
        "s",
        "sass",
        "scala",
        "scm",
        "scss",
        "seestyle",
        "sh",
        "shtml",
        "sln",
        "sls",
        "spec",
        "sql",
        "sqlite",
        "sqlproj",
        "srt",
        "ss",
        "sss",
        "st",
        "strings",
        "sty",
        "styl",
        "stylus",
        "sub",
        "sublime-build",
        "sublime-commands",
        "sublime-completions",
        "sublime-keymap",
        "sublime-macro",
        "sublime-menu",
        "sublime-project",
        "sublime-settings",
        "sublime-workspace",
        "sv",
        "svc",
        "svg",
        "swift",
        "t",
        "tcl",
        "tcsh",
        "terminal",
        "tex",
        "text",
        "textile",
        "tg",
        "tk",
        "tmLanguage",
        "tmpl",
        "tmTheme",
        "toml",
        "tpl",
        "ts",
        "tsv",
        "tsx",
        "tt",
        "tt2",
        "ttml",
        "twig",
        "txt",
        "v",
        "vb",
        "vbproj",
        "vbs",
        "vcproj",
        "vcxproj",
        "vh",
        "vhd",
        "vhdl",
        "vim",
        "viminfo",
        "vimrc",
        "vm",
        "vue",
        "webapp",
        "webmanifest",
        "wsc",
        "x-php",
        "xaml",
        "xht",
        "xhtml",
        "xml",
        "xs",
        "xsd",
        "xsl",
        "xslt",
        "y",
        "yaml",
        "yml",
        "zsh",
        "zshrc",
    ]
    .into();
}
