use crate::binary::BinaryValue;

pub struct Float {
    value: BinaryValue,
}

pub struct Integer {
    value: BinaryValue,
}

impl Float {

    pub fn new(
	value: BinaryValue,
    ) -> Self {
	Self {
	    value,
	}
    }
    
    pub fn from(
	sign: bool,
	exponent: String,
	precision: String,
    ) -> Self {
	Self {
	    value: BinaryValue::new(format!(
		"{}{}{}",
		if sign { '1' } else { '0' },
		exponent,
		precision,
	    )).unwrap(),
	}
    }

    pub fn get(&self) -> BinaryValue {
	self.value.clone()
    }

    pub fn size(&self) -> usize {
	self.value.size()
    }
}

impl Integer {
    
    pub fn new(
	value: BinaryValue,
    ) -> Self {
	Self {
	    value,
	}
    }

    pub fn get(&self) -> BinaryValue {
	self.value.clone()
    }
}
