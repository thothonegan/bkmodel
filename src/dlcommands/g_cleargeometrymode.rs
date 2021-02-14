use crate::errors::Result;

//
/// G_CLEARGEOMETRYMODE: Disables certain geometry parameters (ex. lighting, front-/backface culling, Z-buffer).
//
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub struct G_CLEARGEOMETRYMODE
{
}

impl G_CLEARGEOMETRYMODE
{
    pub fn new (_cmd : [u8;8]) -> Result<G_CLEARGEOMETRYMODE>
    {
        // TODO store the flags
        return Ok(G_CLEARGEOMETRYMODE{});
    }
    
    pub fn psuedo_code (&self) -> String
    {
        return format!("G_CLEARGEOMETRYMODE [TODO: flags]");
    }
}