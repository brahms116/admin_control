use std::fmt;

#[derive(Debug)]
pub struct TypeErr {
    pub fieldname: String,
    pub recieved_type: String,
    pub expected_type: String,
}

impl fmt::Display for TypeErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"For the field {}, we were expecting type {}, but found {} instead.\
        Try using the correct type", self.fieldname, self.expected_type, self.recieved_type)
    }
}

impl std::error::Error for TypeErr {}
