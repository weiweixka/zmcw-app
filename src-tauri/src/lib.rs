use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
/// 运行Tauri应用程序的主函数
///
/// 这个函数负责配置和启动Tauri应用程序，包括设置插件、
/// 窗口属性和应用程序初始化等操作。
pub fn run() {
    // 使用Tauri的Builder创建默认配置的应用程序实例
    // 注释掉的代码包括更新插件和进程插件的初始化
    tauri::Builder::default()
        //.plugin(tauri_plugin_updater::Builder::new().build())
        //.plugin(tauri_plugin_process::init())
        // 初始化shell插件，用于提供系统shell相关的功能
        .plugin(tauri_plugin_shell::init())
        // 设置调用处理器，这里为空表示不使用自定义的IPC调用
        .invoke_handler(tauri::generate_handler![])
        // 应用程序设置，包括窗口初始化和配置
        .setup(|app| {
            // 获取主窗口句柄，如果获取失败则panic
            let window = app.get_webview_window("main").expect("Window failed");
            // 设置窗口忽略鼠标事件，使点击事件穿透到下层窗口
            window
                .set_ignore_cursor_events(true)
                .expect("Failed to set ignore cursor events");
            // 在非Linux系统上最大化窗口
            #[cfg(not(target_os = "linux"))]
            window.maximize().expect("Could not maximize window");

            // 注释掉的代码用于监听鼠标事件
            //let app_handle = app.handle().clone();
            //listen_for_mouse_events(app_handle);
            // 返回Ok表示设置成功
            Ok(())
        })
        // 运行Tauri应用程序，如果出错则panic56S
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
