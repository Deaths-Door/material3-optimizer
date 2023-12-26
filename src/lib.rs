#![doc = include_str!("../README.md")]

#![forbid(    
    unsafe_code,
    unused_mut,
    unused_allocation,
    unused_must_use,
    unreachable_patterns,
    trivial_casts,
    unsafe_op_in_unsafe_fn,
    overflowing_literals,
    missing_docs
)]

use std::path::Path;
use std::fs::File;
use std::io::{BufReader,Read,Write};
use zip::read::ZipArchive;
use zip::result::ZipError;

/// Represents a Material3 Builder extractor. It can be
/// used to extract colors from a Material3 Builder ZIP file and generate theme code
pub struct Material3BuilderExtractor(String);

/// An error that can occur when extracting colors from a Material3 Builder ZIP file.
#[derive(thiserror::Error,Debug)]
pub enum Material3BuilderExtractorError {
    /// An error that occurred during file I/O.
    #[error("An error that occurred during file I/O.")]
    File(#[from] std::io::Error),

    /// An error that occurred during ZIP file parsing.
    #[error("An error that occurred during ZIP file parsing.")]
    Zip(#[from] ZipError),
}

impl Material3BuilderExtractor {
    /// Attempts to create a new `Material3BuilderExtractor` from the specified file.
    ///
    /// # Arguments
    ///
    /// * `value`: The path to the Material3 Builder ZIP file.
    ///
    /// # Returns
    ///
    /// A `Result` containing the newly created `Material3BuilderExtractor` if the file
    /// was successfully loaded, or an error if the file could not be found or opened.
    pub fn try_from_file<P>(value : P) -> Result<Self,Material3BuilderExtractorError> where P: AsRef<Path> {
        let file = File::open(value)?;

        let reader = BufReader::new(file);
        let mut archive = ZipArchive::new(reader)?;

        let mut string = String::new();
        let mut color_kt = archive.by_name("ui/theme/Color.kt")?;

        color_kt.read_to_string(&mut string)?;

        Ok(Material3BuilderExtractor(string))
    }
}

impl Material3BuilderExtractor {
    /// Extracts colors from the Material3 Builder ZIP file and applies the specified
    /// transformation to them.
    ///
    /// # Arguments
    ///
    /// * `self`: The `Material3BuilderExtractor` instance.
    /// * `transform`: A function that takes the extracted colors and returns the
    ///   transformed value.
    ///
    /// # Returns
    ///
    /// The transformed value.
    pub fn extract_and<F,T>(&self,transform : F) -> T where F: FnOnce(&[(&str, &str)],&[(&str, &str)]) -> T {
        let colors = Self::extract_colors(&self.0);

        let (light,dark) = colors.split_at(colors.len() / 2);

        transform(light,dark)
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
}

impl Material3BuilderExtractor {
    /// Generates the light color scheme from the specified colors.
    ///
    /// # Arguments
    ///
    /// * `colors`: The vector of color names and hex codes.
    ///
    /// # Returns
    ///
    /// The generated light color scheme.
    pub fn light_color_scheme(colors : &[(&str, &str)]) -> String {
        Self::color_scheme("lightColorScheme", colors)
    }

    /// Generates the dark color scheme from the specified colors.
    ///
    /// # Arguments
    ///
    /// * `colors`: The vector of color names and hex codes.
    ///
    /// # Returns
    ///
    /// The generated dark color scheme.
    pub fn dark_color_scheme(colors : &[(&str, &str)]) -> String {
        Self::color_scheme("darkColorScheme", colors)
    }

    fn color_scheme(function_name : &str,colors : &[(&str, &str)]) -> String {
        let joined_colors = colors
            .iter()
            .map(|(first, second)| format!("{} = {}", first, second))
            .collect::<Vec<String>>()
            .join("\n");
        format!("{} (\n {})",function_name,joined_colors)
    }
}

impl Material3BuilderExtractor {
    /// Extracts colors from the Material3 Builder ZIP file, generates theme code,
    /// and writes it to the specified file.
    ///
    /// # Arguments
    ///
    /// * `self`: The `Material3BuilderExtractor` instance.
    /// * `directory`: The directory to write the generated theme code to.
    ///
    /// # Returns
    ///
    /// A `Result` containing `()` if the theme code was successfully written,
    /// or an error if an error occurred.
    pub fn extract_and_write_to_file(&self,directory : &str) -> Result<(),std::io::Error> {
        self.extract_and(|light_colors,dark_colors|{
            let (light,dark) = (Self::light_color_scheme(light_colors),Self::dark_color_scheme(dark_colors));

            let f_light = format!("val LightColorScheme = {}",light);
            let f_dark = format!("val DarkColorScheme = {}",dark);
    
            // Auto-detect namespace
            let namespace = {
                let dir = if let Some(index) = directory.find("/kotlin/") {
                    &directory[index + "/kotlin/".len()..]
                } else if let Some(index) = directory.find("/main/") {
                    &directory[index + "/main/".len()..]
                } else {
                    directory
                };
    
                &dir.replace("/",".")
            };
            
            const IMPORTS : &str = "import androidx.compose.ui.graphics.Color\nimport androidx.compose.material3.lightColorScheme\nimport androidx.compose.material3.darkColorScheme";
    
            let content = format!("package {namespace}\n{IMPORTS}\n{f_light}\n{f_dark}");
        
            File::create(directory)?.write_all(&content.as_bytes())
        })
    }
}