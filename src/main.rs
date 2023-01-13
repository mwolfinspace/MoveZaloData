#![allow(non_snake_case)]

use directories::BaseDirs;
use std::{
    fs::{self},
    io,
    path::Path,
    process::{self},
    ptr::null_mut as NULL,
};
use winapi::um::winuser::{self, IDYES};


fn main() {
    if let Some(base_dirs) = BaseDirs::new() {
        //Pre-check ZaloPC
        let a = base_dirs.cache_dir();
        let b = a.join("ZaloPC");
        println!("Dang kiem tra duong dan: {}", b.display());
        println!("ZaloPC da duoc cai tren may: {}", b.exists());
        //Nhập ổ đĩa muốn chuyển
        println!("Nhap o dia muon chuyen ZaloPC sang (VD: D hoac E): ");
        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Failed To read Input");
        let name_driver = name.trim();
        //let len = name_driver.len();
        //name_driver.truncate(len - 1);
        let backup_path = name_driver.to_uppercase() + r":\\ZaloBackup";
        let backup = Path::new(&backup_path);
        let base_path = name_driver.to_uppercase() + r":\\ZaloPC";
        let base = Path::new(&base_path);
        //Hiện đường dẫn muốn chuyển
        println!("ZaloPC se duoc chuyen den {}!", base.display());
        if b.exists() {
            let box_title: Vec<u16> = base_path.encode_utf16().collect();
            let box_msg_ask: Vec<u16> = "Phát hiện ZaloPC ở ổ C! Bạn có muốn tiếp tục?\0"
                .encode_utf16()
                .collect();
            let box_msg_confirm: Vec<u16> = "Bắt đầu chuyển ZaloPC sang ổ chỉ định!\0"
                .encode_utf16()
                .collect();
            let box_msg_cancel: Vec<u16> = "Đã huỷ việc chuyển dữ liệu!\0".encode_utf16().collect();
            let box_msg_complete: Vec<u16> = "Chuyển ZaloPC sang ổ chỉ định thành công!\0"
                .encode_utf16()
                .collect();
            unsafe {
                let msgbox_id = winuser::MessageBoxW(
                    NULL(),
                    box_msg_ask.as_ptr(),
                    box_title.as_ptr(),
                    winuser::MB_YESNO | winuser::MB_ICONEXCLAMATION,
                );
                if msgbox_id == IDYES {
                    winuser::MessageBoxW(
                        NULL(),
                        box_msg_confirm.as_ptr(),
                        box_title.as_ptr(),
                        winuser::MB_OK | winuser::MB_ICONINFORMATION,
                    );
                } else {
                    winuser::MessageBoxW(
                        NULL(),
                        box_msg_cancel.as_ptr(),
                        box_title.as_ptr(),
                        winuser::MB_OK | winuser::MB_ICONINFORMATION,
                    );
                    process::exit(0);
                }
            }
            //Thực hiện sao chép ZaloPC sang ổ D
            println!("Bat dau chuyen du lieu... vui long doi...!");
            println!("Dang sao luu ZaloPC sang {}...", &base_path);
            copy_dir_all(&b, &backup).expect("Error in copying files!");
            println!(" Xong!");
            //Thực hiện xoá ZaloPC ở ổ C
            println!(
                "Dang xoa thu muc ZaloPC goc o duong dan {}... ",
                b.display()
            );
            fs::remove_dir_all(&b).expect("Error in deleting ZaloPC in driver C!");
            println!(" Xong!");
            //Thực hiện mklink cho ZaloPC
            println!("Dang tao lenh mklink ve thu muc ZaloPC goc...");
            std::process::Command::new("cmd")
                .arg("/C")
                .arg("mklink")
                .arg("/d")
                .arg(&b)
                .arg(&base_path)
                .output()
                .expect("Error in making mklink!");
            println!(" Xong!");
            println!("Dang thuc hien chuyen ZaloBackup thanh ZaloPC...");
            //Kiểm tra và xoá thư mục ZaloPC đã bị trùng trước đó
            if base.exists() {
                fs::remove_dir_all(&base_path).expect("Error in delete default mklink folder!");
                //println!(" Xong!");
            }
            //Thực hiện đổi tên Backup thành ZaloPC
            std::process::Command::new("cmd")
                .arg("/C")
                .arg("REN")
                .arg(&backup)
                .arg("ZaloPC")
                .output()
                .expect("Error in renaming folder!");
            if !(base.exists()) {
                fs::rename(&backup_path, &base_path).expect("Error in renaming BackupZalo to ZaloPC!");
            }
            println!(" Xong!");
            unsafe {
                winuser::MessageBoxW(
                    NULL(),
                    box_msg_complete.as_ptr(),
                    box_title.as_ptr(),
                    winuser::MB_OK | winuser::MB_ICONINFORMATION,
                );
                process::exit(0);
            }
        } else {
            let box_msg_cancel: Vec<u16> =
                "Đã huỷ việc chuyển đổi! \n Không phát hiện Zalo trên ổ C!\0"
                    .encode_utf16()
                    .collect();
            let box_title: Vec<u16> = "ZaloPC không tồn tại!\0".encode_utf16().collect();
            unsafe {
                winuser::MessageBoxW(
                    NULL(),
                    box_msg_cancel.as_ptr(),
                    box_title.as_ptr(),
                    winuser::MB_OK | winuser::MB_ICONINFORMATION,
                );
                process::exit(0);
            }
        }
    }
}
fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
