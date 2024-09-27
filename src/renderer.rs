use std::{fmt::format, path::PathBuf};

pub fn build_ppm_header(size: (u16, u16)) -> String {
    format!("P3 {} {} 255", size.0, size.1)
}

pub(crate) enum Format {
    PPM,
    PNG,
}

pub(crate) struct Renderer {
    format: Format,
    out_path: PathBuf,
    size: (u16, u16),
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_build_ppm_header() {
        assert_eq!(
            build_ppm_header((52, 975)),
            String::from_str("P3 52 975 255").unwrap()
        );
    }
}
