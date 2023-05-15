use crate::models::Model;

/// An abstract interface to the Fornjot host.
pub trait Host {
    /// Register a model.
    ///
    /// This is mainly for more advanced use cases (e.g. when you need to close
    /// over extra state to load the model). For simpler models, you probably
    /// want to use [`HostExt::register_model()`] instead.
    fn register_boxed_model(&mut self, model: Box<dyn Model>);
}

/// Extension methods to augment the [`Host`] API.
///
/// The purpose of this trait is to keep [`Host`] object-safe.
pub trait HostExt {
    /// Register a model with the Fornjot runtime.
    fn register_model<M>(&mut self, model: M)
    where
        M: Model + 'static;
}

impl<H: Host + ?Sized> HostExt for H {
    fn register_model<M>(&mut self, model: M)
    where
        M: Model + 'static,
    {
        self.register_boxed_model(Box::new(model));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_is_object_safe() {
        let _: &dyn Host;
    }
}
