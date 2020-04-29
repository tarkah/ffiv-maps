use anyhow::{bail, Error};
use byteorder::{ByteOrder, LittleEndian};
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::Serialize;
use structopt::StructOpt;

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::result;

fn main() -> Result<()> {
    let opts = Opts::from_args();

    if !opts.file.is_file() {
        bail!("<file> must be a valid file");
    }

    let tileset = Tileset::load(&opts.file)?;

    let pretty = PrettyConfig {
        depth_limit: 6,
        separate_tuple_members: true,
        enumerate_arrays: true,
        ..Default::default()
    };
    let s = to_string_pretty(&tileset, pretty).expect("Serialization failed");

    let out_stem = opts.file.file_stem_str();
    fs::write(format!("output/{}.ron", out_stem), s)?;

    Ok(())
}

type Result<T> = result::Result<T, Error>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize)]
enum TextureKind {
    Base,
    Var,
    Anm,
}

impl TextureKind {
    fn make(index: usize) -> Result<TextureKind> {
        match index {
            0 => Ok(TextureKind::Base),
            1 => Ok(TextureKind::Var),
            2 => Ok(TextureKind::Anm),
            n => bail!("Invalid tileset index"),
        }
    }

    fn get_raw(&self) -> usize {
        match self {
            TextureKind::Base => 0,
            TextureKind::Var => 1,
            TextureKind::Anm => 2,
        }
    }

    fn get_suffix(&self) -> &'static str {
        match self {
            TextureKind::Base => "base",
            TextureKind::Var => "var",
            TextureKind::Anm => "anm",
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize)]
enum TriggerKind {
    Uninitialized,
    Passable,
    Blocker,
    UpperLowerDelta,
    LowerUpperDelta,
    Hidden,
    Bridge,
    Damage,
    BottomTransparent,
    BottomHidden,
    Unknown7,
    Unknown12,
    Unknown13,
    Treasure(u8),
    Exit(u8),
    Unknown(u8),
}

impl TriggerKind {
    fn new(v: u8) -> TriggerKind {
        match v {
            0x00 => TriggerKind::Passable,
            0x01 => TriggerKind::Blocker,
            0x02 => TriggerKind::UpperLowerDelta,
            0x03 => TriggerKind::LowerUpperDelta,
            0x04 => TriggerKind::Hidden,
            0x05 => TriggerKind::Bridge,
            0x06 => TriggerKind::Damage,
            0x10 => TriggerKind::BottomTransparent,
            0x11 => TriggerKind::BottomHidden,
            0x07 => TriggerKind::Unknown7,
            0x12 => TriggerKind::Unknown12,
            0x13 => TriggerKind::Unknown13,
            0x20..=0x3F => TriggerKind::Treasure(v & 0x3F),
            0x40..=0x5F => TriggerKind::Exit(v & 0x3F),
            n => TriggerKind::Unknown(n),
        }
    }
}

#[derive(Debug, Serialize)]
struct Cell {
    index: u8,
    kind: TextureKind,
    trigger: TriggerKind,
}

#[derive(Debug, Serialize)]
struct Layer {
    cells: Vec<Cell>,
}

#[derive(Debug, Serialize)]
struct Tileset {
    width: usize,
    height: usize,
    base: PathBuf,
    var: PathBuf,
    anm: PathBuf,
    layers: Vec<Layer>,
}

impl Tileset {
    fn load(path: impl AsRef<Path>) -> Result<Tileset> {
        let mut meta = fs::File::open(&path)?;

        let (base, var, anm) = parse_texture_paths(path)?;

        let width = meta.read_u16()? as _;
        let height = meta.read_u16()? as _;

        let mut layers = build_layers(&mut meta, width, height);

        add_triggers(&mut meta, &mut layers, width, height);

        Ok(Tileset {
            width,
            height,
            base,
            var,
            anm,
            layers,
        })
    }
}

trait ReadExt: io::Read {
    fn read_u16(&mut self) -> Result<u16> {
        let mut buf = [0; 2];
        self.read_exact(&mut buf)?;

        Ok(LittleEndian::read_u16(&buf))
    }
}

impl<R> ReadExt for R where R: io::Read {}

fn build_layers<R: ReadExt>(meta: &mut R, width: usize, height: usize) -> Vec<Layer> {
    let cell_count = width * height;

    (0..2)
        .map(|_| {
            let mut buf = vec![0; cell_count * 2];
            let _ = meta.read_exact(&mut buf);

            let cells = (0..cell_count)
                .map(|i| Cell {
                    index: buf[i * 2],
                    kind: TextureKind::make(buf[i * 2 + 1] as usize).unwrap(),
                    trigger: TriggerKind::Uninitialized,
                })
                .collect();

            Layer { cells }
        })
        .collect()
}

fn add_triggers<R: ReadExt>(meta: &mut R, layers: &mut Vec<Layer>, width: usize, height: usize) {
    let cell_count = width * height;

    let mut buf = vec![0; cell_count * 2];
    let _ = meta.read_exact(&mut buf);

    for i in 0..cell_count {
        let lower = TriggerKind::new(buf[i * 2]);
        let upper = TriggerKind::new(buf[i * 2 + 1]);

        layers[0].cells[i].trigger = lower;
        layers[1].cells[i].trigger = upper;
    }
}

fn parse_texture_paths(path: impl AsRef<Path>) -> Result<(PathBuf, PathBuf, PathBuf)> {
    let (mut base, mut var, mut anm) = (None, None, None);

    let meta_name = path.file_name_str();

    let identifier: &str = if let Some(ident) = meta_name.split('_').next() {
        ident
    } else {
        bail!("Couldn't parse identifier from `.cn2` file name");
    };

    let dir = if let Some(p) = path.as_ref().parent() {
        p
    } else {
        bail!("No parent directory found for supplied `.cn2` file");
    };

    let read_dir = fs::read_dir(&dir)?;

    for maybe_entry in read_dir {
        if let Ok(entry) = maybe_entry {
            let entry_path = entry.path();

            let entry_ext = entry_path.ext_str().to_lowercase();
            let entry_stem = entry_path.file_stem_str().to_lowercase();

            if entry_stem.starts_with(&format!("{}_", identifier)) && entry_ext == "png" {
                if entry_stem.ends_with("_base") {
                    base = Some(entry_path);
                } else if entry_stem.ends_with("_var") {
                    var = Some(entry_path);
                } else if entry_stem.ends_with("_anm") {
                    anm = Some(entry_path);
                }
            }
        }
    }

    if base.is_none() || var.is_none() || anm.is_none() {
        bail!("Couldn't find all related _base, _var and _anm textures");
    }

    Ok((base.unwrap(), var.unwrap(), anm.unwrap()))
}

trait PathExt {
    fn file_name_str(&self) -> &str;

    fn file_stem_str(&self) -> &str;

    fn ext_str(&self) -> &str;
}

impl<P: AsRef<Path>> PathExt for P {
    fn file_name_str(&self) -> &str {
        self.as_ref()
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
    }

    fn file_stem_str(&self) -> &str {
        self.as_ref()
            .file_stem()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
    }

    fn ext_str(&self) -> &str {
        self.as_ref()
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
    }
}

#[derive(StructOpt)]
#[structopt(name = "meta-export", about = "Export CN2 tileset metadata to `.ron` format", version = env!("CARGO_PKG_VERSION"))]
pub struct Opts {
    #[structopt(parse(from_os_str))]
    pub file: PathBuf,
}
