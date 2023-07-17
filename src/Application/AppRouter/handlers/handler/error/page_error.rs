pub enum PageError
{
    CouldntLoadJS,
    CouldntLoadHTML,
    CouldntLoadCSS
}

impl std::fmt::Display for PageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PageError::CouldntLoadJS => write!(f, "CouldntLoadJS"),
            PageError::CouldntLoadHTML => write!(f, "CouldntLoadHTML"),
            PageError::CouldntLoadCSS => write!(f, "CouldntLoadCSS"),
        }
    }
}
