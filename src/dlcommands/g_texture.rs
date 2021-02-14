use crate::errors::Result;
use crate::errors::FailedSliceError;

use snafu::ResultExt;
use std::convert::TryInto;

//
/// G_TEXTURE: Sets the texture scaling factor
//
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub struct G_TEXTURE
{
    m_enable_or_disable_tile_descriptor: bool,
    m_scaling_factor_s: u16,
    m_scaling_factor_t: u16,
    m_max_mipmap_levels_other_than_first: u8,
    m_tile_descriptor_to_enable_disable: u8
}

impl G_TEXTURE
{
    pub fn new (_cmd : [u8;8]) -> Result<G_TEXTURE>
    {
        // [1] is 0
        // 2 is [00LL Lddd] where L is mipmap levels (-1), ddd is tile descripter
        let max_mipmap_levels_other_than_first = (_cmd[2] & 0x38) >> 3;
        let tile_descriptor_to_enable_disable = _cmd[2] & 0x07;
        let enable_or_disable_tile_descriptor = _cmd[3] != 0;
        let scaling_factor_s = u16::from_be_bytes(_cmd[4..6].try_into().context(FailedSliceError)?);
        let scaling_factor_t = u16::from_be_bytes(_cmd[6..8].try_into().context(FailedSliceError)?);

        return Ok(G_TEXTURE{
            m_enable_or_disable_tile_descriptor : enable_or_disable_tile_descriptor,
            m_scaling_factor_s: scaling_factor_s,
            m_scaling_factor_t: scaling_factor_t,
            m_max_mipmap_levels_other_than_first: max_mipmap_levels_other_than_first,
            m_tile_descriptor_to_enable_disable : tile_descriptor_to_enable_disable
        });
    }
    
    pub fn psuedo_code (&self) -> String
    {
        if self.m_enable_or_disable_tile_descriptor {
            return format!("G_TEXTURE enableTileDescriptor:{} scaleS:{} scaleT:{} mipmapLevels:{}",
                self.m_tile_descriptor_to_enable_disable, self.m_scaling_factor_s, 
                self.m_scaling_factor_t, self.m_max_mipmap_levels_other_than_first+1
            );
        } else {
            return format!("G_TEXTURE disableTileDescriptor:{} scaleS:{} scaleT:{} mipmapLevels:{}",
                self.m_tile_descriptor_to_enable_disable, self.m_scaling_factor_s, 
                self.m_scaling_factor_t, self.m_max_mipmap_levels_other_than_first+1
            )
        }

    }
}