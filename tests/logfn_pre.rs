use logfn::logfn;

#[logfn(pre, msg = "execute add", level = "info")]
#[logfn(post, msg = "executed add", level = "error", if = "Result::is_err")]
#[logfn(post, msg = "executed add", level = "info", if = "Result::is_ok")]
fn add(a: usize, b: usize) -> usize {
    a + b
}

#[logfn(pre, msg = "execute add", level = "info")]
#[logfn(post, msg = "executed add", level = "error", if = "Result::is_err")]
fn generics<T>(a: T) -> T
where
    T: std::fmt::Debug,
{
    a
}

#[logfn(pre, msg = "execute async_fn", level = "info")]
#[logfn(
    post,
    msg = "executed async_fn",
    level = "error",
    if = "Result::is_err"
)]
async fn async_fn(a: usize) -> usize {
    async { a }.await
}
