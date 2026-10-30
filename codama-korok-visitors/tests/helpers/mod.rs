use codama_nodes::Path;

pub fn path(value: &str) -> Path {
    value.parse().unwrap()
}
