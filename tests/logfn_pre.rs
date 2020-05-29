use logfn::logfn;

#[logfn(Pre, Info, "execute add")]
#[logfn(Post, Error, "failed to add: {:?}", if = "Option::is_none")]
#[logfn(Post, Info, "executed add", if = "Option::is_some")]
fn add(a: usize, b: usize) -> Option<usize> {
    a.checked_add(b)
}

#[logfn(Pre, Info, "execute add")]
fn generics<T>(a: T) -> T
where
    T: std::fmt::Debug,
{
    a
}

#[logfn(Pre, Info, "execute async_fn")]
#[logfn(Post, Info, "executed async_fn")]
async fn async_fn(a: usize) -> usize {
    async { a }.await
}
