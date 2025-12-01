pub trait PointeeSized {}

pub trait PointeeSizedTr: PointeeSized {}

impl<T: ?Sized> PointeeSizedTr for T {}
