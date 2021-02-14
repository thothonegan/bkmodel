use crate::errors::Result;
use crate::errors::FailedSliceError;

use snafu::ResultExt;
use std::convert::TryInto;

//
/// G_VTX: Fills the vertex buffer with vertex information.
//
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub struct G_VTX
{
    m_vertex_start: u8, // *2
    m_count: u8, // amount of verts to write
    m_data_length: u16,  // length of vertex data to right
    m_segmented_address: u32
}

impl G_VTX
{
    pub fn new (cmd : [u8;8]) -> Result<G_VTX>
    {
        let vertex_start = cmd[1];
        let count = cmd[2] & 0xFC;
        let data_length = ((cmd[3] & 0x03) | cmd[4]) as u16;
        let segmented_address = u32::from_be_bytes(cmd[4..8].try_into().context(FailedSliceError)?);

        return Ok(G_VTX{
            m_vertex_start: vertex_start,
            m_count: count,
            m_data_length: data_length,
            m_segmented_address: segmented_address
        });
    }
    
    pub fn psuedo_code (&self) -> String
    {
        return format!("G_VTX start:{} count:{} dataLen:{:#X} segAddr:{:#X}",
            self.m_vertex_start/2, self.m_count, self.m_data_length, self.m_segmented_address    
        );
    }
}
