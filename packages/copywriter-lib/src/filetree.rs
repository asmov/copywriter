
pub const INPUT_FILETREE: &'static str = r"
    site.toml :: Site Information: Information about the website
    content/ :: Content: Website content organized by datamodel
    pkg/ :: Packages: Third-party bundles of Src files for the website
    src/ :: Src: HTML, CSS, JS, multimedia, etc.
        css/ :: CSS: Styling and themes for the website
        docs/ :: Documents: PDFs, Word documents, etc.
        downloads/ :: Downloads: Files for download
        hbs/ :: Templates: Handlebars templates for building the website
            layout/ :: Layout Templates: Templates for the website's header, footer, etc.
            model/ :: Model Templates: Template pages and snippets for each datamodel
        images/ :: Images: Image files for the website
        js/ :: Javascript: Javascript code for the website
        music/ :: Music: Music for the website
        sounds/ :: Sounds: Sound files for the website
        videos/ :: Videos: Video files for the website
        wasm/ :: WebAssembly: WebAssembly code for the website
    ";
