use std::borrow::Cow;

/// Common media types.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/MIME_types/Common_types)
#[derive(strum::Display, strum::IntoStaticStr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MimeType {
    /// 3GPP audio container.
    #[strum(serialize = "audio/3gpp")]
    Audio3gp,
    /// 3GPP2 audio container.
    #[strum(serialize = "audio/3gpp2")]
    Audio3g2,
    /// 3GPP video container.
    #[strum(serialize = "video/3gpp")]
    Video3gp,
    /// 3GPP2 video container.
    #[strum(serialize = "video/3gpp2")]
    Video3g2,
    /// AAC audio.
    #[strum(serialize = "audio/aac")]
    Aac,
    /// AbiWord document.
    #[strum(serialize = "application/x-abiword")]
    Abw,
    /// Animated Portable Network Graphics (APNG) image.
    #[strum(serialize = "image/apng")]
    Apng,
    /// Archive document (multiple files embedded).
    #[strum(serialize = "application/x-freearc")]
    Arc,
    /// AVIF image.
    #[strum(serialize = "image/avif")]
    Avif,
    /// AVI: Audio Video Interleave.
    #[strum(serialize = "video/x-msvideo")]
    Avi,
    /// Amazon Kindle eBook format.
    #[strum(serialize = "application/vnd.amazon.ebook")]
    Azw,
    /// Any kind of binary data.
    #[strum(serialize = "application/octet-stream")]
    Bin,
    /// Windows OS/2 Bitmap Graphics.
    #[strum(serialize = "image/bmp")]
    Bmp,
    /// BZip archive.
    #[strum(serialize = "application/x-bzip")]
    Bz,
    /// BZip2 archive.
    #[strum(serialize = "application/x-bzip2")]
    Bz2,
    /// CD audio.
    #[strum(serialize = "application/x-cdf")]
    Cda,
    /// C-Shell script.
    #[strum(serialize = "application/x-csh")]
    Csh,
    /// Cascading Style Sheets (CSS).
    #[strum(serialize = "text/css")]
    Css,
    /// Comma-separated values (CSV).
    #[strum(serialize = "text/csv")]
    Csv,
    /// Microsoft Word.
    #[strum(serialize = "application/msword")]
    Doc,
    /// Microsoft Word (OpenXML).
    #[strum(serialize = "application/vnd.openxmlformats-officedocument.wordprocessingml.document")]
    Docx,
    /// MS Embedded OpenType fonts.
    #[strum(serialize = "application/vnd.ms-fontobject")]
    Eot,
    /// Electronic publication (EPUB).
    #[strum(serialize = "application/epub+zip")]
    Epub,
    /// GZip Compressed Archive.
    #[strum(serialize = "application/gzip")]
    Gz,
    /// Graphics Interchange Format (GIF).
    #[strum(serialize = "image/gif")]
    Gif,
    /// HyperText Markup Language (HTML).
    #[strum(serialize = "text/html")]
    Htm,
    /// Icon format.
    #[strum(serialize = "image/vnd.microsoft.icon")]
    Ico,
    /// iCalendar format.
    #[strum(serialize = "text/calendar")]
    Ics,
    /// Java Archive (JAR).
    #[strum(serialize = "application/java-archive")]
    Jar,
    /// JPEG images.
    #[strum(serialize = "image/jpeg")]
    Jpeg,
    /// JavaScript.
    #[strum(serialize = "text/javascript")]
    Js,
    /// JSON format.
    #[strum(serialize = "application/json")]
    Json,
    /// JSON-LD format.
    #[strum(serialize = "application/ld+json")]
    Jsonld,
    /// Markdown.
    #[strum(serialize = "text/markdown")]
    Md,
    /// Musical Instrument Digital Interface (MIDI).
    #[strum(serialize = "audio/midi")]
    Mid,
    /// JavaScript module.
    #[strum(serialize = "text/javascript")]
    Mjs,
    /// MP3 audio.
    #[strum(serialize = "audio/mpeg")]
    Mp3,
    /// MP4 video.
    #[strum(serialize = "video/mp4")]
    Mp4,
    /// MPEG Video.
    #[strum(serialize = "video/mpeg")]
    Mpeg,
    /// Apple Installer Package.
    #[strum(serialize = "application/vnd.apple.installer+xml")]
    Mpkg,
    /// OpenDocument presentation document.
    #[strum(serialize = "application/vnd.oasis.opendocument.presentation")]
    Odp,
    /// OpenDocument spreadsheet document.
    #[strum(serialize = "application/vnd.oasis.opendocument.spreadsheet")]
    Ods,
    /// OpenDocument text document.
    #[strum(serialize = "application/vnd.oasis.opendocument.text")]
    Odt,
    /// Ogg audio.
    #[strum(serialize = "audio/ogg")]
    Oga,
    /// Ogg video.
    #[strum(serialize = "video/ogg")]
    Ogv,
    /// Ogg.
    #[strum(serialize = "application/ogg")]
    Ogx,
    /// Opus audio in Ogg container.
    #[strum(serialize = "audio/ogg")]
    Opus,
    /// OpenType font.
    #[strum(serialize = "font/otf")]
    Otf,
    /// Portable Network Graphics.
    #[strum(serialize = "image/png")]
    Png,
    /// Adobe Portable Document Format (PDF).
    #[strum(serialize = "application/pdf")]
    Pdf,
    /// Hypertext Preprocessor (Personal Home Page).
    #[strum(serialize = "application/x-httpd-php")]
    Php,
    /// Microsoft PowerPoint.
    #[strum(serialize = "application/vnd.ms-powerpoint")]
    Ppt,
    /// Microsoft PowerPoint (OpenXML).
    #[strum(
        serialize = "application/vnd.openxmlformats-officedocument.presentationml.presentation"
    )]
    Pptx,
    /// RAR archive.
    #[strum(serialize = "application/vnd.rar")]
    Rar,
    /// Rich Text Format (RTF).
    #[strum(serialize = "application/rtf")]
    Rtf,
    /// Bourne shell script.
    #[strum(serialize = "application/x-sh")]
    Sh,
    /// Scalable Vector Graphics (SVG).
    #[strum(serialize = "image/svg+xml")]
    Svg,
    /// Tape Archive (TAR).
    #[strum(serialize = "application/x-tar")]
    Tar,
    /// Tagged Image File Format (TIFF).
    #[strum(serialize = "image/tiff")]
    Tif,
    /// MPEG transport stream.
    #[strum(serialize = "video/mp2t")]
    Ts,
    /// TrueType Font.
    #[strum(serialize = "font/ttf")]
    Ttf,
    /// Text, (generally ASCII or ISO 8859-n).
    #[strum(serialize = "text/plain")]
    Txt,
    /// Microsoft Visio.
    #[strum(serialize = "application/vnd.visio")]
    Vsd,
    /// Waveform Audio Format.
    #[strum(serialize = "audio/wav")]
    Wav,
    /// WEBM audio.
    #[strum(serialize = "audio/webm")]
    Weba,
    /// WEBM video.
    #[strum(serialize = "video/webm")]
    Webm,
    /// Web application manifest.
    #[strum(serialize = "application/manifest+json")]
    Webmanifest,
    /// WEBP image.
    #[strum(serialize = "image/webp")]
    Webp,
    /// Web Open Font Format (WOFF).
    #[strum(serialize = "font/woff")]
    Woff,
    /// Web Open Font Format (WOFF2).
    #[strum(serialize = "font/woff2")]
    Woff2,
    /// XHTML.
    #[strum(serialize = "application/xhtml+xml")]
    Xhtml,
    /// Microsoft Excel.
    #[strum(serialize = "application/vnd.ms-excel")]
    Xls,
    /// Microsoft Excel (OpenXML).
    #[strum(serialize = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")]
    Xlsx,
    /// XML.
    #[strum(serialize = "application/xml")]
    Xml,
    /// XUL.
    #[strum(serialize = "application/vnd.mozilla.xul+xml")]
    Xul,
    /// ZIP archive.
    #[strum(serialize = "application/zip")]
    Zip,
    /// 7-zip archive.
    #[strum(serialize = "application/x-7z-compressed")]
    Zip7z,
}

impl From<MimeType> for Cow<'static, str> {
    fn from(mime_type: MimeType) -> Self {
        <&'static str>::from(mime_type).into()
    }
}
