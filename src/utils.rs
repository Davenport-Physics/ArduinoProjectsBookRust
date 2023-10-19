
use half::f16;
use ufmt::{uDisplay, derive::uDebug};

const SIGN_BIT: u16 = 0;
const EXPONENT: u16 = 0b1111;
const FIRST_SIX: u16 = (SIGN_BIT << 15) | (EXPONENT << 10);
const LAST_TEN: u16  = 0b0000001111111111;

pub struct F16Wrapper(f16);

pub trait ByteConverter {
	fn to_f16(self) -> F16Wrapper;
}

impl ByteConverter for u16 {

	fn to_f16(self) -> F16Wrapper {		
		let fraction: u16 = self & LAST_TEN;
		F16Wrapper(f16::from_bits(FIRST_SIX | fraction))
	}
	
}

impl uDisplay for F16Wrapper {

	fn fmt<W>(&self, _: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
	    where
	        W: ufmt::uWrite + ?Sized 
	{
	    todo!();
	}
}