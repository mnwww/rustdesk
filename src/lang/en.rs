lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("desk_tip", "Your desktop can be accessed with this ID and password."),
        ("connecting_status", "Connecting to the RustDesk network..."),
        ("Enable Service", "Enable service"),
        ("Start Service", "Enable service"),
        ("not_ready_status", "Not ready. Please check your connection"),
        ("Transfer File", "Transfer file"),
        ("Recent Sessions", "Recent sessions"),
        ("Address Book", "Address book"),
        ("TCP Tunneling", "TCP tunneling"),
        ("Enable Keyboard/Mouse", "Enable keyboard/mouse"),
        ("Enable Clipboard", "Enable clipboard"),
        ("Enable File Transfer", "Enable file transfer"),
        ("Enable TCP Tunneling", "Enable TCP tunneling"),
        ("IP Whitelisting", "IP whitelisting"),
        ("ID/Relay Server", "ID/Relay server"),
        ("Import Server Config", "Import server config"),
        ("Export Server Config", "Export server config"),
        ("id_change_tip", "Only a-z, A-Z, 0-9 and _ (underscore) characters allowed. The first letter must be a-z, A-Z. Length between 6 and 16."),
        ("Slogan_tip", "Made with heart in this chaotic world!"),
        ("Build Date", "Build date"),
        ("Audio Input", "Audio input"),
        ("Hardware Codec", "Hardware codec"),
        ("ID Server", "ID server"),
        ("Relay Server", "Relay server"),
        ("API Server", "API server"),
        ("invalid_http", "must start with http:// or https://"),
        ("server_not_support", "Not yet supported by the server"),
        ("Password Required", "Password required"),
        ("Wrong Password", "Wrong password"),
        ("Connection Error", "Connection error"),
        ("Login Error", "Login error"),
        ("Show Hidden Files", "Show hidden files"),
        ("Refresh File", "Refresh file"),
        ("Remote Computer", "Remote computer"),
        ("Local Computer", "Local computer"),
        ("Confirm Delete", "Confirm delete"),
        ("Multi Select", "Multi select"),
        ("Select All", "Select all"),
        ("Unselect All", "Unselect all"),
        ("Empty Directory", "Empty directory"),
        ("Custom Image Quality", "Custom image quality"),
        ("Adjust Window", "Adjust window"),
        ("Insert Lock", "Insert lock"),
        ("Set Password", "Set password"),
        ("OS Password", "OS password"),
        ("install_tip", "Due to UAC, RustDesk can not work properly as the remote side in some cases. To avoid UAC, please click the button below to install RustDesk to the system."),
        ("config_acc", "In order to control your Desktop remotely, you need to grant RustDesk \"Accessibility\" permissions."),
        ("config_screen", "In order to access your Desktop remotely, you need to grant RustDesk \"Screen Recording\" permissions."),
        ("Installation Path", "Installation path"),
        ("agreement_tip", "By starting the installation, you accept the license agreement."),
        ("Accept and Install", "Accept and install"),
        ("not_close_tcp_tip", "Don't close this window while you are using the tunnel"),
        ("Remote Host", "Remote host"),
        ("Remote Port", "Remote port"),
        ("Local Port", "Local port"),
        ("Local Address", "Local address"),
        ("Change Local Port", "Change local port"),
        ("setup_server_tip", "Provided by MiniYun"),
        ("Enter Remote ID", "Enter remote ID"),
        ("Auto Login", "Auto Login (Only valid if you set \"Lock after session end\")"),
        ("Enable Direct IP Access", "Enable direct IP access"),
        ("Create Desktop Shortcut", "Create desktop shortcut"),
        ("Change Path", "Change path"),
        ("Create Folder", "Create folder"),
        ("whitelist_tip", "Only whitelisted IP can access me"),
        ("verification_tip", "A verification code has been sent to the registered email address, enter the verification code to continue logging in."),
        ("whitelist_sep", "Separated by comma, semicolon, spaces or new line"),
        ("Add Tag", "Add tag"),
        ("Wrong credentials", "Wrong username or password"),
        ("Edit Tag", "Edit tag"),
        ("Forget Password", "Forget password"),
        ("Add to Favorites", "Add to favorites"),
        ("Remove from Favorites", "Remove from favorites"),
        ("Socks5 Proxy", "Socks5 proxy"),
        ("install_daemon_tip", "For starting on boot, you need to install system service."),
        ("Are you sure to close the connection?", "Are you sure you want to close the connection?"),
        ("One-Finger Tap", "One-finger tap"),
        ("Left Mouse", "Left mouse"),
        ("One-Long Tap", "One-long tap"),
        ("Two-Finger Tap", "Two-finger tap"),
        ("Right Mouse", "Right mouse"),
        ("One-Finger Move", "One-finger move"),
        ("Double Tap & Move", "Double tap & move"),
        ("Mouse Drag", "Mouse drag"),
        ("Three-Finger vertically", "Three-finger vertically"),
        ("Mouse Wheel", "Mouse wheel"),
        ("Two-Finger Move", "Two-finger move"),
        ("Canvas Move", "Canvas move"),
        ("Pinch to Zoom", "Pinch to zoom"),
        ("Canvas Zoom", "Canvas zoom"),
        ("Share Screen", "Share screen"),
        ("Screen Capture", "Screen capture"),
        ("Input Control", "Input control"),
        ("Audio Capture", "Audio capture"),
        ("File Connection", "File connection"),
        ("Screen Connection", "Screen connection"),
        ("Open System Setting", "Open system setting"),
        ("android_input_permission_tip1", "In order for a remote device to control your Android device via mouse or touch, you need to allow RustDesk to use the \"Accessibility\" service."),
        ("android_input_permission_tip2", "Please go to the next system settings page, find and enter [Installed Services], turn on [RustDesk Input] service."),
        ("android_new_connection_tip", "New control request has been received, which wants to control your current device."),
        ("android_service_will_start_tip", "Turning on \"Screen Capture\" will automatically start the service, allowing other devices to request a connection to your device."),
        ("android_stop_service_tip", "Closing the service will automatically close all established connections."),
        ("android_version_audio_tip", "The current Android version does not support audio capture, please upgrade to Android 10 or higher."),
        ("android_start_service_tip", "Tap [Start Service] or enable [Screen Capture] permission to start the screen sharing service."),
        ("android_permission_may_not_change_tip", "Permissions for established connections may not be changed instantly until reconnected."),
        ("doc_mac_permission", "https://rustdesk.com/docs/en/manual/mac/#enable-permissions"),
        ("Ignore Battery Optimizations", "Ignore battery optimizations"),
        ("android_open_battery_optimizations_tip", "If you want to disable this feature, please go to the next RustDesk application settings page, find and enter [Battery], Uncheck [Unrestricted]"),
        ("Start on Boot", "Start on boot"),
        ("Enable Remote Restart", "Enable remote restart"),
        ("Restart Remote Device", "Restart remote device"),
        ("Restarting Remote Device", "Restarting remote device"),
        ("remote_restarting_tip", "Remote device is restarting, please close this message box and reconnect with permanent password after a while"),
        ("Exit Fullscreen", "Exit fullscreen"),
        ("Mobile Actions", "Mobile actions"),
        ("Select Monitor", "Select monitor"),
        ("Control Actions", "Control actions"),
        ("Display Settings", "Display settings"),
        ("Image Quality", "Image quality"),
        ("Scroll Style", "Scroll style"),
        ("Show Toolbar", "Show toolbar"),
        ("Hide Toolbar", "Hide toolbar"),
        ("Direct Connection", "Direct connection"),
        ("Relay Connection", "Relay connection"),
        ("Secure Connection", "Secure connection"),
        ("Insecure Connection", "Insecure connection"),
        ("Dark Theme", "Dark theme"),
        ("Light Theme", "Light theme"),
        ("Follow System", "Follow system"),
        ("Unlock Security Settings", "Unlock security settings"),
        ("Enable Audio", "Enable audio"),
        ("Unlock Network Settings", "Unlock network settings"),
        ("Direct IP Access", "Direct IP access"),
        ("Audio Input Device", "Audio input device"),
        ("Use IP Whitelisting", "Use IP whitelisting"),
        ("Pin Toolbar", "Pin toolbar"),
        ("Unpin Toolbar", "Unpin toolbar"),
        ("Enable Recording Session", "Enable recording session"),
        ("Enable LAN Discovery", "Enable LAN discovery"),
        ("Deny LAN Discovery", "Deny LAN discovery"),
        ("elevated_foreground_window_tip", "The current window of the remote desktop requires higher privilege to operate, so it's unable to use the mouse and keyboard temporarily. You can request the remote user to minimize the current window, or click elevation button on the connection management window. To avoid this problem, it is recommended to install the software on the remote device."),
        ("Keyboard Settings", "Keyboard settings"),
        ("Full Access", "Full access"),
        ("Screen Share", "Screen share"),
        ("JumpLink", "View"),
        ("Please Select the screen to be shared(Operate on the peer side).", "Please select the screen to be shared(Operate on the peer side)."),
        ("One-time Password", "One-time password"),
        ("hide_cm_tip", "Allow hiding only if accepting sessions via password and using permanent password"),
        ("wayland_experiment_tip", "Wayland support is in experimental stage, please use X11 if you require unattended access."),
        ("Add to Address Book", "Add to address book"),
        ("software_render_tip", "If you're using Nvidia graphics card under Linux and the remote window closes immediately after connecting, switching to the open-source Nouveau driver and choosing to use software rendering may help. A software restart is required."),
        ("config_input", "In order to control remote desktop with keyboard, you need to grant RustDesk \"Input Monitoring\" permissions."),
        ("config_microphone", "In order to speak remotely, you need to grant RustDesk \"Record Audio\" permissions."),
        ("request_elevation_tip", "You can also request elevation if there is someone on the remote side."),
        ("Elevation Error", "Elevation error"),
        ("still_click_uac_tip", "Still requires the remote user to click OK on the UAC window of running RustDesk."),
        ("Request Elevation", "Request elevation"),
        ("wait_accept_uac_tip", "Please wait for the remote user to accept the UAC dialog."),
        ("Switch Sides", "Switch sides"),
        ("Default View Style", "Default view style"),
        ("Default Scroll Style", "Default scroll style"),
        ("Default Image Quality", "Default image quality"),
        ("Default Codec", "Default codec"),
        ("Other Default Options", "Other default options"),
        ("relay_hint_tip", "It may not be possible to connect directly; you can try connecting via relay. Additionally, if you want to use a relay on your first attempt, you can add the \"/r\" suffix to the ID or select the option \"Always connect via relay\" in the card of recent sessions if it exists."),
        ("RDP Settings", "RDP settings"),
        ("New Connection", "New connection"),
        ("Your Device", "Your device"),
        ("empty_recent_tip", "Oops, no recent sessions!\nTime to plan a new one."),
        ("empty_favorite_tip", "No favorite peers yet?\nLet's find someone to connect with and add it to your favorites!"),
        ("empty_lan_tip", "Oh no, it looks like we haven't discovered any peers yet."),
        ("empty_address_book_tip", "Oh dear, it appears that there are currently no peers listed in your address book."),
        ("Empty Username", "Empty username"),
        ("Empty Password", "Empty password"),
        ("identical_file_tip", "This file is identical with the peer's one."),
        ("show_monitors_tip", "Show monitors in toolbar"),
        ("View Mode", "View mode"),
        ("login_linux_tip", "You need to login to remote Linux account to enable a X desktop session"),
        ("verify_rustdesk_password_tip", "Verify RustDesk password"),
        ("remember_account_tip", "Remember this account"),
        ("os_account_desk_tip", "This account is used to login the remote OS and enable the desktop session in headless"),
        ("OS Account", "Os account"),
        ("another_user_login_title_tip", "Another user already logged in"),
        ("another_user_login_text_tip", "Disconnect"),
        ("xorg_not_found_title_tip", "Xorg not found"),
        ("xorg_not_found_text_tip", "Please install Xorg"),
        ("no_desktop_title_tip", "No desktop is available"),
        ("no_desktop_text_tip", "Please install GNOME desktop"),
        ("System Sound", "System sound"),
        ("Copy Fingerprint", "Copy fingerprint"),
        ("no fingerprints", "No fingerprints"),
        ("resolution_original_tip", "Original resolution"),
        ("resolution_fit_local_tip", "Fit local resolution"),
        ("resolution_custom_tip", "Custom resolution"),
        ("Accept and Elevate", "Accept and elevate"),
        ("accept_and_elevate_btn_tooltip", "Accept the connection and elevate UAC permissions."),
        ("clipboard_wait_response_timeout_tip", "Timed out waiting for copy response."),
        ("logout_tip", "Are you sure you want to log out?"),
        ("exceed_max_devices", "You have reached the maximum number of managed devices."),
        ("Change Password", "Change password"),
        ("Refresh Password", "Refresh password"),
        ("Grid View", "Grid view"),
        ("List View", "List view"),
        ("Toggle Tags", "Toggle tags"),
        ("pull_ab_failed_tip", "Failed to refresh address book"),
        ("push_ab_failed_tip", "Failed to sync address book to server"),
        ("synced_peer_readded_tip", "The devices that were present in the recent sessions will be synchronized back to the address book."),
        ("Change Color", "Change color"),
        ("Primary Color", "Primary color"),
        ("HSV Color", "HSV color"),
        ("Installation Successful!", "Installation successful!"),
        ("scam_title", "You May Be Being SCAMMED!"),
        ("scam_text1", "If you are on the phone with someone you DON'T know AND TRUST who has asked you to use RustDesk and start the service, do not proceed and hang up immediately."),
        ("scam_text2", "They are likely a scammer trying to steal your money or other private information."),
        ("auto_disconnect_option_tip", "Automatically close incoming sessions on user inactivity"),
        ("Connection failed due to inactivity", "Automatically disconnected due to inactivity"),
        ("upgrade_rustdesk_server_pro_to_{}_tip", "Please upgrade RustDesk Server Pro to version {} or newer!"),
        ("pull_group_failed_tip", "Failed to refresh group"),
        ("doc_fix_wayland", "https://rustdesk.com/docs/en/manual/linux/#x11-required"),
    ].iter().cloned().collect();
}
