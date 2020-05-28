use logfn::logfn;

#[logfn(pre, msg = "execute add", level = "info")]
#[logfn(post, msg = "executed add", level = "error", if = "Option::is_none")]
#[logfn(post, msg = "executed add", level = "info", if = "Option::is_some")]
fn add(a: usize, b: usize) -> Option<usize> {
    a.checked_add(b)
}

#[logfn(pre, msg = "execute add", level = "info")]
fn generics<T>(a: T) -> T
where
    T: std::fmt::Debug,
{
    a
}

#[logfn(pre, msg = "execute async_fn", level = "info")]
#[logfn(post, msg = "executed async_fn", level = "error")]
async fn async_fn(a: usize) -> usize {
    async { a }.await
}
