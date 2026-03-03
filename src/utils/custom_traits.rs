pub trait EnumerableEnum: Sized {
    fn variants_slice() -> Vec<Self>;
}
