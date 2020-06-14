// Modules are other .rs source files
mod hello;

// Export functions called by R
pub use hello::string_from_rust;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
