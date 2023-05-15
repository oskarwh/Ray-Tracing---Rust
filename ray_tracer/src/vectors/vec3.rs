use std::ops::{Add, Sub, Mul};

#[derive(Copy, Clone)]
pub struct Vec3
{
    e: [f32; 3]
}

impl Vec3 
{
    /**
     * Returns new vector with (x,y,z)
     */
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 
    {
        Vec3 {
            e: [x,y,z],
        }
    }

    pub fn x(&self) -> f32
    {
        self.e[0]
    }

    pub fn y(&self) -> f32
    {
        self.e[1]
    }

    pub fn z(&self) -> f32
    {
        self.e[2]
    }

    
    /**
     * Multiplies given vector with given constant
     */
    pub fn const_mul(&self, t: f32) -> Vec3
    {
        Vec3 {
            e: 
            [self.e[0]*t,
            self.e[1]*t,
            self.e[2]*t]
        }
    }

    /**
     * Divides given vector with given constant
     */
    pub fn const_div(&self, t: f32) -> Vec3
    {
        Vec3 {
            e: 
            [(1.0/t) *self.e[0],
            (1.0/t) *self.e[1],
            (1.0/t) *self.e[2]]
        }
    }


    /**
     * Returns the squared lenght of the vector
     */
    pub fn length_squared(&self) -> f32
    {
        (self.e[0]*self.e[0]) + (self.e[1]*self.e[1]) + (self.e[2]*self.e[2])
    }

    /**
     * Returns the lenght of the vector
     */
    pub fn length(&self) -> f32
    {
        self.length_squared().sqrt()
    }

    /**
     * Returns a unit vector from the given vector
     */
    pub fn unit_vector(&self) -> Vec3
    {
        self.const_div(self.length())
    }
}

// Overload "+" operater for Vec3
impl Add for Vec3
{
    type Output = Self;

    fn add(self, other: Self) -> Self
    {
        Self {
            e: 
            [self.e[0] + other.e[0],
            self.e[1] + other.e[1],
            self.e[2] + other.e[2]]
        }
    }
}

// Overload "-" operater for Vec3
impl Sub for Vec3
{
    type Output = Self;

    fn sub(self, other: Self) -> Self
    {
        Self {
            e: 
            [self.e[0] - other.e[0],
            self.e[1] - other.e[1],
            self.e[2] - other.e[2]]
        }
    }
}

// Overload "*" operater for Vec3
impl Mul for Vec3
{
    type Output = Self;

    fn mul(self, other: Self) -> Self
    {
        Self {
            e: 
            [self.e[0] * other.e[0],
            self.e[1] * other.e[1],
            self.e[2] * other.e[2]]
        }
    }
}

/**
 * Returns the dot product of given vectors
 */
pub fn dot(v: &Vec3, other: &Vec3) -> f32
{
    (v.e[0]*other.e[0])+(v.e[1]*other.e[1])+(v.e[2]*other.e[2])
}

/**
 * Returns the cross product of given vectors
 */
pub fn cross(v: &Vec3, other: &Vec3) -> Vec3
{
    Vec3{
        e:[
            (v.e[1]*other.e[2])-(v.e[2]*other.e[1]),
            (v.e[2]*other.e[0])-(v.e[0]*other.e[2]),
            (v.e[0]*other.e[1])-(v.e[1]*other.e[0]),
        ]
    }
    
}

// Alias
pub type Point3 = Vec3;
pub type Color = Vec3;