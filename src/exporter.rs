use crate::loader::Record;
use quick_xml::Writer;
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;

static ELEMENT_ROOT: &str = "IPPhoneDirectory";
static ELEMENT_ENTRY: &str = "DirectoryEntry";
static ELEMENT_NAME: &str = "Name";
static ELEMENT_PHONE: &str = "Telephone";

pub fn export_phonebook(records: &[Record], output_path: &str) -> Result<(), Box<dyn Error>> {
    let mut writer = Writer::new_with_indent(BufWriter::new(File::create(output_path)?), b' ', 2);

    // Write XML header
    let decl = BytesDecl::new("1.0", Some("UTF-8"), None);
    writer.write_event(Event::Decl(decl))?;

    // Root element
    writer.write_event(Event::Start(BytesStart::new(ELEMENT_ROOT)))?;

    for record in records {
        let name = record.name_for_entry();
        let phone_pbx = &record.phonenumber;
        let phone_direct = format!("{}@{}.local.mesh", &record.phonenumber, &record.phonenumber);

        writer.write_event(Event::Start(BytesStart::new(ELEMENT_ENTRY)))?;

        writer.write_event(Event::Start(BytesStart::new(ELEMENT_NAME)))?;
        writer.write_event(Event::Text(BytesText::new(&name)))?;
        writer.write_event(Event::End(BytesEnd::new(ELEMENT_NAME)))?;

        // Direct Call
        writer.write_event(Event::Start(BytesStart::new(ELEMENT_PHONE)))?;
        writer.write_event(Event::Text(BytesText::new(&phone_direct)))?;
        writer.write_event(Event::End(BytesEnd::new(ELEMENT_PHONE)))?;

        // PBX Call
        writer.write_event(Event::Start(BytesStart::new(ELEMENT_PHONE)))?;
        writer.write_event(Event::Text(BytesText::new(&phone_pbx)))?;
        writer.write_event(Event::End(BytesEnd::new(ELEMENT_PHONE)))?;

        writer.write_event(Event::End(BytesEnd::new(ELEMENT_ENTRY)))?;
    }

    // Close the root element
    writer.write_event(Event::End(BytesEnd::new(ELEMENT_ROOT)))?;

    Ok(())
}
