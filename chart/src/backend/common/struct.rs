/// Creates the best rendering backend available on the host.
///
/// The preference order is WebGPU, then WebGL2, then Canvas2D. A canvas
/// can only ever have one kind of context, so the choice has to be made
/// before the first `get_context` call — which is why the WebGPU probe
/// is asynchronous and the factory is asynchronous too.
#[derive(Clone, Copy, Debug, Default)]
pub struct BackendFactory;
