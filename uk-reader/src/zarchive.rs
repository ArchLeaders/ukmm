#[derive(Debug)]
pub(crate) struct ZArchive;

impl super::ROMReader for ZArchive {
    fn get_file_data(&self, name: &str) -> Option<super::ResourceData> {
        unimplemented!()
    }

    fn get_aoc_file_data(&self, name: &str) -> Option<super::ResourceData> {
        unimplemented!()
    }

    fn file_exists(&self, name: &str) -> bool {
        unimplemented!()
    }
}
