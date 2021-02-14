use crate::errors::Result;
use crate::errors::FailedSliceError;

use snafu::ResultExt;
use std::convert::TryInto;

//
/// G_DL: Starts a display list
//
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub struct G_DL
{
    m_storeReturnAddress: bool,
    m_branchSegmentedAddress: u32
}

impl G_DL
{
    pub fn new (cmd : [u8;8]) -> Result<G_DL>
    {
        let storeReturnAddress = cmd[1] == 0;
        let branchSegmentedAddress = u32::from_be_bytes(cmd[4..8].try_into().context(FailedSliceError)?);

        return Ok(G_DL{
            m_storeReturnAddress : storeReturnAddress,
            m_branchSegmentedAddress : branchSegmentedAddress
        });
    }
    
    pub fn psuedo_code (&self) -> String
    {
        return format!("G_DL storeReturnAddr:{} branchAddr:{:#X}",
            self.m_storeReturnAddress, self.m_branchSegmentedAddress
        );
    }
}