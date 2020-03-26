//! Types and traits for working with asynchronous tasks.
//!
//! This module is similar to [`std::thread`], except it uses asynchronous tasks in place of
//! threads.
//!
//! [`std::thread`]: https://doc.rust-lang.org/std/thread
//!
//! ## The task model
//!
//! An executing asynchronous Rust program consists of a collection of native OS threads, on top of
//! which multiple stackless coroutines are multiplexed. We refer to these as "tasks".  Tasks can
//! be named, and provide some built-in support for synchronization.
//!
//! Communication between tasks can be done through channels, Rust's message-passing types, along
//! with [other forms of tasks synchronization](../sync/index.html) and shared-memory data
//! structures. In particular, types that are guaranteed to be threadsafe are easily shared between
//! tasks using the atomically-reference-counted container, [`Arc`].
//!
//! Fatal logic errors in Rust cause *thread panic*, during which a thread will unwind the stack,
//! running destructors and freeing owned resources. If a panic occurs inside a task, there is no
//! meaningful way of recovering, so the panic will propagate through any thread boundaries all the
//! way to the root task. This is also known as a "panic = abort" model.
//!
//! ## Spawning a task
//!
//! A new task can be spawned using the [`task::spawn`][`spawn`] function:
//!
//! ```no_run
//! use async_std::task;
//!
//! task::spawn(async {
//!     // some work here
//! });
//! ```
//!
//! In this example, the spawned task is "detached" from the current task. This means that it can
//! outlive its parent (the task that spawned it), unless this parent is the root task.
//!
//! The root task can also wait on the completion of the child task; a call to [`spawn`] produces a
//! [`JoinHandle`], which implements `Future` and can be `await`ed:
//!
//! ```
//! use async_std::task;
//!
//! # async_std::task::block_on(async {
//! #
//! let child = task::spawn(async {
//!     // some work here
//! });
//! // some work here
//! let res = child.await;
//! #
//! # })
//! ```
//!
//! The `await` operator returns the final value produced by the child task.
//!
//! ## Task-local storage
//!
//! This module also provides an implementation of task-local storage for Rust
//! programs. Task-local storage is a method of storing data into a global
//! variable that each task in the program will have its own copy of.
//! Tasks do not share this data, so accesses do not need to be synchronized.
//!
//! A task-local key owns the value it contains and will destroy the value when the
//! task exits. It is created with the [`task_local!`] macro and can contain any
//! value that is `'static` (no borrowed pointers). It provides an accessor function,
//! [`with`], that yields a shared reference to the value to the specified
//! closure. Task-local keys allow only shared access to values, as there would be no
//! way to guarantee uniqueness if mutable borrows were allowed.
//!
//! [`Arc`]: ../sync/struct.Arc.html
//! [`spawn`]: fn.spawn.html
//! [`JoinHandle`]: struct.JoinHandle.html
//! [`join`]: struct.JoinHandle.html#method.join
//! [`panic!`]: https://doc.rust-lang.org/std/macro.panic.html
//! [`Task`]: struct.Task.html
//! [`task_local!`]: ../macro.task_local.html
//! [`with`]: struct.LocalKey.html#method.with

cfg_std! {
    #[doc(inline)]
    pub use std::task::{Context, Poll, Waker};

    pub use ready::ready;
    pub use yield_now::yield_now;
    mod ready;
    mod yield_now;
}

cfg_default! {
    pub use block_on::block_on;
    pub use sleep::sleep;
    pub use tokio::task::*;

    mod block_on;
    mod sleep;

}
