
#[derive(Debug,Copy,Clone)]
pub enum TextureType
{
    Unknown,
    CI4,
    CI8,
    RGBA16,
    RGBA32,
    IA8
}

impl TextureType
{
    //
    /// Given a u8 typecode from the texture header, return the texture type
    //
    pub fn from_u8 (v : u8) -> TextureType 
    {
        return match v {
            0x01 => TextureType::CI4,
            0x02 => TextureType::CI8,
            0x04 => TextureType::RGBA16,
            0x08 => TextureType::RGBA32,
            0x10 => TextureType::IA8,
            _ => TextureType::Unknown
        }
    }
}
