use crate::errors::Result;

//
/// G_TRI2: Renders two triangles in the vertex buffer
//
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub struct G_TRI2
{
    // note all vert indexes are * 2
    m_vert1: u8,
    m_vert2: u8,
    m_vert3: u8,

    m_vert4: u8,
    m_vert5: u8,
    m_vert6: u8
}

impl G_TRI2
{
    pub fn new (_cmd : [u8;8]) -> Result<G_TRI2>
    {
        let v1 = _cmd[1];
        let v2 = _cmd[2];
        let v3 = _cmd[3];
        // _cmd[4] is 0
        let v4 = _cmd[5];
        let v5 = _cmd[6];
        let v6 = _cmd[7];

        return Ok(G_TRI2{
            m_vert1: v1,
            m_vert2: v2,
            m_vert3: v3,

            m_vert4: v4,
            m_vert5: v5,
            m_vert6: v6,
        });
    }
    
    pub fn psuedo_code (&self) -> String
    {
        // we display as the proper index (undoing the *2)
        return format!("G_TRI2 [v1:{}, v2:{}, v3:{}] [v4:{}, v5:{}, v6:{}]",
            self.m_vert1/2, self.m_vert2/2, self.m_vert3/2,
            self.m_vert4/2, self.m_vert5/2, self.m_vert6/2
        );
    }
}