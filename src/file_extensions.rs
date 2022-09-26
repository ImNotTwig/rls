pub mod file_extensions {
    lazy_static! {
        pub static ref IMAGE_FILES: [String; 38] = [
            "png".to_string(),
            "jfi".to_string(),
            "jfif".to_string(), 
            "jif".to_string(), 
            "jpe".to_string(), 
            "jpeg".to_string(), 
            "jpg".to_string(), 
            "gif".to_string(), 
            "bmp".to_string(),
            "tiff".to_string(), 
            "tif".to_string(), 
            "ppm".to_string(), 
            "pgm".to_string(), 
            "pbm".to_string(), 
            "pnm".to_string(), 
            "webp".to_string(), 
            "raw".to_string(), 
            "arw".to_string(),
            "svg".to_string(), 
            "stl".to_string(), 
            "eps".to_string(), 
            "dvi".to_string(), 
            "ps".to_string(), 
            "cbr".to_string(), 
            "jpf".to_string(), 
            "cbz".to_string(), 
            "xpm".to_string(),
            "ico".to_string(), 
            "cr2".to_string(), 
            "orf".to_string(), 
            "nef".to_string(), 
            "heif".to_string(), 
            "avif".to_string(), 
            "jxl".to_string(), 
            "j2k".to_string(), 
            "jp2".to_string(),
            "j2c".to_string(), 
            "jpx".to_string()
        ];
    }

    lazy_static! {
        pub static ref MUSIC: [String; 16] = [
            "avi".to_string(), 
            "flv".to_string(), 
            "m2v".to_string(), 
            "m4v".to_string(), 
            "mkv".to_string(), 
            "mov".to_string(), 
            "mp4".to_string(), 
            "mpeg".to_string(),
            "mpg".to_string(), 
            "ogm".to_string(), 
            "ogv".to_string(), 
            "vob".to_string(), 
            "wmv".to_string(), 
            "webm".to_string(), 
            "m2ts".to_string(), 
            "heic".to_string()
        ];
    }
    lazy_static! {
        pub static ref LOSSLESS: [String; 4] = [
            "alac".to_string(), 
            "ape".to_string(), 
            "flac".to_string(), 
            "wav".to_string()
        ];
    }
    lazy_static! {
        pub static ref CRYPTO: [String; 8] = [
            "asc".to_string(), 
            "enc".to_string(), 
            "gpg".to_string(), 
            "pgp".to_string(), 
            "sig".to_string(), 
            "signature".to_string(), 
            "pfx".to_string(), 
            "p12".to_string()
        ];
    }
    lazy_static! {
        pub static ref ARCHIVE: [String; 25] = [
            "zip".to_string(), 
            "tar".to_string(), 
            "Z".to_string(), 
            "z".to_string(), 
            "gz".to_string(), 
            "bz2".to_string(), 
            "a".to_string(), 
            "ar".to_string(), 
            "7z".to_string(),
            "iso".to_string(), 
            "dmg".to_string(), 
            "tc".to_string(), 
            "rar".to_string(), 
            "par".to_string(), 
            "tgz".to_string(), 
            "xz".to_string(), 
            "txz".to_string(),
            "lz".to_string(), 
            "tlz".to_string(), 
            "lzma".to_string(), 
            "deb".to_string(), 
            "rpm".to_string(), 
            "zst".to_string(), 
            "lz4".to_string(), 
            "cpio".to_string()
        ];
    }
}