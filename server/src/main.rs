#![forbid(unsafe_code)]
#![recursion_limit = "512"]

use server::error::BoxError;
use server::standalone::async_main as standalone_main;
use tokio::runtime::Builder;

// The app has grown in stack size and we need to make it bigger. This can
// happen when we have too many await points as they are stored on the stack.
// I'm not entirely sure what can be done about it, but we set two different
// values for dev and release mode which seems to mitigate tokio's stack
// overflow.
#[cfg(debug_assertions)]
const THREAD_STACK_SIZE: usize = 16 * 1024 * 1024; // 16 MiB
#[cfg(not(debug_assertions))]
const THREAD_STACK_SIZE: usize = 8 * 1024 * 1024; // 8 MiB

fn main() -> Result<(), BoxError> {
    // Load from .env
    let _ = dotenvy::dotenv();

    let rt = Builder::new_multi_thread()
        .thread_stack_size(THREAD_STACK_SIZE)
        .enable_all()
        .build()
        .expect("failed to build runtime");

    rt.block_on(async {
        standalone_main().await
    })
}
