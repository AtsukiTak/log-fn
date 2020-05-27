use logfn::logfn;

#[logfn(pre, post)]
fn add(a: usize, b: usize) -> usize {
    a + b
}
