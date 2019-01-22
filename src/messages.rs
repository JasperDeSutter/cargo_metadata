use super::{Diagnostic, PackageId, Target};
use std::io::Read;
use std::path::PathBuf;
use serde_json;

/// Profile settings used to determine which compiler flags to use for a
/// target.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactProfile {
    /// Optimization level. Possible values are 0-3, s or z.
    pub opt_level: String,
    /// The amount of debug info. 0 for none, 1 for limited, 2 for full
    pub debuginfo: Option<u32>,
    /// State of the `cfg(debug_assertions)` directive, enabling macros like
    /// `debug_assert!`
    pub debug_assertions: bool,
    /// State of the overflow checks.
    pub overflow_checks: bool,
    /// Whether this profile is a test
    pub test: bool,
    #[doc(hidden)]
    #[serde(skip)]
    __do_not_match_exhaustively: (),
}

/// A compiler-generated file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    /// The workspace member this artifact belongs to
    pub package_id: PackageId,
    /// The target this artifact was compiled for
    pub target: Target,
    /// The profile this artifact was compiled with
    pub profile: ArtifactProfile,
    /// The enabled features for this artifact
    pub features: Vec<String>,
    /// The full paths to the generated artifacts
    pub filenames: Vec<PathBuf>,
    /// If true, then the files were already generated
    pub fresh: bool,
    #[doc(hidden)]
    #[serde(skip)]
    __do_not_match_exhaustively: (),
}

/// Message left by the compiler
// TODO: structify message
// TODO: Better name. This one comes from machine_message.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FromCompiler {
    /// The workspace member this message belongs to
    pub package_id: PackageId,
    /// The target this message is aimed at
    pub target: Target,
    /// The message the compiler sent.
    pub message: Diagnostic,
    #[doc(hidden)]
    #[serde(skip)]
    __do_not_match_exhaustively: (),
}

/// Output of a build script execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildScript {
    /// The workspace member this build script execution belongs to
    pub package_id: PackageId,
    /// The libs to link
    pub linked_libs: Vec<PathBuf>,
    /// The paths to search when resolving libs
    pub linked_paths: Vec<PathBuf>,
    /// The paths to search when resolving libs
    pub cfgs: Vec<PathBuf>,
    /// The environment variables to add to the compilation
    pub env: Vec<(String, String)>,
    #[doc(hidden)]
    #[serde(skip)]
    __do_not_match_exhaustively: (),
}

/// A cargo message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "reason", rename_all = "kebab-case")]
pub enum Message {
    /// The compiler generated an artifact
    CompilerArtifact(Artifact),
    /// The compiler wants to display a message
    CompilerMessage(FromCompiler),
    /// A build script successfully executed.
    BuildScriptExecuted(BuildScript),
    #[doc(hidden)]
    #[serde(other)]
    Unknown,
}

impl ToString for FromCompiler {
    fn to_string(&self) -> String {
        self.message.to_string()
    }
}

/// An iterator of Message.
type MessageIterator<R> =
    serde_json::StreamDeserializer<'static, serde_json::de::IoRead<R>, Message>;

/// Creates an iterator of Message from a Read outputting a stream of JSON
/// messages. For usage information, look at the top-level documentation.
pub fn parse_messages<R: Read>(input: R) -> MessageIterator<R> {
    serde_json::Deserializer::from_reader(input).into_iter::<Message>()
}
