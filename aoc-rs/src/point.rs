#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self
    }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self
    }
}

impl std::ops::Add for Point3D {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self
    }
}

impl std::ops::Sub for Point3D {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self
    }
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn shift(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn manhattan_distance(&self, other: &Point) -> i32 {
        (self.x() - other.x()).abs() + (self.y() - other.y()).abs()
    }
}

impl<T: TryInto<i32>> From<(T, T)> for Point
where
    T::Error: std::fmt::Debug,
{
    fn from((x, y): (T, T)) -> Self {
        Self::new(x.try_into().unwrap(), y.try_into().unwrap())
    }
}

impl Point3D {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn shift(&mut self, x: i32, y: i32, z: i32) {
        self.x += x;
        self.y += y;
        self.z += z;
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn z(&self) -> i32 {
        self.z
    }

    pub fn manhattan_distance(&self, other: &Point3D) -> i32 {
        (self.x() - other.x()).abs() + (self.y() - other.y()).abs() + (self.z() - other.z()).abs()
    }
}

impl<T: TryInto<i32>> From<(T, T, T)> for Point3D
where
    T::Error: std::fmt::Debug,
{
    fn from((x, y, z): (T, T, T)) -> Self {
        Self::new(
            x.try_into().unwrap(),
            y.try_into().unwrap(),
            z.try_into().unwrap(),
        )
    }
}
