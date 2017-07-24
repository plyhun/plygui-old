extern crate embed_resource;

fn main() {
	#[cfg(target_os = "windows")]
    embed_resource::compile("resources/win32/plygui.rc");
}