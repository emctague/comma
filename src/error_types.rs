/// `err_type!` generates an error struct with the given visibility, structure name, and displayed
/// expression string.
#[macro_use]
macro_rules! err_type {
    ($vis: ident, $name:ident, $message:expr) => {

        #[derive(Debug, Clone)]
        $vis struct $name;

        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
                write!(f, $message)
            }
        }

        impl std::error::Error for $name {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                None
            }
        }
    };
}
