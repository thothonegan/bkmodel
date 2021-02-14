use crate::errors::Result;
use crate::errors::FailedSliceError;

use snafu::ResultExt;
use std::convert::TryInto;

//
/// G_SETTIMG: Sets the texture image offset
//
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub struct G_SETTIMG
{
    m_texture_format_flag: u8,
    m_texture_bit_size_flag: u8,
    m_segment_address_of_texture: u32
}

impl G_SETTIMG
{
    pub fn new (cmd : [u8;8]) -> Result<G_SETTIMG>
    {
        // format is: FD [xx] [00] [00] [bb bb bb bb]
        // where xx = fffi i000
        let texture_format_flag = (cmd[1] & 0xe0) >> 5;
        let texture_bit_size_flag = (cmd[1] & 0x18) >> 3;
        let segment_address_of_texture = u32::from_be_bytes(cmd[4..8].try_into().context(FailedSliceError)?);

        return Ok(G_SETTIMG{
            m_texture_format_flag: texture_format_flag,
            m_texture_bit_size_flag: texture_bit_size_flag,
            m_segment_address_of_texture: segment_address_of_texture
        });
    }
    
    pub fn psuedo_code (&self) -> String
    {
        return format!("G_SETTIMG format:{}_{} segAddrOfTex:{:#X}",
            self.p_textureFormatName(),
            self.p_textureBitSize(),
            self.m_segment_address_of_texture
        );
    }

    pub fn p_textureFormatName (&self) -> String {
        return match self.m_texture_format_flag {
            0 => "RGBA".to_string(),
            1 => "YUV".to_string(),
            2 => "CI".to_string(), // index + lookup
            3 => "IA".to_string(), // grayscale + alpha
            4 => "I".to_string(), // grayscale

            _ => "UNK".to_string()
        };
    }

    pub fn p_textureBitSize (&self) -> u8 {
        return match self.m_texture_bit_size_flag {
            0 => 4,
            1 => 8,
            2 => 16,
            3 => 32,
            _ => 0
        }
    }
}