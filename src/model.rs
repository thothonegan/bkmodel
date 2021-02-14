use crate::display_list::DisplayList;
use crate::display_list::DisplayListCommand;
use crate::errors;
use crate::errors::Result;
use crate::geo_type::GeoType;
use crate::texture::Texture;
use crate::texture_type::TextureType;
use crate::vertex_store::{Vertex,VertexStore};

use snafu::ResultExt;
use std::fs::File;
use std::io::Read;
use std::convert::TryInto;

//
/// Main object representing the entire model.
///
/// Note: This is not designed to be efficent - it is designed to 
/// be able to easily introspect, and easily used/converted.
/// 
/// See https://hack64.net/wiki/doku.php?id=banjo_kazooie:model_data
/// Note everything is big endian in the file
//
pub struct Model
{
    // --- internal stats - we shouldnt need those after load, but they're useful for inspecting

    /// The offset within the model that the geometry section starts
    m_internal_geometry_setup_offset : u32,

    /// The offset within the model that the texture section starts
    m_internal_texture_setup_offset: u16,

    /// The geo type
    m_internal_geo_type: GeoType,

    /// The offset where the display list starts
    m_internal_display_list_setup_offset: u32,

    /// The offset the vertex store starts
    m_internal_vertex_store_setup_offset: u32,

    /// The number of triangles (according to the header)
    m_internal_triangle_count: u32,

    /// The number of verticies (according to the header)
    m_internal_vert_count: u32,

    /// Texture headers - information about each texture
    m_textures: Vec<Texture>,

    /// The display list to render the model
    m_displayList: DisplayList,

    /// The vertex store
    m_vertexStore: VertexStore,

    /// The raw data buffer
    /// NOTE: Most of this is in big endian, so convert as needed
    m_data : Vec<u8>
}

impl Model
{
    // --- public static

    //
    /// Loads a model from the given file, returning the model or an error
    //
    pub fn load (filename : String) -> Result<Model> {
        let mut file = File::open(filename).context(errors::IOError)?;

        let mut data = Vec::<u8>::new();
        file.read_to_end(&mut data).context(errors::IOError)?;

        // magic
        let mut cur = 0;
        let magic = u32::from_be_bytes(data[cur..cur+4].try_into().context(errors::FailedSliceError)?);
        cur += 4;

        let expected_magic : u32 = 0x0000000B;
        if magic != expected_magic
        {
            return errors::InvalidMagic{expected_magic:expected_magic, got_magic:magic}.fail();
        }

        let geometry_setup_offset = u32::from_be_bytes(data[cur..cur+4].try_into().context(errors::FailedSliceError)?); //file.read_u32::<BigEndian>().context(errors::IOError)?;
        cur += 4;

        let texture_setup_offset = u16::from_be_bytes(data[cur..cur+2].try_into().context(errors::FailedSliceError)?); // file.read_u16::<BigEndian>().context(errors::IOError)?;
        cur += 2;

        let geo_type_u16 = u16::from_be_bytes(data[cur..cur+2].try_into().context(errors::FailedSliceError)?); // file.read_u16::<BigEndian>().context(errors::IOError)?;
        let geo_type = GeoType::from_u16(geo_type_u16);
        cur += 2;

        let display_list_setup_offset = u32::from_be_bytes(data[cur..cur+4].try_into().context(errors::FailedSliceError)?); // file.read_u32::<BigEndian>().context(errors::IOError)?;
        cur += 4;

        let vertex_store_setup_offset = u32::from_be_bytes(data[cur..cur+4].try_into().context(errors::FailedSliceError)?); // file.read_u32::<BigEndian>().context(errors::IOError)?;
        cur += 4;

        /* unknown 32bit */
        cur += 4;

        /* animation setup 32bit*/
        cur += 4;

        /* collision setup 32bit*/
        cur += 4;

        /* effects setup end address */
        cur += 4;

        /* effects setup */
        cur += 4;

        /* unknown */
        cur += 4;

        /* some animation setup offset? (vertex clipping?) */
        cur += 4;

        /* triangle count [0000kkkk]*/
        cur += 2;
        let tri_count = u16::from_be_bytes(data[cur..cur+2].try_into().context(errors::FailedSliceError)?);
        cur += 2;

        /* vert count [llll0000]*/
        let vert_count = u16::from_be_bytes(data[cur..cur+2].try_into().context(errors::FailedSliceError)?);
        cur += 2;
        cur += 2;


        let mut model = Model {
            m_internal_geometry_setup_offset: geometry_setup_offset,
            m_internal_texture_setup_offset: texture_setup_offset,
            m_internal_geo_type: geo_type,
            m_internal_display_list_setup_offset : display_list_setup_offset,
            m_internal_vertex_store_setup_offset : vertex_store_setup_offset,
            m_internal_triangle_count : tri_count as u32,
            m_internal_vert_count : vert_count as u32, // doesnt sound right

            m_textures : Vec::new(),
            m_displayList: DisplayList::new(),
            m_vertexStore: VertexStore::new(),
            m_data: data
        };

        model.p_parse_displaylist()?;
        model.p_parse_textures()?;
        model.p_parse_vertexstore()?;

        return Ok(model);
    }

    // --- public helpers

    //
    /// Returns a list of valid subjects on the
    //
    pub fn sub_object_names (&self) -> Vec<String> {

        let mut v = Vec::<String>::new();

        // format is always [type]
        // or [type]_[extrastuff: like texture number]

        // display list
        if self.m_internal_display_list_setup_offset != 0x00
        {
            v.push("displaylist".to_string());
        }

        // textures
        for i in &self.m_textures {
            v.push(format!("texture_{}", i.index()));
        }

        return v;
    }

    // --- public properties

    //
    /// The list of textures inside the model
    //
    pub fn textures (&self) -> &Vec<Texture>
    {
        return &self.m_textures;
    }

    //
    /// The display list
    //
    pub fn display_list (&self) -> &DisplayList
    {
        return &self.m_displayList;
    }

    //
    /// The vertex store
    //
    pub fn vertex_store (&self) -> &VertexStore
    { return &self.m_vertexStore; }

    //
    /// The offset within the model that the geometry section starts
    //
    pub fn internal_geometry_setup_offset (&self) -> u32
    { return self.m_internal_geometry_setup_offset; }

    //
    /// The texture setup offset
    //
    pub fn internal_texture_setup_offset (&self) -> u16
    { return self.m_internal_texture_setup_offset; }

    //
    /// The geo type
    //
    pub fn internal_geo_type (&self) -> GeoType
    { return self.m_internal_geo_type; }

    //
    /// The display list offset in the file
    //
    pub fn internal_display_list_setup_offset (&self) -> u32
    { return self.m_internal_display_list_setup_offset; }

    //
    /// The vertex store offset in the file
    //
    pub fn internal_vertex_store_setup_offset (&self) -> u32
    { return self.m_internal_vertex_store_setup_offset; }

    //
    /// The triangle count from the header
    //
    pub fn internal_triangle_count (&self) -> u32
    { return self.m_internal_triangle_count; }

    //
    /// The vertex count from the header
    //
    pub fn internal_vert_count (&self) -> u32
    { return self.m_internal_vert_count; }

    // --- private

    //
    /// Parse the texture section, generating the basic header info
    //
    fn p_parse_textures (&mut self) -> Result<()> {
        let mut cur = (self.m_internal_texture_setup_offset) as usize;
        let _load_bytes = u32::from_be_bytes(self.m_data[cur..cur+4].try_into().context(errors::FailedSliceError)?);
        cur += 4;

        cur += 1;
        let tex_count = self.m_data[cur];
        cur += 3; // skip the rest of the byte

        for i in 0 .. tex_count
        {
            let segment_address = u32::from_be_bytes(self.m_data[cur..cur+4].try_into().context(errors::FailedSliceError)?);
            cur += 4;

            cur += 1;
            let texture_type_u8 = self.m_data[cur];
            let texture_type = TextureType::from_u8(texture_type_u8);
            cur += 3;
            let width = self.m_data[cur];
            cur += 1;
            let height = self.m_data[cur];
            cur += 1;
            cur += 2;

            let wasted_data = u32::from_be_bytes(self.m_data[cur..cur+4].try_into().context(errors::FailedSliceError)?);
            if wasted_data != 0x00 {
                eprintln!("ERROR: Wasted data in texture wasnt 0! was: {}", wasted_data); // safety check
            }
            cur += 4;

            let abs_address = self.m_internal_texture_setup_offset as u32 + segment_address +
                /* header size */ 0x8 + 0x10 * tex_count as u32;
            
           // println!("Texture: {}: addr={:#X} (abs: {:#X}) type={:?} width={} height={}", i, segment_address, abs_address, texture_type, width, height);

            let texture = Texture::new(
                i, segment_address, abs_address, texture_type, width, height
            );
            self.m_textures.push(texture);
        }

        return Ok(());
    }

    //
    /// Parse the display list stuff
    //
    fn p_parse_displaylist (&mut self) -> Result<()>
    {
        let mut cur = self.m_internal_display_list_setup_offset as usize;
        let command_count = u32::from_be_bytes(self.m_data[cur..cur+4].try_into().context(errors::FailedSliceError)?);
        cur += 4;

        /* blank*/ cur += 4;

        for _ in 0 .. command_count
        {
            // each command is 8 bytes
            let command_bytes = self.m_data[cur..cur+8].try_into().context(errors::FailedSliceError)?;
            cur += 8;

            let command = DisplayListCommand::parse(command_bytes)?;
            self.m_displayList.add_command(command);
        }

        return Ok(());
    }

    //
    /// Parse the vertex store
    //
    fn p_parse_vertexstore (&mut self) -> Result<()>
    {
        let mut cur = self.m_internal_vertex_store_setup_offset as usize;

        // draw distance - negative coords
        cur += 6;

        // draw distance - pos coords
        cur += 6;

        // object coordinate range
        cur += 4;

        // collision range? (enemies/objects)
        cur += 2;

        // collission range? (banjo)
        cur += 2;

        // vertex count
        let vertex_count = u16::from_be_bytes(self.m_data[cur..cur+2].try_into().context(errors::FailedSliceError)?);
        cur += 2;

        // verticies
        for i in 0..(vertex_count/2)
        {
            // each vertex should be 16 bytes in a specific format
            // i16: [x][y][z]
            // u16: flag
            // i16: [u][v]
            // u8: [r][g][b][a] or [nx][ny][nz][a]
            let x = i16::from_be_bytes(self.m_data[cur..cur+2].try_into().context(errors::FailedSliceError)?);
            cur += 2;
            let y = i16::from_be_bytes(self.m_data[cur..cur+2].try_into().context(errors::FailedSliceError)?);
            cur += 2;
            let z = i16::from_be_bytes(self.m_data[cur..cur+2].try_into().context(errors::FailedSliceError)?);
            cur += 2;
            let flag = u16::from_be_bytes(self.m_data[cur..cur+2].try_into().context(errors::FailedSliceError)?);
            cur += 2;
            let u = i16::from_be_bytes(self.m_data[cur..cur+2].try_into().context(errors::FailedSliceError)?);
            cur += 2;
            let v = i16::from_be_bytes(self.m_data[cur..cur+2].try_into().context(errors::FailedSliceError)?);
            cur += 2;

            // might be signed if normals
            let r = self.m_data[cur]; cur += 1;
            let g = self.m_data[cur]; cur += 1;
            let b = self.m_data[cur]; cur += 1;
            let a = self.m_data[cur]; cur += 1;

            let vertex = Vertex {
                pos: [x, y, z],
                flag,
                uv: [u, v],
                rgb_or_norm: [r, g, b],
                alpha: a
            };

            self.m_vertexStore.add(vertex);
        }

        return Ok(());
    }
}
