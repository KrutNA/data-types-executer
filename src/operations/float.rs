use crate::init::FloatInitializer;
use crate::binary::BinaryValue;
use crate::init::parse_binary as parse_binary;
use crate::data_types::Float;

macro_rules! setup {
    ($value:expr, $init:expr, $func:ident) => {
	$init.$func($value).unwrap()
    }
}

pub fn sum_f1(
    init: FloatInitializer,
    value1: Float,
    value2: Float,
    display: bool,
) -> Option<Float> {
    if value1.size() == value2.size()
	&& value1.size() == init.get_exp_size() + init.get_prec_size() + 1
    {
	let (value1_sign, value1_exp, value1_prec) = setup!(value1, init, get_parts_f1);
	let (value2_sign, value2_exp, value2_prec) = setup!(value2, init, get_parts_f1);
	
	let exp_size = init.get_exp_size();
	let prec_size = init.get_prec_size();
	
	let result_exp = value1_exp - value2_exp;
	
    // let value1_exp = parse_binary(&value1_str[1..(exp_size+1)]);
    // let value2_exp = parse_binary(&value2_str[1..(exp_size+1)]);
    
    } else {    
	None
    }
}
