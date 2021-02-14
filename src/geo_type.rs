//
/// A geo type (geometry?)
//
#[derive(Copy, Clone, Debug)]
pub enum GeoType
{
    //
    /// Unknown/Invalid geotype
    //
    Unknown,

    //
    /// Normal type
    //
    Normal,

    //
    /// Trilinear Mipmapping (RGBA16)
    //
    TrilinearMipMapping,

    //
    /// Environmental mapping
    //
    EnvMapping
}

impl GeoType
{
    //
    /// Given a U16 (from the model header), return the geo type equivilant
    //
    pub fn from_u16 (val : u16) -> GeoType
    {
        return match val
        {
            0x0000 => GeoType::Normal,
            0x0002 => GeoType::TrilinearMipMapping,
            0x0004 => GeoType::EnvMapping,
            _ => GeoType::Unknown
        }
    }
}