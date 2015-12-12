

struct SkipErr<T>(Option<T>);

impl<T: fmt::Debug> fmt::Debug for SkipErr<T> where Option<T>: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: fmt::Display> fmt::Display for SkipErr<T> where Option<T>: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: serde::Deserialize> serde::Deserialize for SkipErr<T> {
    fn deserialize<D: serde::Deserializer>(de: &mut D) -> StdResult<SkipErr<T>, D::Error> {
        serde::Deserialize::deserialize(de).map(Some).or_else(|_| Ok(None)).map(SkipErr)
    }
}

impl<T> Into<Option<T>> for SkipErr<T> {
    fn into(self) -> Option<T> {
        self.0
    }
}

impl<T> From<Option<T>> for SkipErr<T> {
    fn from(value: Option<T>) -> SkipErr<T> {
        SkipErr(value)
    }
}

impl<T> Deref for SkipErr<T> {
    type Target = Option<T>;
    fn deref(&self) -> &Option<T> {
        &self.0
    }
}

impl<T> DerefMut for SkipErr<T> {
    fn deref_mut(&mut self) -> &mut Option<T> {
        &mut self.0
    }
}

