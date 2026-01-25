use std::fs::{self, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

// Import the PE struct and the constant for CodeView (PDB) debug type
use goblin::pe::PE;

pub fn parse(dll_path: &Path, dest: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let dest_pdb_name = "rustplugin.pdb";

    let dest_pdb_name_with_null = "rustplugin.pdb\0";

    let dll_dir = dll_path.parent().ok_or("No path to dll")?;

    let dest_dll_path = dest.join("rustplugin.dll");
    let dest_pdb_path = dest.join("rustplugin.pdb");

    std::fs::copy(dll_path, &dest_dll_path)?;

    let buffer = fs::read(&dest_dll_path)?;
    let pe = PE::parse(&buffer)?;

    let debug_data = pe.debug_data.ok_or("No debug data found in this DLL")?;
    let debug_info = debug_data.codeview_pdb70_debug_info.ok_or("No pdb 70")?;

    let buffer_ptr = buffer.as_ptr() as usize;
    let file_path_ptr = debug_info.filename.as_ptr() as usize;
    //
    let file_path_offset = file_path_ptr - buffer_ptr;
    //
    let file_name = std::str::from_utf8(debug_info.filename)?;
    let file_name_without_null = file_name.trim_matches(char::from(0));

    let pdb_path = dll_dir.join(file_name_without_null);

    std::fs::copy(&pdb_path, &dest_pdb_path)?;
    if dest_pdb_name.len() >= file_name.len() {
        return Result::Err("Not enough space to rename pdb".into());
    }

    {
        let mut file = OpenOptions::new()
            .write(true)
            .read(true)
            .open(&dest_dll_path)?;
        file.seek(SeekFrom::Start(file_path_offset as u64))?;

        // lets null all the previous data
        for _ in 0..=file_name.len() {
            file.write_all(&[0])?;
        }

        file.seek(SeekFrom::Start(file_path_offset as u64))?;
        file.write_all(dest_pdb_name.as_bytes())?;
        file.write_all(&[0])?;

        let mut updated_filename = String::new();

        file.seek(SeekFrom::Start(file_path_offset as u64))?;
        file.take(dest_pdb_name_with_null.len() as u64)
            .read_to_string(&mut updated_filename)?;

        if updated_filename != dest_pdb_name_with_null {
            return Err(format!(
                "Failed to update pdb name. Expected {} but found {}",
                dest_pdb_name_with_null, updated_filename
            )
            .into());
        }
    }
    // //

    Ok(())
}
