use crate::errors::Result;

//
/// G_SPNOOP: Stalls the signal processor (the RSP) and the RDP.
/// Only for debugging
//
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub struct G_SPNOOP
{}

impl G_SPNOOP
{
    pub fn new (_cmd : [u8;8]) -> Result<G_SPNOOP>
    {
        return Ok(G_SPNOOP{});
    }
    
    pub fn psuedo_code (&self) -> String
    {
        return format!("G_SPNOOP");
    }
}