use crate::dlcommands::unknown::Unknown;
use crate::dlcommands::g_cleargeometrymode::G_CLEARGEOMETRYMODE;
use crate::dlcommands::g_dl::G_DL;
use crate::dlcommands::g_enddl::G_ENDDL;
use crate::dlcommands::g_popmtx::G_POPMTX;
use crate::dlcommands::g_rdppipesync::G_RDPPIPESYNC;
use crate::dlcommands::g_setcombine::G_SETCOMBINE;
use crate::dlcommands::g_setothermode_l::G_SetOtherMode_L;
use crate::dlcommands::g_setothermode_h::G_SetOtherMode_H;
use crate::dlcommands::g_setgeometrymode::G_SETGEOMETRYMODE;
use crate::dlcommands::g_settimg::G_SETTIMG;
use crate::dlcommands::g_spnoop::G_SPNOOP;
use crate::dlcommands::g_texture::G_TEXTURE;
use crate::dlcommands::g_tri2::G_TRI2;
use crate::dlcommands::g_vtx::G_VTX;
use crate::errors::Result;

// ----- display list command

/// https://hack64.net/wiki/doku.php?id=f3dex
#[derive(Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
pub enum DisplayListCommand
{
    Unknown(Unknown),

    G_CLEARGEOMETRYMODE(G_CLEARGEOMETRYMODE),
    G_DL(G_DL),
    G_ENDDL(G_ENDDL),
    G_POPMTX(G_POPMTX),
    G_RDPPIPESYNC(G_RDPPIPESYNC),
    G_SETCOMBINE(G_SETCOMBINE),
    G_SETGEOMETRYMODE(G_SETGEOMETRYMODE),
    G_SetOtherMode_L(G_SetOtherMode_L),
    G_SetOtherMode_H(G_SetOtherMode_H),
    G_SETTIMG(G_SETTIMG),
    G_SPNOOP(G_SPNOOP),
    G_TEXTURE(G_TEXTURE),
    G_TRI2(G_TRI2),
    G_VTX(G_VTX),
}

impl DisplayListCommand
{
    //
    // Parse the bytes for a DLC, returning the command
    //
    pub fn parse(bytes : [u8;8]) -> Result<DisplayListCommand>
    {
        return match bytes[0] {
            0x00 => Ok(DisplayListCommand::G_SPNOOP(G_SPNOOP::new(bytes)?)),
            // 01 through 03
            0x04 => Ok(DisplayListCommand::G_VTX(G_VTX::new(bytes)?)),
            // 05 is undefined
            0x06 => Ok(DisplayListCommand::G_DL(G_DL::new(bytes)?)),
            // 07 through B0
            0xB1 => Ok(DisplayListCommand::G_TRI2(G_TRI2::new(bytes)?)),
            // B2 through B5
            0xB6 => Ok(DisplayListCommand::G_CLEARGEOMETRYMODE(G_CLEARGEOMETRYMODE::new(bytes)?)),
            0xB7 => Ok(DisplayListCommand::G_SETGEOMETRYMODE(G_SETGEOMETRYMODE::new(bytes)?)),
            0xB8 => Ok(DisplayListCommand::G_ENDDL(G_ENDDL::new(bytes)?)),
            0xB9 => Ok(DisplayListCommand::G_SetOtherMode_L(G_SetOtherMode_L::new(bytes)?)),
            0xBA => Ok(DisplayListCommand::G_SetOtherMode_H(G_SetOtherMode_H::new(bytes)?)),
            0xBB => Ok(DisplayListCommand::G_TEXTURE(G_TEXTURE::new(bytes)?)),
            // 0xBC
            0xBD => Ok(DisplayListCommand::G_POPMTX(G_POPMTX::new(bytes)?)),
            // BE through E6
            0xE7 => Ok(DisplayListCommand::G_RDPPIPESYNC(G_RDPPIPESYNC::new(bytes)?)),
            // E8 through FB
            0xFC => Ok(DisplayListCommand::G_SETCOMBINE(G_SETCOMBINE::new(bytes)?)),
            0xFD => Ok(DisplayListCommand::G_SETTIMG(G_SETTIMG::new(bytes)?)),
            // 0xFE - 0xFF

            _ => Ok(DisplayListCommand::Unknown(Unknown::new(bytes)?))
        }
    }

    pub fn psuedo_code(&self) -> String
    {
        return match self {
            DisplayListCommand::Unknown(s) => s.psuedo_code(),
            DisplayListCommand::G_CLEARGEOMETRYMODE(s) => s.psuedo_code(),
            DisplayListCommand::G_DL(s) => s.psuedo_code(),
            DisplayListCommand::G_ENDDL(s) => s.psuedo_code(),
            DisplayListCommand::G_POPMTX(s) => s.psuedo_code(),
            DisplayListCommand::G_RDPPIPESYNC(s) => s.psuedo_code(),
            DisplayListCommand::G_SETCOMBINE(s) => s.psuedo_code(),
            DisplayListCommand::G_SETGEOMETRYMODE(s) => s.psuedo_code(),
            DisplayListCommand::G_SetOtherMode_L(s) => s.psuedo_code(),
            DisplayListCommand::G_SetOtherMode_H(s) => s.psuedo_code(),
            DisplayListCommand::G_SETTIMG(s) => s.psuedo_code(),
            DisplayListCommand::G_SPNOOP(s) => s.psuedo_code(),
            DisplayListCommand::G_TEXTURE(s) => s.psuedo_code(),
            DisplayListCommand::G_TRI2(s) => s.psuedo_code(),
            DisplayListCommand::G_VTX(s) => s.psuedo_code()
            //_ => "-------------- NOT HANDLED".to_string()
        };
    }
}

// --- display list

pub struct DisplayList
{
    commands: Vec<DisplayListCommand>
}

impl DisplayList
{
    //
    /// Create a new display list
    //
    pub fn new () -> DisplayList
    {
        return DisplayList{
            commands: Vec::new()
        }
    }

    //
    /// Add a command to the display list
    //
    pub fn add_command (&mut self, command: DisplayListCommand)
    {
        self.commands.push(command);
    }

    pub fn command_count (&self) -> usize
    { return self.commands.len(); }

    pub fn commands (&self) -> &Vec<DisplayListCommand>
    { return &self.commands; }

}
