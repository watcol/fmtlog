#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Modules(Vec<Module>);

impl From<Module> for Modules {
    fn from(m: Module) -> Self {
        Self(vec![m])
    }
}

impl<T: IntoIterator> From<T> for Modules
where
    T::Item: Into<Module> + Clone,
{
    fn from(m: T) -> Self {
        m.into_iter().fold(Self::new(), |acc, x| acc.append(x))
    }
}

impl Modules {
    fn new() -> Self {
        Self::default()
    }

    // Append a module when it is not duplicate existing module.
    fn append<T: Into<Module> + Clone>(self, module: T) -> Self {
        if self.0.iter().any(|m| m.is_parent(&module)) {
            return self;
        }

        let mut new: Vec<_> = self
            .0
            .into_iter()
            .filter(|m| !m.is_child(&module))
            .collect();

        new.push(module.into());
        Self(new)
    }

    pub fn contains<T: Into<Module> + Clone>(&self, module: &T) -> bool {
        // When no module is specified, always return "true".
        if self.0.len() == 0 {
            return true;
        }

        self.0.iter().any(|m| m.is_parent(module))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Module(String);

impl From<String> for Module {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for Module {
    fn from(s: &str) -> Self {
        Self(s.into())
    }
}

impl Into<String> for Module {
    fn into(self) -> String {
        self.0
    }
}

impl Module {
    fn is_parent<T: Into<Module> + Clone>(&self, other: &T) -> bool {
        let ref parent = self.0;
        let child = other.clone().into().0;

        if *parent == child {
            return true;
        }

        child.get(..parent.len() + 2) == Some(&format!("{}::", parent))
    }

    fn is_child<T: Into<Module> + Clone>(&self, other: &T) -> bool {
        other.clone().into().is_parent(self)
    }
}
