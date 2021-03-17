use crate::datatype::XDataType;
use arrow::array::{Float32Array, Float64Array, Int32Array, Int64Array, UInt32Array};
use arrow::datatypes::DataType;
use rustler::Env;
use rustler::Term;
use rustler::{Encoder, ResourceArc};

pub struct Int32ArrayResource(Int32Array);
pub struct Int64ArrayResource(Int64Array);
pub struct UInt32ArrayResource(UInt32Array);
pub struct Float64ArrayResource(Float64Array);
pub struct Float32ArrayResource(Float32Array);

pub enum ArrayResource {
    Int32(ResourceArc<Int32ArrayResource>),
    Int64(ResourceArc<Int64ArrayResource>),
    UInt32(ResourceArc<UInt32ArrayResource>),
    Float32(ResourceArc<Float32ArrayResource>),
    Float64(ResourceArc<Float64ArrayResource>),
}

impl Encoder for ArrayResource {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        match self {
            ArrayResource::Int32(inner) => inner.encode(env),
            ArrayResource::Int64(inner) => inner.encode(env),
            ArrayResource::UInt32(inner) => inner.encode(env),
            ArrayResource::Float32(inner) => inner.encode(env),
            ArrayResource::Float64(inner) => inner.encode(env),
        }
    }
}

pub enum PrimitiveValue {
    Int32(i32),
    Int64(i64),
    UInt32(u32),
    Float32(f32),
    Float64(f64),
}

pub enum ArrayValues {
    Int32(Vec<Option<i32>>),
    Int64(Vec<Option<i64>>),
    UInt32(Vec<Option<u32>>),
    Float32(Vec<Option<f32>>),
    Float64(Vec<Option<f64>>),
}

impl Encoder for PrimitiveValue {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        match self {
            PrimitiveValue::Int32(v) => v.encode(env),
            PrimitiveValue::Int64(v) => v.encode(env),
            PrimitiveValue::UInt32(v) => v.encode(env),
            PrimitiveValue::Float32(v) => v.encode(env),
            PrimitiveValue::Float64(v) => v.encode(env),
        }
    }
}

impl Encoder for ArrayValues {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        match self {
            ArrayValues::Int32(v) => v.encode(env),
            ArrayValues::Int64(v) => v.encode(env),
            ArrayValues::UInt32(v) => v.encode(env),
            ArrayValues::Float32(v) => v.encode(env),
            ArrayValues::Float64(v) => v.encode(env),
        }
    }
}

impl Int32ArrayResource {
    pub fn new(data: Vec<Option<i32>>) -> Int32ArrayResource {
        Int32ArrayResource(Int32Array::from(data))
    }
}

impl Int64ArrayResource {
    pub fn new(data: Vec<Option<i64>>) -> Int64ArrayResource {
        Int64ArrayResource(Int64Array::from(data))
    }
}

impl UInt32ArrayResource {
    fn new(data: Vec<Option<u32>>) -> UInt32ArrayResource {
        UInt32ArrayResource(UInt32Array::from(data))
    }
}

impl Float64ArrayResource {
    fn new(data: Vec<Option<f64>>) -> Float64ArrayResource {
        Float64ArrayResource(Float64Array::from(data))
    }
}

impl Float32ArrayResource {
    fn new(data: Vec<Option<f32>>) -> Float32ArrayResource {
        Float32ArrayResource(Float32Array::from(data))
    }
}

#[rustler::nif]
fn make_array(a: Term, b: XDataType) -> ArrayResource {
    match &b.0 {
        DataType::Int32 => {
            let values: Vec<Option<i32>> = a.decode().unwrap();
            ArrayResource::Int32(ResourceArc::new(Int32ArrayResource::new(values)))
        }
        DataType::Int64 => {
            let values: Vec<Option<i64>> = a.decode().unwrap();
            ArrayResource::Int64(ResourceArc::new(Int64ArrayResource::new(values)))
        }
        DataType::UInt32 => {
            let values: Vec<Option<u32>> = a.decode().unwrap();
            ArrayResource::UInt32(ResourceArc::new(UInt32ArrayResource::new(values)))
        }
        DataType::Float32 => {
            let values: Vec<Option<f32>> = a.decode().unwrap();
            ArrayResource::Float32(ResourceArc::new(Float32ArrayResource::new(values)))
        }
        DataType::Float64 => {
            let values: Vec<Option<f64>> = a.decode().unwrap();
            ArrayResource::Float64(ResourceArc::new(Float64ArrayResource::new(values)))
        }
        _ => ArrayResource::Int64(ResourceArc::new(Int64ArrayResource::new(vec![]))),
    }
}

#[rustler::nif]
fn to_list(arr: Term, dtype: XDataType) -> ArrayValues {
    match &dtype.0 {
        DataType::Int32 => {
            let array: ResourceArc<Int32ArrayResource> = arr.decode().unwrap();
            ArrayValues::Int32(array.0.into_iter().collect())
        }
        DataType::Int64 => {
            let array: ResourceArc<Int64ArrayResource> = arr.decode().unwrap();
            ArrayValues::Int64(array.0.into_iter().collect())
        }
        DataType::UInt32 => {
            let array: ResourceArc<UInt32ArrayResource> = arr.decode().unwrap();
            ArrayValues::UInt32(array.0.into_iter().collect())
        }
        DataType::Float32 => {
            let array: ResourceArc<Float32ArrayResource> = arr.decode().unwrap();
            ArrayValues::Float32(array.0.into_iter().collect())
        }
        DataType::Float64 => {
            let array: ResourceArc<Float64ArrayResource> = arr.decode().unwrap();
            ArrayValues::Float64(array.0.into_iter().collect())
        }
        _ => ArrayValues::Int64(vec![]),
    }
}

#[rustler::nif]
fn sum(arr: Term, dtype: XDataType) -> PrimitiveValue {
    match &dtype.0 {
        DataType::Int32 => {
            let array: ResourceArc<Int32ArrayResource> = arr.decode().unwrap();
            PrimitiveValue::Int32(arrow::compute::kernels::aggregate::sum(&array.0).unwrap())
        }
        DataType::Int64 => {
            let array: ResourceArc<Int64ArrayResource> = arr.decode().unwrap();
            PrimitiveValue::Int64(arrow::compute::kernels::aggregate::sum(&array.0).unwrap())
        }
        DataType::UInt32 => {
            let array: ResourceArc<UInt32ArrayResource> = arr.decode().unwrap();
            PrimitiveValue::UInt32(arrow::compute::kernels::aggregate::sum(&array.0).unwrap())
        }
        DataType::Float32 => {
            let array: ResourceArc<Float32ArrayResource> = arr.decode().unwrap();
            PrimitiveValue::Float32(arrow::compute::kernels::aggregate::sum(&array.0).unwrap())
        }
        DataType::Float64 => {
            let array: ResourceArc<Float64ArrayResource> = arr.decode().unwrap();
            PrimitiveValue::Float64(arrow::compute::kernels::aggregate::sum(&array.0).unwrap())
        }
        _ => PrimitiveValue::Int32(0),
    }
}

#[rustler::nif]
fn len(arr: Term, dtype: XDataType) -> usize {
    match &dtype.0 {
        DataType::Int32 => {
            let array: ResourceArc<Int32ArrayResource> = arr.decode().unwrap();
            array.0.len()
        }
        DataType::Int64 => {
            let array: ResourceArc<Int64ArrayResource> = arr.decode().unwrap();
            array.0.len()
        }
        DataType::UInt32 => {
            let array: ResourceArc<UInt32ArrayResource> = arr.decode().unwrap();
            array.0.len()
        }
        DataType::Float32 => {
            let array: ResourceArc<Float32ArrayResource> = arr.decode().unwrap();
            array.0.len()
        }
        DataType::Float64 => {
            let array: ResourceArc<Float64ArrayResource> = arr.decode().unwrap();
            array.0.len()
        }
        _ => 0,
    }
}