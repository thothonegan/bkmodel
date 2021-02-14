use crate::errors::Result;

//
/// G_SETCOMBINE: Sets the blend mode (via the color combiner).
//
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub struct G_SETCOMBINE
{}

impl G_SETCOMBINE
{
    pub fn new (_cmd : [u8;8]) -> Result<G_SETCOMBINE>
    {
        // TODO: this has a bunch of flags for customing the combiner.
        return Ok(G_SETCOMBINE{});
    }
    
    pub fn psuedo_code (&self) -> String
    {
        return format!("G_SETCOMBINE [TODO: flags]");
    }
}