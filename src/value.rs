use indexmap::IndexMap;

#[derive(Debug)]
pub enum Value {
    I128(i128),
    U128(u128),
    I64(i64),
    U64(u64),
    I32(i32),
    U32(u32),
    I16(i16),
    U16(u16),
    I8(i8),
    U8(u8),
    F64(f64),
    F32(f32),
    Bool(bool),
    String(String),
    Char(char),
    Vec(Vec<Value>),
    Object(IndexMap<String, Value>),
}

#[allow(dead_code)]
impl Value {
    pub fn is_i128(&self) -> bool {
        matches!(self, Value::I128(_))
    }
    pub fn is_u128(&self) -> bool {
        matches!(self, Value::U128(_))
    }
    pub fn is_i64(&self) -> bool {
        matches!(self, Value::I64(_))
    }
    pub fn is_u64(&self) -> bool {
        matches!(self, Value::U64(_))
    }
    pub fn is_i32(&self) -> bool {
        matches!(self, Value::I32(_))
    }
    pub fn is_u32(&self) -> bool {
        matches!(self, Value::U32(_))
    }
    pub fn is_i16(&self) -> bool {
        matches!(self, Value::I16(_))
    }
    pub fn is_u16(&self) -> bool {
        matches!(self, Value::U16(_))
    }
    pub fn is_i8(&self) -> bool {
        matches!(self, Value::I8(_))
    }
    pub fn is_u8(&self) -> bool {
        matches!(self, Value::U8(_))
    }
    pub fn is_f64(&self) -> bool {
        matches!(self, Value::F64(_))
    }
    pub fn is_f32(&self) -> bool {
        matches!(self, Value::F32(_))
    }
    pub fn is_bool(&self) -> bool {
        matches!(self, Value::Bool(_))
    }
    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }
    pub fn is_char(&self) -> bool {
        matches!(self, Value::Char(_))
    }
    pub fn is_vec(&self) -> bool {
        matches!(self, Value::Vec(_))
    }
    pub fn is_object(&self) -> bool {
        matches!(self, Value::Object(_))
    }
    pub fn as_i128(&self) -> Option<i128> {
        match self {
            Value::I128(value) => Some(*value),
            _ => None,
        }
    }
    pub fn as_u128(&self) -> Option<u128> {
        match self {
            Value::U128(value) => Some(*value),
            _ => None,
        }
    }
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Value::I64(value) => Some(*value),
            _ => None,
        }
    }
    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Value::U64(value) => Some(*value),
            _ => None,
        }
    }
    pub fn as_i32(&self) -> Option<i32> {
        match self {
            Value::I32(value) => Some(*value),
            _ => None,
        }
    }
    pub fn as_u32(&self) -> Option<u32> {
        match self {
            Value::U32(value) => Some(*value),
            _ => None,
        }
    }
    pub fn as_i16(&self) -> Option<i16> {
        match self {
            Value::I16(value) => Some(*value),
            _ => None,
        }
    }
    pub fn as_u16(&self) -> Option<u16> {
        match self {
            Value::U16(value) => Some(*value),
            _ => None,
        }
    }
    pub fn as_i8(&self) -> Option<i8> {
        match self {
            Value::I8(value) => Some(*value),
            _ => None,
        }
    }
    pub fn as_u8(&self) -> Option<u8> {
        match self {
            Value::U8(value) => Some(*value),
            _ => None,
        }
    }
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::F64(value) => Some(*value),
            _ => None,
        }
    }
    pub fn as_f32(&self) -> Option<f32> {
        match self {
            Value::F32(value) => Some(*value),
            _ => None,
        }
    }
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(value) => Some(*value),
            _ => None,
        }
    }
    pub fn as_string(&self) -> Option<&String> {
        match self {
            Value::String(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_char(&self) -> Option<char> {
        match self {
            Value::Char(value) => Some(*value),
            _ => None,
        }
    }
    pub fn as_vec(&self) -> Option<&Vec<Value>> {
        match self {
            Value::Vec(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_object(&self) -> Option<&IndexMap<String, Value>> {
        match self {
            Value::Object(value) => Some(value),
            _ => None,
        }
    }
}

impl From<i128> for Value {
    fn from(value: i128) -> Self {
        Value::I128(value)
    }
}

impl From<u128> for Value {
    fn from(value: u128) -> Self {
        Value::U128(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::I64(value)
    }
}

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        Value::U64(value)
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::I32(value)
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Value::U32(value)
    }
}

impl From<i16> for Value {
    fn from(value: i16) -> Self {
        Value::I16(value)
    }
}

impl From<u16> for Value {
    fn from(value: u16) -> Self {
        Value::U16(value)
    }
}

impl From<i8> for Value {
    fn from(value: i8) -> Self {
        Value::I8(value)
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Value::U8(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::F64(value)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Value::F32(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl From<char> for Value {
    fn from(value: char) -> Self {
        Value::Char(value)
    }
}

impl From<Vec<Value>> for Value {
    fn from(value: Vec<Value>) -> Self {
        Value::Vec(value)
    }
}

impl From<IndexMap<String, Value>> for Value {
    fn from(value: IndexMap<String, Value>) -> Self {
        Value::Object(value)
    }
}
