#[derive(Debug, PartialEq)]
pub enum MatrixifyErr {
    NonSquareMatrix,
    NonThirdMatrix,
    ZeroDeterminant,
    UnknownDeterminant,
    InappropriateSizes,
    InvalidIndex,
    DivByVector,
    NotAVector,
    ZeroDivision,
}

#[derive(Debug, PartialEq)]
pub enum MatrixType {
    Identity,
    NegIdentity,
    RevIdentity,
    NegRevIdentity,
    Cross,
    NegCross,
    Rhomb,
    NegRhomb,
    Ones,
}
