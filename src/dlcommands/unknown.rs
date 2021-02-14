
use crate::errors::Result;

//
/// Unknown command
//
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub struct Unknown
{
    cmd: [u8;8]
}

impl Unknown
{
    pub fn new (cmd : [u8;8]) -> Result<Unknown>
    {
        return Ok(Unknown{cmd});
    }

    pub fn psuedo_code (&self) -> String
    {
        return format!("UNKNOWN {:?}", self.cmd);
    }
}