use std::fs::File;
use std::io::{BufReader,Read,Write};
use zip::read::ZipArchive;

pub struct OptimzeResult {
    light : String,
    dark : String
}

impl OptimzeResult {
    pub fn write_result_to_file(&self,output_directory : &str) {
        let f_light = format!("val LightColorScheme = {}",self.light);
        let f_dark = format!("val DarkColorScheme = {}",self.dark);

        let namespace = {
            let directory = if let Some(index) = output_directory.find("/kotlin/") {
                &output_directory[index + "/kotlin/".len()..]
            } else if let Some(index) = output_directory.find("/main/") {
                &output_directory[index + "/main/".len()..]
            } else {
                output_directory
            };

            &directory.replace("/",".")
        };

        const IMPORTS : &str = "import androidx.compose.ui.graphics.Color\nimport androidx.compose.material3.lightColorScheme\nimport androidx.compose.material3.darkColorScheme";

        let content = format!("package {namespace}\n{IMPORTS}\n{f_light}\n{f_dark}");
    
        File::create(output_directory)
            .expect("Error : File Creation Failed")
            .write_all(&content.as_bytes())
            .expect("Error : Writing to File");
    }
}

impl TryFrom<&str> for OptimzeResult {
    type Error = &'static str;

    fn try_from(input_directory: &str) -> Result<Self, Self::Error> {
        let string = open_zip_and_read_color_kt(input_directory)?;
        
        let colors = extract_colors(&string);

        let (_light,_dark) = colors.split_at(colors.len() / 2);

        let light = construct_color_scheme("lightColorScheme",_light);
        let dark = construct_color_scheme("darkColorScheme",_dark);

        Ok(Self { light,dark })
    }
}

fn open_zip_and_read_color_kt(input_directory: &str) -> Result<String,&'static str> {
    // Read ui/theme/Color.kt in .zip file
    let file = File::open(input_directory).map_err(|_| "Error: The .zip file in the specified directory was not found.")?;

    let reader = BufReader::new(file);
    let mut archive = ZipArchive::new(reader).map_err(|_| "Error: Failed to read the contents of the zip file. Please ensure the zip file is valid and accessible.")?;
    
    
    let mut string = String::new();
    
    let mut color_kt = archive.by_name("ui/theme/Color.kt").map_err(|_|  r"Error: The file ui/theme/Color.kt was not found in the provided zip archive.")?;
            
    color_kt.read_to_string(&mut string).map_err(|_| "Error: Failed to read the file. Please check if the file exists and is accessible.")?;

    Ok(string)
}

fn extract_colors<'a>(string : &'a str) -> Vec<(&'a str, &'a str)> {
    const START_PATTERN : &str = "md_theme_light_primary";
    const END_PATTERN : &str = "md_theme_dark_scrim";

    let start_index = string.find(START_PATTERN).unwrap();

    let _index = string.find(END_PATTERN).unwrap() + END_PATTERN.len();
    let end_index = string[_index..].find('\n').unwrap() + _index;

    string[start_index..end_index]
        .lines()
        .filter(|line| line.trim().starts_with("val"))
        .map(|line| {
            let parts: Vec<&str> = line.trim_start_matches("val").split('=').collect();
            let name = parts[0];
            let last_index = name.rfind("_").unwrap();

            let color = parts[1];

            (&name[last_index + 1..],color)
        })
        .collect()
}

fn construct_color_scheme(function_name : &str,colors : &[(&str, &str)]) -> String {
    let joined_colors = colors
        .iter()
        .map(|(first, second)| format!("{} = {}", first, second))
        .collect::<Vec<String>>()
        .join("\n");
    format!("{} (\n {})",function_name,joined_colors).to_string()
}