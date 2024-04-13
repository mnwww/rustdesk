lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "狀態"),
        ("Your Desktop", "您的桌面"),
        ("desk_tip", "您可以透過此 ID 及密碼存取您的桌面"),
        ("Password", "密碼"),
        ("Ready", "就緒"),
        ("Established", "已建立"),
        ("connecting_status", "正在連線到 RustDesk 網路 ..."),
        ("Enable Service", "啟用服務"),
        ("Start Service", "啟動服務"),
        ("Service is running", "服務正在執行"),
        ("Service is not running", "服務尚未執行"),
        ("not_ready_status", "尚未就緒，請檢查您的網路連線。"),
        ("Control Remote Desktop", "控制遠端桌面"),
        ("Transfer File", "傳輸檔案"),
        ("Connect", "連線"),
        ("Recent Sessions", "近期的工作階段"),
        ("Address Book", "通訊錄"),
        ("Confirmation", "確認"),
        ("TCP Tunneling", "TCP 通道"),
        ("Remove", "移除"),
        ("Refresh random password", "重新產生隨機密碼"),
        ("Set your own password", "自行設定密碼"),
        ("Enable Keyboard/Mouse", "啟用鍵盤和滑鼠"),
        ("Enable Clipboard", "啟用剪貼簿"),
        ("Enable File Transfer", "啟用檔案傳輸"),
        ("Enable TCP Tunneling", "啟用 TCP 通道"),
        ("IP Whitelisting", "IP 白名單"),
        ("ID/Relay Server", "ID / 中繼伺服器"),
        ("Import Server Config", "匯入伺服器設定"),
        ("Export Server Config", "匯出伺服器設定"),
        ("Import server configuration successfully", "匯入伺服器設定成功"),
        ("Export server configuration successfully", "匯出伺服器設定成功"),
        ("Invalid server configuration", "無效的伺服器設定"),
        ("Clipboard is empty", "剪貼簿是空的"),
        ("Stop service", "停止服務"),
        ("Change ID", "更改 ID"),
        ("Your new ID", "您的新 ID"),
        ("length %min% to %max%", "長度在 %min% 與 %max% 之間"),
        ("starts with a letter", "以字母開頭"),
        ("allowed characters", "允許的字元"),
        ("id_change_tip", "僅能使用以下字元：a-z、A-Z、0-9、_ (底線)。首字元必須為 a-z 或 A-Z。長度介於 6 到 16 之間。"),
        ("Website", "網站"),
        ("About", "關於"),
        ("Slogan_tip", "在這個混沌的世界中用心製作！"),
        ("Privacy Statement", "隱私權聲明"),
        ("Mute", "靜音"),
        ("Build Date", "建構日期"),
        ("Version", "版本"),
        ("Home", "首頁"),
        ("Audio Input", "音訊輸入"),
        ("Enhancements", "增強功能"),
        ("Hardware Codec", "硬體編解碼器"),
        ("Adaptive bitrate", "自適應位元速率"),
        ("ID Server", "ID 伺服器"),
        ("Relay Server", "中繼伺服器"),
        ("API Server", "API 伺服器"),
        ("invalid_http", "開頭必須為 http:// 或 https://"),
        ("Invalid IP", "IP 無效"),
        ("Invalid format", "格式無效"),
        ("server_not_support", "伺服器尚未支援"),
        ("Not available", "無法使用"),
        ("Too frequent", "修改過於頻繁，請稍後再試。"),
        ("Cancel", "取消"),
        ("Skip", "跳過"),
        ("Close", "關閉"),
        ("Retry", "重試"),
        ("OK", "確定"),
        ("Password Required", "需要密碼"),
        ("Please enter your password", "請輸入您的密碼"),
        ("Remember password", "記住密碼"),
        ("Wrong Password", "密碼錯誤"),
        ("Do you want to enter again?", "您要重新輸入嗎？"),
        ("Connection Error", "連線錯誤"),
        ("Error", "錯誤"),
        ("Reset by the peer", "對方重設了連線"),
        ("Connecting...", "正在連線 ..."),
        ("Connection in progress. Please wait.", "正在連線，請稍候。"),
        ("Please try 1 minute later", "請於 1 分鐘後再試"),
        ("Login Error", "登入錯誤"),
        ("Successful", "成功"),
        ("Connected, waiting for image...", "已連線，等待畫面傳輸 ..."),
        ("Name", "名稱"),
        ("Type", "類型"),
        ("Modified", "修改時間"),
        ("Size", "大小"),
        ("Show Hidden Files", "顯示隱藏檔案"),
        ("Receive", "接收"),
        ("Send", "傳送"),
        ("Refresh File", "重新整理檔案"),
        ("Local", "本機"),
        ("Remote", "遠端"),
        ("Remote Computer", "遠端電腦"),
        ("Local Computer", "本機電腦"),
        ("Confirm Delete", "確認刪除"),
        ("Delete", "刪除"),
        ("Properties", "屬性"),
        ("Multi Select", "多選"),
        ("Select All", "全選"),
        ("Unselect All", "取消全選"),
        ("Empty Directory", "空資料夾"),
        ("Not an empty directory", "不是一個空資料夾"),
        ("Are you sure you want to delete this file?", "您確定要刪除此檔案嗎？"),
        ("Are you sure you want to delete this empty directory?", "您確定要刪除此空資料夾嗎？"),
        ("Are you sure you want to delete the file of this directory?", "您確定要刪除此資料夾中的檔案嗎？"),
        ("Do this for all conflicts", "套用到其他衝突"),
        ("This is irreversible!", "此操作不可逆！"),
        ("Deleting", "正在刪除 ..."),
        ("files", "檔案"),
        ("Waiting", "正在等候 ..."),
        ("Finished", "已完成"),
        ("Speed", "速度"),
        ("Custom Image Quality", "自訂畫面品質"),
        ("Privacy mode", "隱私模式"),
        ("Block user input", "封鎖使用者輸入"),
        ("Unblock user input", "取消封鎖使用者輸入"),
        ("Adjust Window", "調整視窗"),
        ("Original", "原始"),
        ("Shrink", "縮減"),
        ("Stretch", "延展"),
        ("Scrollbar", "滾動條"),
        ("ScrollAuto", "自動滾動"),
        ("Good image quality", "最佳化畫面品質"),
        ("Balanced", "平衡"),
        ("Optimize reaction time", "最佳化反應時間"),
        ("Custom", "自訂"),
        ("Show remote cursor", "顯示遠端游標"),
        ("Show quality monitor", "顯示品質監測"),
        ("Disable clipboard", "停用剪貼簿"),
        ("Lock after session end", "工作階段結束後鎖定電腦"),
        ("Insert", "插入"),
        ("Insert Lock", "鎖定遠端電腦"),
        ("Refresh", "重新載入"),
        ("ID does not exist", "ID 不存在"),
        ("Failed to connect to rendezvous server", "無法連線到 rendezvous 伺服器"),
        ("Please try later", "請稍候再試"),
        ("Remote desktop is offline", "遠端桌面已離線"),
        ("Key mismatch", "金鑰不符"),
        ("Timeout", "逾時"),
        ("Failed to connect to relay server", "無法連線到中繼伺服器"),
        ("Failed to connect via rendezvous server", "無法透過 rendezvous 伺服器連線"),
        ("Failed to connect via relay server", "無法透過中繼伺服器連線"),
        ("Failed to make direct connection to remote desktop", "無法直接連線到遠端桌面"),
        ("Set Password", "設定密碼"),
        ("OS Password", "作業系統密碼"),
        ("install_tip", "UAC 會導致 RustDesk 在某些情況下無法正常作為遠端端點運作。若要避開 UAC，請點選下方按鈕將 RustDesk 安裝到系統中。"),
        ("Click to upgrade", "點選以升級"),
        ("Click to download", "點選以下載"),
        ("Click to update", "點選以更新"),
        ("Configure", "設定"),
        ("config_acc", "為了遠端控制您的桌面，您需要授予 RustDesk「協助工具」權限。"),
        ("config_screen", "為了遠端存取您的桌面，您需要授予 RustDesk「螢幕錄製」權限。"),
        ("Installing ...", "正在安裝 ..."),
        ("Install", "安裝"),
        ("Installation", "安裝"),
        ("Installation Path", "安裝路徑"),
        ("Create start menu shortcuts", "新增開始功能表捷徑"),
        ("Create desktop icon", "新增桌面捷徑"),
        ("agreement_tip", "開始安裝即表示您接受授權條款。"),
        ("Accept and Install", "接受並安裝"),
        ("End-user license agreement", "終端使用者授權合約"),
        ("Generating ...", "正在產生 ..."),
        ("Your installation is lower version.", "您安裝的版本過舊。"),
        ("not_close_tcp_tip", "在使用通道時請不要關閉此視窗"),
        ("Listening ...", "正在等待通道連線 ..."),
        ("Remote Host", "遠端主機"),
        ("Remote Port", "遠端連接埠"),
        ("Action", "操作"),
        ("Add", "新增"),
        ("Local Port", "本機連接埠"),
        ("Local Address", "本機地址"),
        ("Change Local Port", "修改本機連接埠"),
        ("setup_server_tip", "Provided by MiniYun"),
        ("Too short, at least 6 characters.", "過短，至少需要 6 個字元。"),
        ("The confirmation is not identical.", "兩次輸入不相符"),
        ("Permissions", "權限"),
        ("Accept", "接受"),
        ("Dismiss", "關閉"),
        ("Disconnect", "中斷連線"),
        ("Allow using keyboard and mouse", "允許使用鍵盤和滑鼠"),
        ("Allow using clipboard", "允許使用剪貼簿"),
        ("Allow hearing sound", "允許分享音訊"),
        ("Allow file copy and paste", "允許檔案複製和貼上"),
        ("Connected", "已連線"),
        ("Direct and encrypted connection", "加密直接連線"),
        ("Relayed and encrypted connection", "加密中繼連線"),
        ("Direct and unencrypted connection", "直接且未加密的連線"),
        ("Relayed and unencrypted connection", "中繼且未加密的連線"),
        ("Enter Remote ID", "輸入遠端 ID"),
        ("Enter your password", "輸入您的密碼"),
        ("Logging in...", "正在登入 ..."),
        ("Enable RDP session sharing", "啟用 RDP 工作階段共享"),
        ("Auto Login", "自動登入 (只在您設定「工作階段結束後鎖定」時有效)"),
        ("Enable Direct IP Access", "啟用 IP 直接存取"),
        ("Rename", "重新命名"),
        ("Space", "空白"),
        ("Create Desktop Shortcut", "新增桌面捷徑"),
        ("Change Path", "更改路徑"),
        ("Create Folder", "新增資料夾"),
        ("Please enter the folder name", "請輸入資料夾名稱"),
        ("Fix it", "修復"),
        ("Warning", "警告"),
        ("Login screen using Wayland is not supported", "不支援使用 Wayland 的登入畫面"),
        ("Reboot required", "需要重新啟動"),
        ("Unsupported display server", "不支援的顯示伺服器"),
        ("x11 expected", "預期為 x11"),
        ("Port", "連接埠"),
        ("Settings", "設定"),
        ("Username", "使用者名稱"),
        ("Invalid port", "連接埠無效"),
        ("Closed manually by the peer", "對方關閉了工作階段"),
        ("Enable remote configuration modification", "允許遠端使用者更改設定"),
        ("Run without install", "跳過安裝直接執行"),
        ("Connect via relay", "中繼連線"),
        ("Always connect via relay", "一律透過中繼連線"),
        ("whitelist_tip", "只有白名單上的 IP 可以存取"),
        ("Login", "登入"),
        ("Verify", "驗證"),
        ("Remember me", "記住我"),
        ("Trust this device", "信任此裝置"),
        ("Verification code", "驗證碼"),
        ("verification_tip", "驗證碼已發送到註冊的電子郵件地址，請輸入驗證碼以繼續登入。"),
        ("Logout", "登出"),
        ("Tags", "標籤"),
        ("Search ID", "搜尋 ID"),
        ("whitelist_sep", "使用逗號、分號、空格，或是換行來分隔"),
        ("Add ID", "新增 ID"),
        ("Add Tag", "新增標籤"),
        ("Unselect all tags", "取消選取所有標籤"),
        ("Network error", "網路錯誤"),
        ("Username missed", "缺少使用者名稱"),
        ("Password missed", "缺少密碼"),
        ("Wrong credentials", "登入資訊錯誤"),
        ("The verification code is incorrect or has expired", "驗證碼錯誤或已過期"),
        ("Edit Tag", "編輯標籤"),
        ("Forget Password", "忘記密碼"),
        ("Favorites", "我的最愛"),
        ("Add to Favorites", "加入我的最愛"),
        ("Remove from Favorites", "從我的最愛中移除"),
        ("Empty", "空空如也"),
        ("Invalid folder name", "資料夾名稱無效"),
        ("Socks5 Proxy", "Socks5 代理伺服器"),
        ("Hostname", "主機名稱"),
        ("Discovered", "已探索"),
        ("install_daemon_tip", "若要在開機時啟動，您需要安裝系統服務。"),
        ("Remote ID", "遠端 ID"),
        ("Paste", "貼上"),
        ("Paste here?", "在此貼上？"),
        ("Are you sure to close the connection?", "您確定要關閉連線嗎？"),
        ("Download new version", "下載新版本"),
        ("Touch mode", "觸控模式"),
        ("Mouse mode", "滑鼠模式"),
        ("One-Finger Tap", "單指輕觸"),
        ("Left Mouse", "滑鼠左鍵"),
        ("One-Long Tap", "單指長按"),
        ("Two-Finger Tap", "雙指輕觸"),
        ("Right Mouse", "滑鼠右鍵"),
        ("One-Finger Move", "單指移動"),
        ("Double Tap & Move", "點兩下並移動"),
        ("Mouse Drag", "滑鼠拖曳"),
        ("Three-Finger vertically", "三指垂直滑動"),
        ("Mouse Wheel", "滑鼠滾輪"),
        ("Two-Finger Move", "雙指移動"),
        ("Canvas Move", "移動畫布"),
        ("Pinch to Zoom", "雙指縮放"),
        ("Canvas Zoom", "縮放畫布"),
        ("Reset canvas", "重設畫布"),
        ("No permission of file transfer", "沒有檔案傳輸權限"),
        ("Note", "備註"),
        ("Connection", "連線"),
        ("Share Screen", "螢幕分享"),
        ("Chat", "聊天"),
        ("Total", "總計"),
        ("items", "個項目"),
        ("Selected", "已選擇"),
        ("Screen Capture", "畫面錄製"),
        ("Input Control", "輸入控制"),
        ("Audio Capture", "音訊錄製"),
        ("File Connection", "檔案連線"),
        ("Screen Connection", "畫面連線"),
        ("Do you accept?", "是否接受？"),
        ("Open System Setting", "開啟系統設定"),
        ("How to get Android input permission?", "如何取得 Android 的輸入權限？"),
        ("android_input_permission_tip1", "為了讓遠端裝置能夠透過滑鼠或觸控控制您的 Android 裝置，您需要允許 RustDesk 使用「輔助功能」服務。"),
        ("android_input_permission_tip2", "請前往下一個系統設定頁面，找到並進入「已安裝的服務」，開啟「RustDesk Input」服務。"),
        ("android_new_connection_tip", "收到新的控制請求，對方想要控制您目前的裝置。"),
        ("android_service_will_start_tip", "開啟「畫面錄製」將自動啟動服務，允許其他裝置向您的裝置請求連線。"),
        ("android_stop_service_tip", "關閉服務將自動關閉所有已建立的連線。"),
        ("android_version_audio_tip", "目前的 Android 版本不支援音訊錄製，請升級至 Android 10 或更新的版本。"),
        ("android_start_service_tip", "點選「啟動服務」或啟用「畫面錄製」權限以啟動螢幕分享服務。"),
        ("android_permission_may_not_change_tip", "已建立連線的權限可能不會立即改變，除非重新連線。"),
        ("Account", "帳號"),
        ("Overwrite", "取代"),
        ("This file exists, skip or overwrite this file?", "此檔案/資料夾已存在，要略過或是取代此檔案嗎？"),
        ("Quit", "退出"),
        ("Help", "說明"),
        ("Failed", "失敗"),
        ("Succeeded", "成功"),
        ("Someone turns on privacy mode, exit", "有人開啟了隱私模式，退出"),
        ("Unsupported", "不支援"),
        ("Peer denied", "對方拒絕"),
        ("Please install plugins", "請安裝外掛程式"),
        ("Peer exit", "對方退出"),
        ("Failed to turn off", "關閉失敗"),
        ("Turned off", "已關閉"),
        ("In privacy mode", "開啟隱私模式"),
        ("Out privacy mode", "退出隱私模式"),
        ("Language", "語言"),
        ("Keep RustDesk background service", "保持 RustDesk 後台服務"),
        ("Ignore Battery Optimizations", "忽略電池最佳化"),
        ("android_open_battery_optimizations_tip", "如果您想要停用此功能，請前往下一個 RustDesk 應用程式設定頁面，找到並進入「電池」，取消勾選「不受限制」"),
        ("Start on Boot", "開機時啟動"),
        ("Start the screen sharing service on boot, requires special permissions", "開機時啟動螢幕分享服務，需要特殊權限。"),
        ("Connection not allowed", "不允許連線"),
        ("Legacy mode", "傳統模式"),
        ("Map mode", "1：1 傳輸模式"),
        ("Translate mode", "翻譯模式"),
        ("Use permanent password", "使用固定密碼"),
        ("Use both passwords", "同時使用兩種密碼"),
        ("Set permanent password", "設定固定密碼"),
        ("Enable Remote Restart", "啟用遠端重新啟動"),
        ("Allow remote restart", "允許遠端重新啟動"),
        ("Restart Remote Device", "重新啟動遠端裝置"),
        ("Are you sure you want to restart", "確定要重新啟動嗎？"),
        ("Restarting Remote Device", "正在重新啟動遠端裝置"),
        ("remote_restarting_tip", "遠端裝置正在重新啟動，請關閉此對話框，並在一段時間後使用永久密碼重新連線"),
        ("Copied", "已複製"),
        ("Exit Fullscreen", "退出全螢幕"),
        ("Fullscreen", "全螢幕"),
        ("Mobile Actions", "手機操作"),
        ("Select Monitor", "選擇顯示器"),
        ("Control Actions", "控制操作"),
        ("Display Settings", "顯示設定"),
        ("Ratio", "比例"),
        ("Image Quality", "畫質"),
        ("Scroll Style", "滾動樣式"),
        ("Show Toolbar", "顯示工具列"),
        ("Hide Toolbar", "隱藏工具列"),
        ("Direct Connection", "直接連線"),
        ("Relay Connection", "中繼連線"),
        ("Secure Connection", "安全連線"),
        ("Insecure Connection", "非安全連線"),
        ("Scale original", "原始尺寸"),
        ("Scale adaptive", "適應視窗"),
        ("General", "一般"),
        ("Security", "安全"),
        ("Theme", "主題"),
        ("Dark Theme", "黑暗主題"),
        ("Light Theme", "明亮主題"),
        ("Dark", "黑暗"),
        ("Light", "明亮"),
        ("Follow System", "跟隨系統"),
        ("Enable hardware codec", "啟用硬體編解碼器"),
        ("Unlock Security Settings", "解鎖安全設定"),
        ("Enable Audio", "啟用音訊"),
        ("Unlock Network Settings", "解鎖網路設定"),
        ("Server", "伺服器"),
        ("Direct IP Access", "IP 直接連線"),
        ("Proxy", "代理伺服器"),
        ("Apply", "套用"),
        ("Disconnect all devices?", "中斷所有遠端連線？"),
        ("Clear", "清空"),
        ("Audio Input Device", "音訊輸入裝置"),
        ("Use IP Whitelisting", "只允許白名單上的 IP 進行連線"),
        ("Network", "網路"),
        ("Pin Toolbar", "釘選工具列"),
        ("Unpin Toolbar", "取消釘選工具列"),
        ("Recording", "錄製"),
        ("Directory", "路徑"),
        ("Automatically record incoming sessions", "自動錄製連入的工作階段"),
        ("Change", "變更"),
        ("Start session recording", "開始錄影"),
        ("Stop session recording", "停止錄影"),
        ("Enable Recording Session", "啟用錄製工作階段"),
        ("Allow recording session", "允許錄製工作階段"),
        ("Enable LAN Discovery", "允許區域網路探索"),
        ("Deny LAN Discovery", "拒絕區域網路探索"),
        ("Write a message", "輸入聊天訊息"),
        ("Prompt", "提示"),
        ("Please wait for confirmation of UAC...", "請等待對方確認 UAC ..."),
        ("elevated_foreground_window_tip", "目前的遠端桌面視窗需要更高的權限才能繼續操作，暫時無法使用滑鼠、鍵盤，可以請求對方最小化目前視窗，或者在連線管理視窗點選提升權限。為了避免這個問題，建議在遠端裝置上安裝本軟體。"),
        ("Disconnected", "斷開連線"),
        ("Other", "其他"),
        ("Confirm before closing multiple tabs", "關閉多個分頁前詢問我"),
        ("Keyboard Settings", "鍵盤設定"),
        ("Full Access", "完全存取"),
        ("Screen Share", "僅分享螢幕畫面"),
        ("Wayland requires Ubuntu 21.04 or higher version.", "Wayland 需要 Ubuntu 21.04 或更新的版本。"),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "Wayland 需要更新的版的 Linux 發行版。請嘗試使用 X11 桌面或更改您的作業系統。"),
        ("JumpLink", "查看"),
        ("Please Select the screen to be shared(Operate on the peer side).", "請選擇要分享的螢幕畫面（在對方的裝置上操作）。"),
        ("Show RustDesk", "顯示 RustDesk"),
        ("This PC", "此電腦"),
        ("or", "或"),
        ("Continue with", "繼續"),
        ("Elevate", "提升權限"),
        ("Zoom cursor", "縮放游標"),
        ("Accept sessions via password", "只允許透過輸入密碼進行連線"),
        ("Accept sessions via click", "只允許透過點選接受進行連線"),
        ("Accept sessions via both", "允許輸入密碼或點選接受進行連線"),
        ("Please wait for the remote side to accept your session request...", "請等待對方接受您的連線請求 ..."),
        ("One-time Password", "一次性密碼"),
        ("Use one-time password", "使用一次性密碼"),
        ("One-time password length", "一次性密碼長度"),
        ("Request access to your device", "請求存取您的裝置"),
        ("Hide connection management window", "隱藏連線管理視窗"),
        ("hide_cm_tip", "在只允許密碼連線並且只用固定密碼的情況下才允許隱藏"),
        ("wayland_experiment_tip", "Wayland 支援處於實驗階段，如果您需要使用無人值守存取，請使用 X11。"),
        ("Right click to select tabs", "右鍵選擇分頁"),
        ("Skipped", "已跳過"),
        ("Add to Address Book", "新增到通訊錄"),
        ("Group", "群組"),
        ("Search", "搜尋"),
        ("Closed manually by web console", "被 Web 控制台手動關閉"),
        ("Local keyboard type", "本機鍵盤類型"),
        ("Select local keyboard type", "請選擇本機鍵盤類型"),
        ("software_render_tip", "如果您使用 Nvidia 顯示卡，並且遠端視窗在建立連線後會立刻關閉，那麼請安裝 nouveau 顯示卡驅動程式並且選擇使用軟體渲染可能會有幫助。重新啟動軟體後生效。"),
        ("Always use software rendering", "使用軟體渲染"),
        ("config_input", "為了能夠透過鍵盤控制遠端桌面，請給予 RustDesk \"輸入監控\" 權限。"),
        ("config_microphone", "為了支援透過麥克風進行音訊傳輸，請給予 RustDesk \"錄音\"權限。"),
        ("request_elevation_tip", "如果遠端使用者可以操作電腦，也可以請求提升權限。"),
        ("Wait", "等待"),
        ("Elevation Error", "權限提升失敗"),
        ("Ask the remote user for authentication", "請求遠端使用者進行身分驗證"),
        ("Choose this if the remote account is administrator", "當遠端使用者帳戶是管理員時，請選擇此選項"),
        ("Transmit the username and password of administrator", "發送管理員的使用者名稱和密碼"),
        ("still_click_uac_tip", "依然需要遠端使用者在 UAC 視窗點選確認。"),
        ("Request Elevation", "請求權限提升"),
        ("wait_accept_uac_tip", "請等待遠端使用者確認 UAC 對話框。"),
        ("Elevate successfully", "權限提升成功"),
        ("uppercase", "大寫字母"),
        ("lowercase", "小寫字母"),
        ("digit", "數字"),
        ("special character", "特殊字元"),
        ("length>=8", "長度不能小於 8"),
        ("Weak", "弱"),
        ("Medium", "中"),
        ("Strong", "強"),
        ("Switch Sides", "反轉存取方向"),
        ("Please confirm if you want to share your desktop?", "請確認是否要讓對方存取您的桌面？"),
        ("Display", "顯示"),
        ("Default View Style", "預設顯示方式"),
        ("Default Scroll Style", "預設滾動方式"),
        ("Default Image Quality", "預設圖像品質"),
        ("Default Codec", "預設編解碼器"),
        ("Bitrate", "位元速率"),
        ("FPS", "幀率"),
        ("Auto", "自動"),
        ("Other Default Options", "其他預設選項"),
        ("Voice call", "語音通話"),
        ("Text chat", "文字聊天"),
        ("Stop voice call", "停止語音通話"),
        ("relay_hint_tip", "可能無法直接連線，可以嘗試中繼連線。\n另外，如果想要直接使用中繼連線，可以在 ID 後面新增/r，如果近期工作階段裏存在該分頁，也可以在分頁選項裡選擇強制走中繼連線。"),
        ("Reconnect", "重新連線"),
        ("Codec", "編解碼器"),
        ("Resolution", "解析度"),
        ("No transfers in progress", "沒有正在進行的傳輸"),
        ("Set one-time password length", "設定一次性密碼長度"),
        ("install_cert_tip", ""),
        ("confirm_install_cert_tip", ""),
        ("RDP Settings", "RDP 設定"),
        ("Sort by", "排序方式"),
        ("New Connection", "新連線"),
        ("Restore", "還原"),
        ("Minimize", "最小化"),
        ("Maximize", "最大化"),
        ("Your Device", "您的裝置"),
        ("empty_recent_tip", "哎呀，沒有近期的工作階段！\n是時候安排點新工作了。"),
        ("empty_favorite_tip", "空空如也"),
        ("empty_lan_tip", "喔不，看來我們目前找不到任何夥伴。"),
        ("empty_address_book_tip", "老天，看來您的通訊錄中沒有任何夥伴。"),
        ("eg: admin", "例如：admin"),
        ("Empty Username", "空使用者帳號"),
        ("Empty Password", "空密碼"),
        ("Me", "我"),
        ("identical_file_tip", "此檔案與對方的檔案一致。"),
        ("show_monitors_tip", "在工具列中顯示顯示器"),
        ("View Mode", "瀏覽模式"),
        ("login_linux_tip", "需要登入到遠端 Linux 使用者帳戶才能啟用 X 介面"),
        ("verify_rustdesk_password_tip", "驗證 RustDesk 密碼"),
        ("remember_account_tip", "記住此使用者帳戶"),
        ("os_account_desk_tip", "此使用者帳戶將用於登入遠端作業系統並啟用無頭模式的桌面連線"),
        ("OS Account", "作業系統使用者帳戶"),
        ("another_user_login_title_tip", "另一個使用者已經登入"),
        ("another_user_login_text_tip", "斷開連線"),
        ("xorg_not_found_title_tip", "找不到 Xorg"),
        ("xorg_not_found_text_tip", "請安裝 Xorg"),
        ("no_desktop_title_tip", "沒有可用的桌面"),
        ("no_desktop_text_tip", "請安裝 GNOME 桌面"),
        ("No need to elevate", "不需要提升權限"),
        ("System Sound", "系統音效"),
        ("Default", "預設"),
        ("New RDP", "新的 RDP"),
        ("Fingerprint", "指紋"),
        ("Copy Fingerprint", "複製指紋"),
        ("no fingerprints", "沒有指紋"),
        ("Select a peer", "選擇夥伴"),
        ("Select peers", "選擇夥伴"),
        ("Plugins", "外掛程式"),
        ("Uninstall", "解除安裝"),
        ("Update", "更新"),
        ("Enable", "啟用"),
        ("Disable", "停用"),
        ("Options", "選項"),
        ("resolution_original_tip", "原始解析度"),
        ("resolution_fit_local_tip", "調整成本機解析度"),
        ("resolution_custom_tip", "自動解析度"),
        ("Collapse toolbar", "收回工具列"),
        ("Accept and Elevate", "接受並提升"),
        ("accept_and_elevate_btn_tooltip", "接受連線並提升 UAC 權限。"),
        ("clipboard_wait_response_timeout_tip", "等待複製回應逾時。"),
        ("Incoming connection", "接收的連線"),
        ("Outgoing connection", "發起的連線"),
        ("Exit", "退出"),
        ("Open", "開啟"),
        ("logout_tip", "確定要登出嗎？"),
        ("Service", "服務"),
        ("Start", "啟動"),
        ("Stop", "停止"),
        ("exceed_max_devices", "超過最大裝置數量"),
        ("Sync with recent sessions", "與近期工作階段同步"),
        ("Sort tags", "排序標籤"),
        ("Open connection in new tab", "在新分頁開啟連線"),
        ("Move tab to new window", "移動標籤到新視窗"),
        ("Can not be empty", "不能為空"),
        ("Already exists", "已經存在"),
        ("Change Password", "更改密碼"),
        ("Refresh Password", "重新整理密碼"),
        ("ID", "ID"),
        ("Grid View", "網格檢視"),
        ("List View", "清單檢視"),
        ("Select", "選擇"),
        ("Toggle Tags", "切換標籤"),
        ("pull_ab_failed_tip", "通訊錄更新失敗"),
        ("push_ab_failed_tip", "成功同步通訊錄至伺服器"),
        ("synced_peer_readded_tip", "最近會話中存在的設備將會被重新同步到通訊錄。"),
        ("Change Color", "更改顏色"),
        ("Primary Color", "基本色"),
        ("HSV Color", "HSV 色"),
        ("Installation Successful!", ""),
        ("Installation failed!", ""),
        ("Reverse mouse wheel", ""),
        ("{} sessions", ""),
        ("scam_title", ""),
        ("scam_text1", ""),
        ("scam_text2", ""),
        ("Don't show again", ""),
        ("I Agree", ""),
        ("Decline", ""),
        ("Timeout in minutes", ""),
        ("auto_disconnect_option_tip", ""),
        ("Connection failed due to inactivity", ""),
        ("Check for software update on startup", ""),
        ("upgrade_rustdesk_server_pro_to_{}_tip", ""),
        ("pull_group_failed_tip", ""),
        ("Filter by intersection", "")
    ].iter().cloned().collect();
}
