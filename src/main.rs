use winsafe::{
    EnumWindows, HPROCESS, HWND, HwndPlace, POINT, SIZE,
    co::{PROCESS, PROCESS_NAME, SWP},
};

fn resize(hwnd: &HWND) {
    let rect = hwnd.GetWindowRect().unwrap();

    hwnd.SetWindowPos(
        HwndPlace::None,
        POINT::with(rect.left, rect.top),
        SIZE::with(rect.right - rect.left, rect.bottom - rect.top + 1),
        SWP::NOZORDER | SWP::NOMOVE,
    )
    .expect("SetWindowPos");
}

fn is_pass_window(hwnd: &HWND) -> bool {
    let Ok(title) = hwnd.GetWindowText() else {
        return false;
    };

    if !title.contains("PasS") {
        return false;
    }

    let (_, pid) = hwnd.GetWindowThreadProcessId();

    let Ok(proc) = HPROCESS::OpenProcess(PROCESS::QUERY_LIMITED_INFORMATION, false, pid) else {
        return false;
    };

    let Ok(path) = proc.QueryFullProcessImageName(PROCESS_NAME::WIN32) else {
        return false;
    };

    if !path.ends_with("PasS.exe") {
        return false;
    }

    true
}

fn recursive_find_subwindow(hwnd: HWND, depth: usize, is_child_of_parts_selection: bool, resize_applied: &mut bool) {
    let mut nth = 0;

    hwnd.EnumChildWindows(|hwnd: HWND| {
        if is_child_of_parts_selection && depth == 2 && nth == 0 {
            println!("    Target component found");
            resize(&hwnd);
            *resize_applied = true;

            println!("    [OK] Resize Applied");
        }

        nth += 1;

        let mut is_child_of_parts_selection = false;

        if let Ok(text) = hwnd.GetWindowText() {
            if text == "部品選択" {
                println!("  Parts selection component found");
                is_child_of_parts_selection = true;
            }
        }

        recursive_find_subwindow(hwnd, depth + 1, is_child_of_parts_selection, resize_applied);

        true
    });
}

fn main() {
    let mut pass_window_found = false;
    let mut resize_applied = false;

    EnumWindows(|hwnd: HWND| -> bool {
        if is_pass_window(&hwnd) {
            pass_window_found = true;

            println!("PasS Window found");
            recursive_find_subwindow(hwnd, 0, false, &mut resize_applied);
        }

        true
    })
    .unwrap();

    if pass_window_found == false {
        println!("ERROR: Failed to find PasS window");
        std::process::exit(1);
    }

    if resize_applied == false {
        println!("ERROR: Failed to resize target component");
        std::process::exit(1);
    }

    std::process::exit(0);
}
