use crate::errors::Result;

//
/// G_RDPPIPESYNC : Waits for the RDP to finish rendering its current primitive.
/// THIS DOES NOT START A DISPLAY LIST.
//
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub struct G_RDPPIPESYNC
{}

impl G_RDPPIPESYNC
{
    pub fn new (_cmd : [u8;8]) -> Result<G_RDPPIPESYNC>
    {
        // no args
        return Ok(G_RDPPIPESYNC{});
    }
    
    pub fn psuedo_code (&self) -> String
    {
        return format!("G_RDPPIPESYNC");
    }
}