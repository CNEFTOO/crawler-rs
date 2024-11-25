pub trait Filter {
    fn do_filter(&self) -> bool;
}

pub struct SimpleFilter;

impl SimpleFilter {
    pub fn new(host_limit: &str) -> SimpleFilter {
        Self
    }
}

impl Filter for SimpleFilter {
    fn do_filter(&self) -> bool {
        false
    }
}