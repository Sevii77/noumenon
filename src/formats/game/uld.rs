#![allow(dead_code)]

use binrw::binrw;
use modular_bitfield::bitfield;
use crate::{NullReader, NullWriter};

#[binrw]
#[brw(little)]
#[derive(Debug)]
pub struct UldHeader {
	pub magic: [u8; 4],
	pub version: [u8; 4],
	
	pub component_offset: u32,
	pub widget_offset: u32,
}

#[binrw]
#[brw(little)]
#[derive(Debug)]
pub struct AtkHeader {
	pub magic: [u8; 4],
	pub version: [u8; 4],
	
	pub asset_list_offset: u32,
	pub part_list_offset: u32,
	pub component_list_offset: u32,
	pub time_line_list_offset: u32,
	pub widget_offset: u32,
	pub rewrite_data_offset: u32,
	pub time_line_list_size: u32,
}

#[binrw]
#[brw(little)]
#[bw(import(element_count: u32))]
#[derive(Debug)]
pub struct ElementHeader {
	pub magic: [u8; 4],
	pub version: [u8; 4],
	#[bw(args{val: element_count})]
	pub element_count: u32,
	pub unk: i32,
}

#[binrw]
#[brw(little)]
#[derive(Debug)]
pub struct TextureEntry {
	pub id: u32,
	#[br(try_map = |v: [u8; 44]| v.null_terminated())]
	#[bw(try_map = |v: &String| v.null_terminated(44))]
	pub path: String,
	pub unk: u32,
	pub unk2: u32,
}

#[binrw]
#[brw(little)]
#[derive(Debug)]
pub struct PartsData {
	pub id: u32,
	pub part_count: u32,
	pub offset: i32,
	#[br(count = part_count)]
	pub parts: Vec<PartData>,
}

#[binrw]
#[brw(little)]
#[derive(Debug)]
pub struct PartData {
	pub id: u32,
	pub u: u16,
	pub v: u16,
	pub w: u16,
	pub h: u16,
}

#[binrw]
#[brw(little)]
#[derive(Debug)]
pub struct ComponentData {
	pub id: u32,
	#[br(map = |v: u8| v != 0)]
	#[bw(map = |v: &bool| if *v {1u8} else {0})]
	pub should_ignore_input: bool,
	#[br(map = |v: u8| v != 0)]
	#[bw(map = |v: &bool| if *v {1u8} else {0})]
	pub drag_arrow: bool,
	#[br(map = |v: u8| v != 0)]
	#[bw(map = |v: &bool| if *v {1u8} else {0})]
	pub drop_arrow: bool,
	pub typ: u8, // todo
	#[bw(calc = nodes.len() as u32)]
	pub node_count: u32,
	pub size: u16,
	pub offset: u16,
	pub componenet: u8, // todo
	#[br(count = node_count)]
	pub nodes: Vec<NodeData>
}

#[bitfield]
#[binrw]
#[br(map = Self::from_bytes)]
#[bw(map = |&v| Self::into_bytes(v))]
#[derive(Debug, Clone, Copy)]
pub struct NodeDataFields {
	pub anchor_right: bool,
	pub anchor_left: bool,
	pub anchor_bottom: bool,
	pub anchor_top: bool,
	pub fill: bool,
	pub clip: bool,
	pub enabled: bool,
	pub visible: bool,
}

#[binrw]
#[brw(little)]
#[derive(Debug)]
pub struct NodeData {
	pub id: u32,
	pub parent_id: i32,
	pub next_id: i32,
	pub prev_id: i32,
	
	pub child_id: i32,
	
	pub node_typ: i32,
	pub node_offset: i16,
	pub tab_index: i16,
	pub unk: [i32; 4],
	pub x: i16,
	pub y: i16,
	pub w: u16,
	pub h: u16,
	pub rotation: f32,
	pub scale_x: f32,
	pub scale_y: f32,
	pub origin_x: i16,
	pub origin_y: i16,
	pub priority: u16,
	
	pub options: NodeDataFields,
	
	pub unk2: u8,
	
	pub multiply_red: i16,
	pub multiply_green: i16,
	pub multiply_blue: i16,
	pub add_red: i16,
	pub add_green: i16,
	pub add_blue: i16,
	pub alpha: u8,
	pub clip_count: u8,
	pub timeline_id: u16
}

#[binrw]
#[brw(little)]
#[derive(Debug)]
pub enum Node {
	Image(ImageNode),
	Text(TextNode),
	NineGrid(NineGridNode),
	Counter(CounterNode),
	Collision(CollisionNode),
}

#[binrw]
#[brw(repr = u8)]
#[repr(u8)]
#[derive(Debug)]
pub enum Font {
	Axis = 0,
	MiedingerMed = 1,
	Miedinger = 2,
	TrumpGothic = 3,
	Jupiter = 4,
	JupiterLarge = 5,
}

#[binrw]
#[brw(repr = u8)]
#[repr(u8)]
#[derive(Debug)]
pub enum SheetType {
	Addon = 0,
	Lobby = 1,
}

#[binrw]
#[brw(repr = u8)]
#[repr(u8)]
#[derive(Debug)]
pub enum CollisionType {
	Hit = 0,
	Focus = 1,
	Move = 2,
}

#[binrw]
#[brw(repr = u8)]
#[repr(u8)]
#[derive(Debug)]
pub enum GridPartsType {
	Divide = 0,
	Compose = 1,
}

#[binrw]
#[brw(repr = u8)]
#[repr(u8)]
#[derive(Debug)]
pub enum GridRenderType {
	Scale = 0,
	Tile = 1,
}

#[binrw]
#[brw(little)]
#[derive(Debug)]
pub struct ImageNode {
	pub partlist_id: u32,
	pub part_id: u32,
	#[br(map = |v: u8| v != 0)]
	#[bw(map = |v: &bool| if *v {1u8} else {0})]
	pub flip_h: bool,
	#[br(map = |v: u8| v != 0)]
	#[bw(map = |v: &bool| if *v {1u8} else {0})]
	pub flip_v: bool,
	pub wrap: u8,
	pub unk: u8,
}

#[bitfield]
#[binrw]
#[br(map = Self::from_bytes)]
#[bw(map = |&v| Self::into_bytes(v))]
#[derive(Debug, Clone, Copy)]
pub struct TextFields {
	pub bold: bool,
	pub italic: bool,
	pub edge: bool,
	pub glare: bool,
	pub multiline: bool,
	pub ellipsis: bool,
	pub paragraph: bool,
	pub emboss: bool,
}

#[binrw]
#[brw(little)]
#[derive(Debug)]
pub struct TextNode {
	pub id: u32,
	pub color: [u8; 4],
	pub alignment: u16,
	pub font: Font,
	pub font_size: u8,
	pub edge_color: [u8; 4],
	pub options: TextFields,
	pub sheet_type: SheetType,
	pub char_spacing: u8,
	pub line_spacing: u8,
	pub unk: u32,
}

#[binrw]
#[brw(little)]
#[derive(Debug)]
pub struct NineGridNode {
	pub partlist_id: u32,
	pub part_id: u32,
	pub grid_parts_type: GridPartsType,
	pub grid_render_type: GridRenderType,
	pub top_offset: u16,
	pub bottom_offset: u16,
	pub left_offset: u16,
	pub right_offset: u16,
	pub unk: [u8; 2],
}

#[binrw]
#[brw(little)]
#[derive(Debug)]
pub struct CounterNode {
	pub partlist_id: u32,
	pub part_id: u32,
	pub number_width: u8,
	pub comma_width: u8,
	pub space_width: u8,
	pub alignment: u16,
	pub unk: u16,
}

#[binrw]
#[brw(little)]
#[derive(Debug)]
pub struct CollisionNode {
	pub typ: CollisionType,
	pub unk: u16,
	pub x: i32,
	pub y: i32,
	pub radius: u32,
}

#[binrw]
#[brw(little)]
#[derive(Debug)]
pub struct Timeline {
	
}

#[binrw]
#[brw(little)]
#[derive(Debug)]
pub struct Uld {
	pub uld_header: UldHeader,
	
	pub atk_header: AtkHeader,
	
	#[bw(args(assets.len() as u32))]
	pub asset_header: ElementHeader,
	#[br(count = asset_header.element_count)]
	pub assets: Vec<TextureEntry>,
	
	#[bw(args(parts.len() as u32))]
	pub parts_header: ElementHeader,
	#[br(count = parts_header.element_count)]
	pub parts: Vec<PartsData>,
	
	#[bw(args(components.len() as u32))]
	pub component_header: ElementHeader,
	#[br(count = component_header.element_count)]
	pub components: Vec<ComponentData>,
	
	#[bw(args(timelines.len() as u32))]
	pub timeline_header: ElementHeader,
	#[br(count = timeline_header.element_count)]
	pub timelines: Vec<Timeline>,
}