use logfn::logfn;

#[logfn(pre, msg = "", level = "info")]
#[logfn(post, msg = "", level = "error", if = "Result::is_err")]
#[logfn(post, msg = "", level = "info", if = "Result::is_ok")]
fn add(a: usize, b: usize) -> usize {
    a + b
}
