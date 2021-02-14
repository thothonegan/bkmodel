use crate::errors::Result;

//
/// G_ENDDL: Ends the current display list
//
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub struct G_ENDDL
{}

impl G_ENDDL
{
    pub fn new (_cmd : [u8;8]) -> Result<G_ENDDL>
    {
        return Ok(G_ENDDL{});
    }
    
    pub fn psuedo_code (&self) -> String
    {
        return format!("G_ENDDL");
    }
}