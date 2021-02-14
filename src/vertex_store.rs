
//
/// Single vertex
//
#[repr(C, align(16))]
pub struct Vertex
{
    pub pos: [i16; 3], // x, y, z
    pub flag: u16,
    pub uv: [i16; 2], // u, v
    pub rgb_or_norm: [u8; 3], // rgb or normal
    pub alpha: u8 // alpha
}

//
/// Stores verticies
//
pub struct VertexStore
{
    m_verticies: Vec<Vertex>
}

impl VertexStore
{
    //
    /// Create a new vertex store
    //
    pub fn new () -> VertexStore
    {
        return VertexStore {
            m_verticies: Vec::new()
        }
    }

    //
    /// Add a vertex to the store
    //
    pub fn add (&mut self, v: Vertex) {
        self.m_verticies.push(v);
    }

    //
    /// Get the list of verticies
    //
    pub fn verticies (&self) ->  &Vec<Vertex>
    { return &self.m_verticies; }
}