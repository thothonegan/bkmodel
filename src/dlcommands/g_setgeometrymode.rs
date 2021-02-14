use crate::errors::Result;

//
/// G_SETGEOMETRYMODE: Enables certain geometry parameters (ex. lighting, front-/backface culling, Z-buffer).
//
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub struct G_SETGEOMETRYMODE
{
}

impl G_SETGEOMETRYMODE
{
    pub fn new (_cmd : [u8;8]) -> Result<G_SETGEOMETRYMODE>
    {
        // TODO store the flags
        return Ok(G_SETGEOMETRYMODE{});
    }
    
    pub fn psuedo_code (&self) -> String
    {
        return format!("G_SETGEOMETRYMODE [TODO: flags]");
    }
}