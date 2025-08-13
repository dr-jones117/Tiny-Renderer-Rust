use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Debug, Clone)]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T: Copy> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 { x, y, z }
    }
}

impl<T: Add<Output = T> + Copy> Add for &Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, other: Self) -> Vec3<T> {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Sub<Output = T> + Copy> Sub for &Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, other: Self) -> Vec3<T> {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T: Add<Output = T> + Mul<Output = T> + Copy> Mul for &Vec3<T> {
    type Output = T;

    fn mul(self, other: Self) -> T {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }
}

impl<T: Copy> Vec4<T> {
    pub fn new(x: T, y: T, z: T, a: T) -> Vec4<T> {
        Vec4 { x, y, z, w: a }
    }
}

impl<T: Add<Output = T> + Copy> Add for &Vec4<T> {
    type Output = Vec4<T>;

    fn add(self, other: Self) -> Vec4<T> {
        Vec4 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl<T: Sub<Output = T> + Copy> Sub for &Vec4<T> {
    type Output = Vec4<T>;

    fn sub(self, other: Self) -> Vec4<T> {
        Vec4 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl<T: Add<Output = T> + Mul<Output = T> + Copy> Mul for &Vec4<T> {
    type Output = T;

    fn mul(self, other: Self) -> T {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z) + (self.w * other.w)
    }
}
