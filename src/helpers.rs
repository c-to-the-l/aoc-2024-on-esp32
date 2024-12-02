use alloc::string::String;
use defmt::error;
use picoserve::io::Read;

use crate::error::{AerError, IntoAer};

const PREFIX_LEN: usize = "message=".len();

/// Helper function for reading input from a POST request. 
/// 
/// Expects that it will have a specific prefix (`message=`), because that's how it 
/// arrives when using a "text/plain" post method.
/// 
/// More complex post methods are a bit too heavy for our little microcontroller.
pub async fn read_input<R: Read>(r: &mut R, size_hint: usize) -> crate::Result<String> {
    let mut skip_message: [u8; PREFIX_LEN] = Default::default();
    if let Err(e) =  r.read_exact(&mut skip_message).await {
        error!("Error decoding input: {:?}", defmt::Debug2Format(&e));
        return Err(AerError::MissingMessage)
    }


    let mut input_buf = alloc::vec![0u8; size_hint - PREFIX_LEN];
    let mut read_count: usize = 0;

    loop {
        let read_size = r.read(&mut input_buf[read_count..]).await.into_aer()?;
        if read_size == 0 {
            break;
        }
        read_count += read_size;
        if read_count == size_hint {
            break;
        }
    }
    Ok(String::from_utf8(input_buf)?)
}