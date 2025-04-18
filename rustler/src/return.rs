use crate::codegen_runtime::{NifReturnable, NifReturned};
use crate::error::Error;
use crate::{Env, Term};

pub enum Return<'a> {
    Term(Term<'a>),
    Error(Error),
}

unsafe impl NifReturnable for Return<'_> {
    unsafe fn into_returned(self, env: Env) -> NifReturned {
        match self {
            Return::Term(inner) => NifReturned::Term(inner.as_c_arg()),
            Return::Error(inner) => inner.into_returned(env),
        }
    }
}
