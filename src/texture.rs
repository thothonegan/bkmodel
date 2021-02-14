use crate::texture_type::TextureType;

// proxy for the texture - stores a bit of the header info
pub struct Texture
{
    //
    /// The index of the texture
    //
    m_texture_index : u8,

    //
    /// The segment address from the texture data start
    //
    m_segment_address : u32,

    //
    /// The absolute address within the loaded model
    //
    m_absolute_addres : u32,

    //
    /// The texture type
    //
    m_texture_type : TextureType,

    //
    /// The width on the pixel grid
    //
    m_width : u8,

    //
    /// The height on the pixel grid
    //
    m_height : u8
}

impl Texture
{
    //
    /// Create a new texture
    //
    pub fn new (index : u8, segment_address : u32,
        absolute_address : u32,
        texture_type: TextureType, width: u8, height: u8) -> Texture
    {
        return Texture {
            m_texture_index: index,
            m_segment_address: segment_address,
            m_absolute_addres : absolute_address,
            m_texture_type : texture_type,
            m_width: width,
            m_height: height
        };
    }

    // --- public properties

    pub fn index (&self) -> u8 {
        return self.m_texture_index;
    }

    pub fn segment_address (&self) -> u32 {
        return self.m_segment_address;
    }

    pub fn absolute_address (&self) -> u32 {
        return self.m_absolute_addres;
    }
    
    pub fn texture_type(&self) -> TextureType {
        return self.m_texture_type;
    }

    pub fn width (&self) -> u8 {
        return self.m_width;
    }

    pub fn height (&self) -> u8 {
        return self.m_height;
    }
}