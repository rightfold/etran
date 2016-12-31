#[derive(Clone, Debug)]
pub enum Value {
    Level(Level),
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Level {
    Debug,
    Info,
    Notice,
    Warning,
    Error,
    Critical,
    Alert,
}
