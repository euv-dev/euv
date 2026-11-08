use super::*;

/// One native deliverable in a resolved build plan.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NativeTarget {
    /// The platform this deliverable produces.
    pub platform: NativePlatform,
    /// The kind of builder this deliverable needs.
    pub task: NativeTask,
}

/// The resolved plan for one native build invocation.
///
/// The plan names the host it was resolved against, the deliverables that
/// will be built, and the platforms that were requested but cannot be
/// built here.
#[derive(Clone, Data, Debug)]
pub struct NativePlan {
    /// The host operating system the plan was resolved against.
    #[get(pub(crate))]
    pub host: HostOs,
    /// The host architecture the plan was resolved against.
    #[get(pub(crate))]
    pub arch: String,
    /// The deliverables that will be built, in build order.
    #[get(pub(crate))]
    pub targets: Vec<NativeTarget>,
    /// The platforms that were requested but cannot be built on this host.
    #[get(pub(crate))]
    pub unsupported: Vec<NativePlatform>,
}

/// The native build settings a command line carries.
///
/// The flags that select native work live beside the ordinary build
/// flags, so `euv build` keeps its current behaviour and the native
/// options only take effect when they are given.
#[derive(Clone, Data, Debug, Parser)]
pub struct BuildArgs {
    /// The ordinary build arguments shared with the run mode.
    #[command(flatten)]
    #[get(pub(crate))]
    pub mode: ModeArgs,
    /// Build every native deliverable this host can produce.
    #[arg(long)]
    #[get(pub(crate), type(copy))]
    pub all: bool,
    /// Build only the named native platforms.
    #[arg(long, value_delimiter = ',')]
    #[get(pub(crate))]
    pub platforms: Vec<NativePlatform>,
    /// The native client project directory, when it is not inside this repository.
    #[arg(long)]
    #[get(pub(crate))]
    pub native_path: Option<PathBuf>,
    /// Print the resolved native plan and exit without building.
    #[arg(long)]
    #[get(pub(crate), type(copy))]
    pub plan: bool,
}
