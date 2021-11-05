fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_path = "proto".to_string();
    let files = &["todo_tags"];

    tonic_build::configure()
    // .type_attribute(".todoTags.Item", "use serde_derive::{Serialize, Deserialize};")
    // .type_attribute(".", "#[derive(Serialize, Deserialize)]")
    .compile(
            files.iter()
                  .map(|file| format!("{}/{}.proto", &proto_path, file) )
                  .collect::<Vec<String>>().as_ref(),
            &[proto_path]
        )?;
    Ok(())
}

