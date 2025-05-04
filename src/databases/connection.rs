pub trait Database {
    fn validate(&self) -> bool;
    fn dump(&self, execluded_entity: Vec<&str>, dum_path: &str);
    fn restor(&self, dum_path: &str);
}
