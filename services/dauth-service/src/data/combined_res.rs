use auth_vector::types::{Res, ResStar, XResHash, XResStarHash};
#[derive(Debug, Clone, Copy)]
pub enum ResKind {
    Res(Res),
    ResStar(ResStar),
}

#[derive(Debug, Clone, Copy)]
pub enum XResHashKind {
    XResHash(XResHash),
    XResStarHash(XResStarHash),
}
