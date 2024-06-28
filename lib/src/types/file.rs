#[derive(Clone)]
pub struct File {
    pub remote_md5: String,
    pub remote_path: String,
    pub remote_size: u64,
    pub local_md5: String,
    pub local_path: String,
    pub local_size: u64,
    pub extension: String,
}

impl Default for File {
    fn default() -> Self {
        let remote_md5 = String::new();
        let remote_path = String::new();
        let remote_size = 0;
        let local_md5 = String::new();
        let local_path = String::new();
        let local_size = 0;
        let extension = String::new();

        Self {
            remote_md5,
            remote_path,
            remote_size,
            local_md5,
            local_path,
            local_size,
            extension,
        }
    }
}
