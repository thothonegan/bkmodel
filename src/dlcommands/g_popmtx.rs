use crate::errors::Result;
use crate::errors::FailedSliceError;

use snafu::ResultExt;
use std::convert::TryInto;

//
/// G_POPMTX: Pops the given number of matricies from the MVM
//
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub struct G_POPMTX
{
    //
    /// The number to pop off the MVM
    //
    m_count : u32
}

impl G_POPMTX
{
    pub fn new (_cmd : [u8;8]) -> Result<G_POPMTX>
    {
        // BD 39 00 02 aa aa aa aa
        let count = u32::from_be_bytes(_cmd[4..8].try_into().context(FailedSliceError)?);
        return Ok(G_POPMTX{
            m_count: count
        });
    }
    
    pub fn psuedo_code (&self) -> String
    {
        return format!("G_POPMTX count:{}", self.m_count);
    }
}