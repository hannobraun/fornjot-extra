//! # Fornjot Model Host
//!
//! This library is part of the [Fornjot] ecosystem. Fornjot is an open-source,
//! code-first CAD application; and collection of libraries that make up the CAD
//! application, but can be used independently.
//!
//! This library is an internal component of Fornjot. It is not relevant to end
//! users that just want to create CAD models.
//!
//! The purpose of this library is to load Fornjot models and watch them for
//! changes. Fornjot models are basically plugins that can be loaded into a CAD
//! application. This library is the host for these model plugins.
//!
//! [Fornjot]: https://www.fornjot.app/

#![warn(missing_docs)]

mod host;
mod host_thread;
mod model;
mod parameters;
mod platform;
mod watcher;

pub(crate) use self::host_thread::{EventLoopClosed, HostThread};

pub use self::{
    host::{Host, HostCommand},
    host_thread::ModelEvent,
    model::{Error, Evaluation, Model},
    parameters::Parameters,
    watcher::Watcher,
};
