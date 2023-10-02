use rand::Rng;
const NOISE: f64 = 3.0;

pub struct Frame {
    sequence_number: Option<usize>,
    acknowledgement_number: Option<usize>,
    lost: bool,
    information: String,
}


impl Frame {
    pub fn new(sequence_number: usize, information: String) -> Self {
       Frame {
        sequence_number: Option::from(sequence_number),
        acknowledgement_number: Option::from(None),
        lost: false,
        information,
       } 
    }

    pub fn get_ack(&self) -> Self {
        Frame { sequence_number: Option::from(None) , acknowledgement_number: Option::from(self.sequence_number.unwrap() + 1), lost: false, information: String::from("") }
    }

    pub fn noise(&mut self) {
        let mut rng = rand::thread_rng();
        self.lost = rng.gen_bool(1 as f64/NOISE);
    }
    
    pub unsafe fn any_as_u8_slice<T: Sized>(&self) -> &[u8] {
    ::core::slice::from_raw_parts(
            (self as *const Frame) as *const u8,
            ::core::mem::size_of::<Frame>(),
        )
    }

    pub unsafe fn u8_slice_as_any<T: Sized>(bytes: &[u8]) -> Frame {
        ::core::slice::from_raw_parts(bytes.as_ptr() as *const Frame, 1)[0]
    }

    pub fn get_sequence_number(&self) -> Result<usize, String> {
        match self.sequence_number.is_none() {
            true => Err("this is an acknowledgement!".to_string()),
            false => Ok(self.sequence_number.unwrap()),
        }
    }
}



