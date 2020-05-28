use logfn::logfn;

#[logfn(pre, msg = "execute add", level = "info")]
#[logfn(post, msg = "executed add", level = "error", if = "Result::is_err")]
#[logfn(post, msg = "executed add", level = "info", if = "Result::is_ok")]
fn add(a: usize, b: usize) -> usize {
    a + b
}
