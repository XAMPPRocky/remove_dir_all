#![cfg(windows)]
use remove_dir_all::remove_dir_all;
use std::fs::{self, File};

macro_rules! assert_not_found {
    ($path:expr) => {{
        match fs::metadata($path) {
            Ok(_) => panic!("did not expect to retrieve metadata for {}", $path),
            Err(ref err) if err.kind() != ::std::io::ErrorKind::NotFound => {
                panic!("expected path {} to be NotFound, was {:?}", $path, err)
            }
            _ => {}
        }
    }};
}

#[test]
fn removes_empty() {
    fs::create_dir_all("./empty").unwrap();
    assert!(fs::metadata("./empty").unwrap().is_dir());

    remove_dir_all("./empty").unwrap();
    assert_not_found!("./empty");
}

#[test]
fn removes_files() {
    fs::create_dir_all("./files").unwrap();

    for i in 0..5 {
        let path = format!("./files/empty-{}.txt", i);

        {
            let mut _file = File::create(&path);
        }

        assert!(fs::metadata(&path).unwrap().is_file());
    }

    remove_dir_all("./files").unwrap();
    assert_not_found!("./files");
}

#[test]
fn removes_dirs() {
    for i in 0..5 {
        let path = format!("./dirs/{}/subdir", i);

        fs::create_dir_all(&path).unwrap();

        assert!(fs::metadata(&path).unwrap().is_dir());
    }

    remove_dir_all("./dirs").unwrap();
    assert_not_found!("./dirs");
}

#[test]
fn removes_read_only() {
    env_logger::init();
    for i in 0..5 {
        let path = format!("./readonly/{}/subdir", i);

        fs::create_dir_all(&path).unwrap();

        let file_path = format!("{}/file.txt", path);
        {
            let file = File::create(&file_path).unwrap();

            if i % 2 == 0 {
                let metadata = file.metadata().unwrap();
                let mut permissions = metadata.permissions();
                permissions.set_readonly(true);

                fs::set_permissions(&file_path, permissions).unwrap();
            }
        }

        assert_eq!(
            i % 2 == 0,
            fs::metadata(&file_path).unwrap().permissions().readonly()
        );

        if i % 2 == 1 {
            let metadata = fs::metadata(&path).unwrap();

            let mut permissions = metadata.permissions();
            permissions.set_readonly(true);

            fs::set_permissions(&path, permissions).unwrap();

            assert!(fs::metadata(&path).unwrap().permissions().readonly());
        }
    }

    remove_dir_all("./readonly").unwrap();
    assert_not_found!("./readonly");
}

// TODO: Should probably test readonly hard links...
