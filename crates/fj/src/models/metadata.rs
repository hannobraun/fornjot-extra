/// Information about a particular module that can be used by the host for
/// things like introspection and search.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Metadata {
    /// A short, human-friendly name used to identify this module.
    pub name: String,

    /// A semver-compliant version number.
    pub version: String,

    /// A short, one-line description.
    pub short_description: Option<String>,

    /// A more elaborate description.
    pub description: Option<String>,

    /// A link to the homepage.
    pub homepage: Option<String>,

    /// A link to the source code.
    pub repository: Option<String>,

    /// The name of the software license(s) this software is released under.
    ///
    /// This is interpreted as a SPDX license expression (e.g.  `MIT OR
    /// Apache-2.0`). See [the SPDX site][spdx] for more information.
    ///
    /// [spdx]: https://spdx.dev/spdx-specification-21-web-version/#h.jxpfx0ykyb60
    pub license: Option<String>,
}

impl Metadata {
    /// Create a [`Metadata`] object with the bare minimum required fields.
    ///
    /// # Panics
    ///
    /// The `name` and `version` fields must not be empty.
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        let name = name.into();
        assert!(!name.is_empty());
        let version = version.into();
        assert!(!version.is_empty());

        Self {
            name,
            version,
            short_description: None,
            description: None,
            homepage: None,
            repository: None,
            license: None,
        }
    }

    /// Set the [`Metadata::short_description`] field.
    pub fn with_short_description(
        self,
        short_description: impl Into<String>,
    ) -> Self {
        let short_description = short_description.into();
        if short_description.is_empty() {
            return self;
        }

        Self {
            short_description: Some(short_description),
            ..self
        }
    }

    /// Set the [`Metadata::description`] field.
    pub fn with_description(self, description: impl Into<String>) -> Self {
        let description = description.into();
        if description.is_empty() {
            return self;
        }

        Self {
            description: Some(description),
            ..self
        }
    }

    /// Set the [`Metadata::homepage`] field.
    pub fn with_homepage(self, homepage: impl Into<String>) -> Self {
        let homepage = homepage.into();
        if homepage.is_empty() {
            return self;
        }

        Self {
            homepage: Some(homepage),
            ..self
        }
    }

    /// Set the [`Metadata::repository`] field.
    pub fn with_repository(self, repository: impl Into<String>) -> Self {
        let repository = repository.into();
        if repository.is_empty() {
            return self;
        }

        Self {
            repository: Some(repository),
            ..self
        }
    }

    /// Set the [`Metadata::license`] field.
    pub fn with_license(self, license: impl Into<String>) -> Self {
        let license = license.into();
        if license.is_empty() {
            return self;
        }

        Self {
            license: Some(license),
            ..self
        }
    }
}

/// Metadata about a [`crate::models::Model`].
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ModelMetadata {
    /// A short, human-friendly name used to identify this model.
    pub name: String,

    /// A description of what this model does.
    pub description: Option<String>,

    /// Arguments that the model uses when calculating its geometry.
    pub arguments: Vec<ArgumentMetadata>,
}

impl ModelMetadata {
    /// Create metadata for a model.
    ///
    /// # Panics
    ///
    /// The `name` must not be empty.
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        assert!(!name.is_empty());

        Self {
            name,
            description: None,
            arguments: Vec::new(),
        }
    }

    /// Set the [`ModelMetadata::description`].
    pub fn with_description(self, description: impl Into<String>) -> Self {
        let description = description.into();
        if description.is_empty() {
            return self;
        }

        Self {
            description: Some(description),
            ..self
        }
    }

    /// Add an argument to the [`ModelMetadata::arguments`] list.
    ///
    /// As a convenience, string literals can be automatically converted into
    /// [`ArgumentMetadata`] with no description or default value.
    pub fn with_argument(mut self, arg: impl Into<ArgumentMetadata>) -> Self {
        self.arguments.push(arg.into());
        self
    }
}

/// Metadata describing a model's argument.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ArgumentMetadata {
    /// The name used to refer to this argument.
    pub name: String,

    /// A short description of this argument that could be shown to the user
    /// in something like a tooltip.
    pub description: Option<String>,

    /// Something that could be used as a default if no value was provided.
    pub default_value: Option<String>,
}

impl ArgumentMetadata {
    /// Create a new [`ArgumentMetadata`].
    ///
    /// # Panics
    ///
    /// The `name` must not be empty.
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        assert!(!name.is_empty());
        Self {
            name,
            description: None,
            default_value: None,
        }
    }

    /// Set the [`ArgumentMetadata::description`].
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        let description = description.into();
        if description.is_empty() {
            return self;
        }

        self.description = Some(description);
        self
    }

    /// Set the [`ArgumentMetadata::default_value`].
    pub fn with_default_value(
        mut self,
        default_value: impl Into<String>,
    ) -> Self {
        self.default_value = Some(default_value.into());
        self
    }
}

impl From<&str> for ArgumentMetadata {
    fn from(name: &str) -> Self {
        Self::new(name)
    }
}
