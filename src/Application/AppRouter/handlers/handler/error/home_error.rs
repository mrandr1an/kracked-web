pub enum HomeError
{
    CouldntLoadJS,
    CouldntLoadHTML,
    CouldntLoadCSS
}

impl std::fmt::Display for HomeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HomeError::CouldntLoadJS => write!(f, "CouldntLoadJS"),
            HomeError::CouldntLoadHTML => write!(f, "CouldntLoadHTML"),
            HomeError::CouldntLoadCSS => write!(f, "CouldntLoadCSS"),
        }
    }
}
