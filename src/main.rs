mod commandline_options;
mod dlcommands;
mod display_list;
mod errors;
mod geo_type;
mod model;
mod texture;
mod texture_type;
mod vertex_store;

use commandline_options::CommandLineOptions;
use commandline_options::SubCommand;
use model::Model;
use clap::Clap;
use snafu::ErrorCompat;

fn main() {
    let opts = CommandLineOptions::parse();

    let model_file = opts.filename;

    match opts.sub_command {
        SubCommand::Inspect(_inspect_options) => {
            let model = match Model::load(model_file) {
                Ok(m) => m,
                Err(e) => {
                    eprintln!("An error occurred on load: {}", e);
                    if let Some(backtrace) = ErrorCompat::backtrace(&e) {
                       eprintln!("{}", backtrace);
                    }
                    return;
                }
            };

            println! ("---- Model information -----------");
            println! ("Subobjects:");
            
            let names = model.sub_object_names();
            for name in names {
                println! ("- {}", name);
            }
            println! ("");

            println! ("Textures:");
            let textures = model.textures();
            for texture in textures {
                println!("- texture_{}: addr={:#X} (abs: {:#X}) type={:?} width={} height={}",
                    texture.index(), texture.segment_address(),
                    texture.absolute_address(),
                    texture.texture_type(),
                    texture.width(),
                    texture.height()
                );
            }

            println!("");

            println! ("Internal information:");
            println! ("- Geometry Setup Offset: {:#X}", model.internal_geometry_setup_offset());
            println! ("- Texture Setup Offset: {:#X}", model.internal_texture_setup_offset());
            println! ("- Geo Type: {:?}", model.internal_geo_type());
            println! ("- Display List Setup Offset: {:#X}", model.internal_display_list_setup_offset());
            println! ("- Vertex Store Setup Offset: {:#X}", model.internal_vertex_store_setup_offset());
            println! ("- Tri count: {}", model.internal_triangle_count());
            println! ("- Vert count: {}", model.internal_vert_count());
        },

        SubCommand::ViewDisplayList(_options) => {
            let model = match Model::load(model_file) {
                Ok(m) => m,
                Err(e) => {
                    eprintln!("An error occurred on load: {}", e);
                    if let Some(backtrace) = ErrorCompat::backtrace(&e) {
                       eprintln!("{}", backtrace);
                    }
                    return;
                }
            };

            let displayList = model.display_list();
            let commands = displayList.commands();

            for command in commands
            {
              println!("{}", command.psuedo_code());
            }

            println!("");
            println!("# total length: {} commands", commands.len());
        },

        SubCommand::ViewVertexStore(_options) => {
            let model = match Model::load(model_file) {
                Ok(m) => m,
                Err(e) => {
                    eprintln!("An error occurred on load: {}", e);
                    if let Some(backtrace) = ErrorCompat::backtrace(&e) {
                       eprintln!("{}", backtrace);
                    }
                    return;
                }
            };

            let vs = model.vertex_store();

            for v in vs.verticies()
            {
              println!("pos:[{},{},{}] uv:[{},{}] rgborn:[{},{},{}] alpha:{} flag:{:#X}",
                    v.pos[0], v.pos[1], v.pos[2],
                    v.uv[0], v.uv[1],
                    v.rgb_or_norm[0], v.rgb_or_norm[1], v.rgb_or_norm[2],
                    v.alpha, // or nx, ny, nz, a
                    v.flag
                );
            }

            println!("");
            println!("# total length: {} verts", vs.verticies().len());
        }
    }
}
