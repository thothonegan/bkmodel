use crate::errors::Result;

//
/// G_SetOtherMode_L: Sets the lower half of the RDP Other modes
//
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub struct G_SetOtherMode_L
{}

impl G_SetOtherMode_L
{
    pub fn new (_cmd : [u8;8]) -> Result<G_SetOtherMode_L>
    {
        // TODO: flags. They're stored weirdly.
        return Ok(G_SetOtherMode_L{});
    }
    
    pub fn psuedo_code (&self) -> String
    {
        return format!("G_SetOtherMode_L [TODO:flags]");
    }
}