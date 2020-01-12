// use std::convert::TryFrom;
use crate::binary::BinaryValue;
use crate::data_types::Float;
use crate::init::parse_binary as parse_binary;

const D_F64: f64 = 2.0;
const D16_F64: f64 = 16.0;

const ONE_F64: f64 = 1.0;
const ZERO_F64: f64 = 0.0;

const MIN_DIFF: f64 = 1e-10;

const D_I64: i64 = 2;
const D_U64: u64 = 2u64;

const TETRAD_SIZE: usize = 4;
const BIT_SIZE: usize = 1;

pub struct FloatInitializer {
    exponent_size: usize,
    precision_size: usize,
}

macro_rules! setup_from {
    ($self:expr, $value:expr) => {{
	let data = {
	    let data_bin = $value.get().get();
	    data_bin.as_str().to_owned()
	};
	let sign = if data.starts_with("0") { ONE_F64 } else { -ONE_F64 };
	let mut exponent = parse_binary(&data[1..($self.exponent_size+1)])
	    - D_I64.pow($self.exponent_size as u32 - 1) - 1;
	if exponent > 0 { exponent -= 1; }
	// Debug
	// println!("Exponent: {}", exponent);
	(exponent, sign, data)
    }}
}

// Setuping with integer part of value
macro_rules! get_trunc {
    ($value:expr, $binary:expr, $exponent:expr, $v_size:expr, $size:expr) => {{
	let mut tmp = $value.trunc().abs();
	while tmp > MIN_DIFF {
	    $binary.append(&mut Self::convert_trunc(tmp, $v_size, $size));
	    // Debug
	    // println!("{} : {} : {:?}", tmp, exponent, binary);
	    tmp = (tmp / $v_size).trunc();
	    $exponent += 1;
	}
	// Reversing array
	$binary.reverse();
    }}
}

macro_rules! get_fract {
    ($self:expr, $value:expr, $binary:expr, $exponent:expr,
     $v_size:expr, $size:expr, $Self:ty, $shift:expr) => {{
	 // Getting float part of value if required.
	 let mut tmp = $value.fract().abs();
	 let mut is_zero = $exponent <= D_U64.pow($self.exponent_size as u32 - 1) + $shift;
	 if is_zero { $exponent -= 1 }
	 while $binary.len() <= $self.precision_size + $size + $shift {
	     if (is_zero
		 && $exponent <= D_U64.pow($self.exponent_size as u32 - 1)
		 && (tmp * $v_size) < ONE_F64) {
		 $exponent -= 1;
	     } else {
		 is_zero = false;
		 $binary.append(&mut <$Self>::convert_fract(tmp, $v_size, $size));
	     }
	     // Debug
	     // println!("{} : {} : {:?}", tmp, (tmp * D16_F64), binary);
	     tmp = (tmp * $v_size).fract();
	 }
     }}
}

macro_rules! get_parts {
    ($self:expr, $value:expr, $format:expr) => {
	if $self.exponent_size + $self.precision_size + 1 == $value.size() {	
	    let value_str = {
		let value_bin = $value.get().get();
		value_bin.as_str().to_owned()
	    };
	    Some((
		value_str.starts_with("1"),
		BinaryValue::from(&value_str[1..($self.exponent_size+1)]).unwrap(),
		BinaryValue::new(format!($format, &value_str[($self.exponent_size+1)..])).unwrap())
	    )
	} else {
	    None
	}
    }
}

#[allow(unused_parens)]
impl FloatInitializer{

    pub fn new(
	exponent_size: usize,
	precision_size: usize
    ) -> Self {
	Self {
	    exponent_size,
	    precision_size,
	}
    }

    pub fn get_exp_size(&self) -> usize {
	self.exponent_size
    }

    pub fn get_prec_size(&self) -> usize {
	self.precision_size
    }
    
    pub fn get_f1(
	&self,
	value: f64
    ) -> Float {
	let mut exponent = D_U64.pow(self.exponent_size as u32 - 1);
	let mut binary: Vec<char> = Vec::new();

	get_trunc!(value, binary, exponent, D16_F64, TETRAD_SIZE);
	get_fract!(self, value, binary, exponent, D16_F64, TETRAD_SIZE, Self, 0);
	
	let result: BinaryValue = BinaryValue::new(
	    binary[0..self.precision_size]
		.iter()
		.cloned()
		.collect())
	    .unwrap();
	// println!("LENGTH: {}", 1 + self.exponent_size + result.get().len());
	Float::from(
	    value.is_sign_negative(),
	    format!(
		"{:0width$b}",
		exponent,
		width = self.exponent_size,
	    ),
	    match binary[self.precision_size] {
		'1' => result.inc().get(),
		_ => result.get(),
	    })
    }

    pub fn from_f1(
	&self,
	value: Float
    ) -> Option<f64> {
	if 1 + self.exponent_size + self.precision_size == value.size() {
	    let (exponent, sign, data) = setup_from!(self, value);
	    let mut result = 0f64;
	    for index in 0..((self.precision_size as f64 / TETRAD_SIZE as f64).ceil() as usize) {
		let part = &data[
		    (self.exponent_size + 1 + index * TETRAD_SIZE)..(
			if (index + 1) * TETRAD_SIZE > self.precision_size {
			    value.size()
			} else {
			    self.exponent_size + 1 + (index + 1) * TETRAD_SIZE
			}
		    )
		];
		// Debug
		// println!("Part {}: {}", index, part);
		result += parse_binary(part) as f64 * D16_F64.powi(exponent as i32);
		exponent -= 1;
	    }
	    Some(sign * result)
	} else {
	    None
	}
    }

    pub fn get_f2(
	&self,
	value: f64
    ) -> Float {
	let mut exponent = D_U64.pow(self.exponent_size as u32 - 1) + 1;
	let mut binary: Vec<char> = Vec::new();
	
	get_trunc!(value, binary, exponent, D16_F64, TETRAD_SIZE);
	get_fract!(self, value, binary, exponent, D_F64, BIT_SIZE, Self, 1);

	let result: BinaryValue = BinaryValue::new(
	    binary[1..self.precision_size+1]
		.iter()
		.cloned()
		.collect())
	    .unwrap();
	// println!("LENGTH: {}", 1 + self.exponent_size + result.get().len());
	Float::from(
	    value.is_sign_negative(),
	    format!(
		"{:0width$b}",
		exponent,
		width = self.exponent_size,
	    ),
	    match binary[self.precision_size + 1] {
		'1' => result.inc().get(),
		_ => result.get(),
	    })
    }

    pub fn from_f2(
	&self,
	value: Float
    ) -> Option<f64> {
	if 1 + self.exponent_size + self.precision_size == value.size() {
	    let (exponent, sign, data) = setup_from!(self, value);
	    let mut result = D_F64.powi(exponent as i32);
	    exponent -= 1;
	    for val in data[(self.exponent_size+1)..].chars() {
		// Debug
		// println!("Part {}: {}", index, part);
		result += if val == '1' {ONE_F64} else {ZERO_F64} * D_F64.powi(exponent as i32);
		exponent -= 1;
	    }
	    Some(sign * result)
	} else {
	    None
	}
    }

    pub fn get_parts_f1(&self, value: Float) -> Option<(bool, BinaryValue, BinaryValue)> {
	get_parts!(self, value, "{}")
    }

    pub fn get_parts_f2(&self, value: Float) -> Option<(bool, BinaryValue, BinaryValue)> {
	get_parts!(self, value, "1{}")
    }
    
    fn convert_trunc(tmp: f64, size: f64, width: usize) -> Vec<char> {
	format!("{:0width$b}",
		((tmp / size).fract() * size) as i64,
		width = width)
	    .chars()
	    .rev()
	    .collect()
    }

    fn convert_fract(tmp: f64, size: f64, width: usize) -> Vec<char> {
	format!("{:0width$b}",
		(tmp * size) as i64,
		width = width)
	    .chars()
	    .collect()
    }
}
