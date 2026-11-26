#[cfg(not(target_arch = "wasm32"))]
use super::*;

/// Hermite smoothstep interpolation in `[0.0, 1.0]` for analytical anti-aliasing of procedural surfaces.
///
/// # Arguments
///
/// - `f32` - Lower transition edge.
/// - `f32` - Upper transition edge.
/// - `f32` - Input scalar.
///
/// # Returns
///
/// - `f32` - Smooth $C^1$ interpolation weight in `[0.0, 1.0]`.
#[must_use]
pub(crate) fn smoothstep_f32(edge0: f32, edge1: f32, x: f32) -> f32 {
    let span: f32 = (edge1 - edge0).max(1e-5);
    let t: f32 = ((x - edge0) / span).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Returns the process-wide persistent 3D worker pool, lazily spawning warm worker threads once.
///
/// # Returns
///
/// - `&'static Engine3dWorkerPool` - Shared persistent worker pool reference.
#[cfg(not(target_arch = "wasm32"))]
#[must_use]
pub(crate) fn global_engine3d_pool() -> &'static Engine3dWorkerPool {
    ENGINE3D_WORKER_POOL.get_or_init(Engine3dWorkerPool::new)
}

/// Executes dynamic work-stealing chunks for `job` until all chunks are claimed.
///
/// # Arguments
///
/// - `&Engine3dJob` - Caller-pinned parallel chunk job descriptor.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn run_engine3d_job_loop(job: &Engine3dJob) {
    loop {
        let chunk_idx: usize = job.next_chunk.fetch_add(1, Ordering::AcqRel);
        if chunk_idx >= job.total_chunks {
            break;
        }
        let start: usize = chunk_idx * job.chunk_size;
        let end: usize = (start + job.chunk_size).min(job.total_items);
        if start < end {
            (job.trampoline)(job.ctx_ptr, start, end);
        }
    }
}

/// Type-erased trampoline invoking `F(start, end)` on a claimed range `[start, end)`.
///
/// # Arguments
///
/// - `usize` - Type-erased `&F` pointer address.
/// - `usize` - Inclusive start row/item index.
/// - `usize` - Exclusive end row/item index.
#[cfg(not(target_arch = "wasm32"))]
fn invoke_engine3d_chunk<F>(ctx_ptr: usize, start: usize, end: usize)
where
    F: Fn(usize, usize) + Sync,
{
    let work: &F = unsafe { &*(ctx_ptr as *const F) };
    work(start, end);
}

/// Dispatches disjoint index ranges `[start, end)` across the persistent warm 3D thread pool (zero thread creation per frame).
///
/// # Arguments
///
/// - `usize` - Total item count (e.g., rows).
/// - `usize` - Minimum chunk granularity per worker claim.
/// - `F` - Thread-safe range worker closure receiving `(start_idx, end_idx)`.
pub(crate) fn parallel_for_ranges<F>(total_items: usize, chunk_size: usize, work: F)
where
    F: Fn(usize, usize) + Sync,
{
    if total_items == 0 {
        return;
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let pool: &'static Engine3dWorkerPool = global_engine3d_pool();
        let grain: usize = chunk_size.max(1);
        let total_chunks: usize = total_items.div_ceil(grain);
        if pool.worker_count == 0 || total_chunks <= 1 {
            work(0, total_items);
            return;
        }
        let Ok(_dispatch_guard) = pool.dispatch_lock.try_lock() else {
            work(0, total_items);
            return;
        };
        let job: Engine3dJob = Engine3dJob {
            trampoline: invoke_engine3d_chunk::<F>,
            ctx_ptr: (&raw const work) as usize,
            total_items,
            chunk_size: grain,
            total_chunks,
            next_chunk: AtomicUsize::new(0),
            workers_done: AtomicUsize::new(0),
        };
        let job_ptr: usize = (&raw const job) as usize;
        {
            let (lock, wake_workers, _): &(Mutex<Engine3dPoolState>, Condvar, Condvar) =
                &pool.shared;
            if let Ok(mut state) = lock.lock() {
                let next_epoch: u64 = state.get_epoch().wrapping_add(1);
                state.set_epoch(next_epoch);
                state.set_active_job(job_ptr);
                wake_workers.notify_all();
            } else {
                work(0, total_items);
                return;
            }
        }
        run_engine3d_job_loop(&job);
        {
            let (lock, _, wake_caller): &(Mutex<Engine3dPoolState>, Condvar, Condvar) =
                &pool.shared;
            if let Ok(mut state) = lock.lock() {
                while job.workers_done.load(Ordering::Acquire) < pool.worker_count {
                    match wake_caller.wait(state) {
                        Ok(next_guard) => state = next_guard,
                        Err(poisoned) => {
                            state = poisoned.into_inner();
                            break;
                        }
                    }
                }
                state.set_active_job(0);
            }
        }
    }
    #[cfg(target_arch = "wasm32")]
    {
        let _: usize = chunk_size;
        work(0, total_items);
    }
}
