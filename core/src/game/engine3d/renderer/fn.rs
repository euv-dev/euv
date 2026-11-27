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
        let dispatch_workers: usize = pool.worker_count.min(total_chunks.saturating_sub(1));
        if dispatch_workers == 0 {
            work(0, total_items);
            return;
        }
        let job: Engine3dJob = Engine3dJob {
            trampoline: invoke_engine3d_chunk::<F>,
            ctx_ptr: (&raw const work) as usize,
            total_items,
            chunk_size: grain,
            total_chunks,
            next_chunk: AtomicUsize::new(0),
            workers_done: AtomicUsize::new(0),
        };
        let shared: &Engine3dPoolShared = &pool.shared;
        shared
            .active_workers
            .store(dispatch_workers, Ordering::Relaxed);
        shared
            .active_job
            .store((&raw const job) as usize, Ordering::Release);
        let active_slots: &[Engine3dWorkerSlot] = &shared.slots[..dispatch_workers];
        for slot in active_slots.iter() {
            let next_epoch: usize = slot.epoch.load(Ordering::Relaxed).wrapping_add(1);
            slot.epoch.store(next_epoch, Ordering::SeqCst);
        }
        for slot in active_slots.iter() {
            if slot.parked.load(Ordering::SeqCst) != 0
                && let Ok(_guard) = slot.lock.lock()
            {
                slot.wake.notify_one();
            }
        }
        run_engine3d_job_loop(&job);
        let mut caller_spins: u32 = 0;
        while job.workers_done.load(Ordering::Acquire) < dispatch_workers {
            if caller_spins < ENGINE3D_CALLER_SPIN_LIMIT {
                caller_spins += 1;
                spin_loop();
                continue;
            }
            if let Ok(mut guard) = shared.caller_lock.lock() {
                shared.caller_parked.store(1, Ordering::SeqCst);
                while job.workers_done.load(Ordering::SeqCst) < dispatch_workers {
                    let Ok(next_guard) = shared.wake_caller.wait(guard) else {
                        break;
                    };
                    guard = next_guard;
                }
                shared.caller_parked.store(0, Ordering::SeqCst);
            } else {
                spin_loop();
            }
        }
        shared.active_job.store(0, Ordering::Release);
    }
    #[cfg(target_arch = "wasm32")]
    {
        let _: usize = chunk_size;
        work(0, total_items);
    }
}
