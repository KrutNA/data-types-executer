use std::ops::{Sub, Add, Neg};
#[derive(Debug, PartialEq, Clone)]
pub struct BinaryValue {
    value: String,
    moves: Vec<bool>,
}

impl BinaryValue {

    pub fn new(value: String) -> Result<Self, String> {
	if !value.chars().all(|v| v == '0' || v == '1') {
	    Result::Err(String::from("String is not a binary string"))
	} else {
	    Ok(Self {
		value,
		moves: Vec::new(),
	    })
	}
    }

    
    pub fn from(value: &str) -> Result<Self, String> {
	if !value.chars().all(|v| v == '0' || v == '1') {
	    Result::Err(String::from("String is not a binary string"))
	} else {
	    Ok(Self {
		value: String::from(value),
		moves: Vec::new(),
	    })
	}
    }

    pub fn new_with_moves(value: String, moves: Vec<bool>) -> Result<Self, String> {
	if !value.chars().all(|v| v == '0' || v == '1') {
	    Result::Err(String::from("String is not a binary string"))
	} else {
	    Ok(Self {
		value,
		moves,
	    })
	}	
    }
    
    pub fn size(&self) -> usize {
	self.value.len()
    }

    pub fn get(&self) -> String {
	self.value.clone()
    }
    
    pub fn get_nth(&self, n: usize) -> Option<char> {
	self.value.chars().nth(n)
    }

    pub fn get_moves(&self) -> Option<Vec<bool>> {
	if self.moves.len() == 0 {
	    None
	} else {
	    Some(self.moves)
	}
    }
    
    fn bit_sum(v1: char, v2: char, memory: &mut char) -> char {
	match (v1, v2) {
	    ('1', '1') => {
		*memory = '1';
		'0'
	    },
	    ('0', '0') => '0',
	    _ => '1',
	}
    }

    pub fn inc(&self) -> BinaryValue {
	self.add(&BinaryValue::new(
	    format!("{:0width$}",
		    1,
		    width = self.value.len()))
		  .unwrap())
    }
}

impl Add for &BinaryValue {
    type Output = BinaryValue;
    
    fn add(self, other: Self) -> BinaryValue {
    	let mut memory = '0';
	let mut moves: Vec<bool> = Vec::new(); 
    	let mut chars: Vec<char> = Vec::new();
    	for i in (0..self.value.len()).rev() {
    	    let tmp = memory;
    	    let res = BinaryValue::bit_sum(self.get_nth(i).unwrap(),
    					   other.get_nth(i).unwrap(),
    					   &mut memory);
    	    chars.push(BinaryValue::bit_sum(res, tmp, &mut memory));
	    moves.push(if memory == '1' { true } else { false });
    	}
	moves.reverse();
    	BinaryValue::new_with_moves(
	    chars.iter().rev().collect(),
	    moves
	).unwrap()	
    }
}

impl Sub for &BinaryValue {
    type Output = BinaryValue;

    fn sub(self, other: Self) -> BinaryValue {
	self.add(&other.neg())
    }
}

impl Neg for &BinaryValue {
    type Output = BinaryValue;

    fn neg(self) -> BinaryValue {
	let value = BinaryValue::new(
	    self.value
		.chars()
		.map(|v| if v == '1' {'0'} else {'1'})
		.collect()).unwrap();
	value.inc()
    }
}
