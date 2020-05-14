fn main() {
    fn ensure<T: mem_markers::FixedLayout>() {}
    ensure::<&str>();
}
