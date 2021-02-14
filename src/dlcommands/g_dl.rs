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
    m_store_return_address: bool,
    m_branch_segmented_address: u32
}

impl G_DL
{
    pub fn new (cmd : [u8;8]) -> Result<G_DL>
    {
        let store_return_address = cmd[1] == 0;
        let branch_segmented_address = u32::from_be_bytes(cmd[4..8].try_into().context(FailedSliceError)?);

        return Ok(G_DL{
            m_store_return_address : store_return_address,
            m_branch_segmented_address : branch_segmented_address
        });
    }
    
    pub fn psuedo_code (&self) -> String
    {
        return format!("G_DL storeReturnAddr:{} branchAddr:{:#X}",
            self.m_store_return_address, self.m_branch_segmented_address
        );
    }
}