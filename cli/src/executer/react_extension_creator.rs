use std::collections::HashMap;

use crate::{generator::{save_bytes_as_image, create_folder, write_file}, hashmap};

// 将所有文本文件加载成静态字符串
const GITIGNORE: &'static str = include_str!("../../resources/template-global/.gitignore");
const TSCONFIG: &'static str = include_str!("../../resources/template-react-normal/tsconfig.json");
const TSCONFIG_NODE: &'static str = include_str!("../../resources/template-react-normal/tsconfig.node.json");
const README: &'static str = include_str!("../../resources/template-react-normal/README.md");
const COPY_JS: &'static str = include_str!("../../resources/template-react-normal/copy.js");
const OPTIONS_HTML: &'static str = include_str!("../../resources/template-react-normal/options.html");
const POPUP_HTML: &'static str = include_str!("../../resources/template-react-normal/popup.html");


// Load all images as static byte array
const PACK_IMAGE: &'static [u8] = include_bytes!("../../resources/template-global/.github/imgs/pack.png");
const PUBLIC_LOGO: &'static [u8] = include_bytes!("../../resources/template-global/public/images/ashley.png");
const FAVICON_ICO: &'static [u8] = include_bytes!("../../resources/template-global/public/favicon.ico");
const VITE_LOGO: &'static [u8] = include_bytes!("../../resources/template-global/src/assets/images/vite.png");
const REACT_LOGO: &'static [u8] = include_bytes!("../../resources/template-global/src/assets/images/react.png");
const MANIFEST_LOGO: &'static [u8] = include_bytes!("../../resources/template-global/src/assets/images/manifest.png");


/**
 * Create React Chrome Extension(Normal)
 */
pub fn create_react_normal_extension(project_name: &str, description: Option<&str>) {    
    // manifest.json
    const MANIFEST: &'static str = include_str!("../../resources/template-react-normal/public/manifest.json");

    // package.json
    const PACKAGE_JSON: &'static str = include_str!("../../resources/template-react-normal/src/package.json");

    // vite.config.ts
    const VITE_CONFIG: &'static str = include_str!("../../resources/template-react-normal/vite.config.ts");
    const VITE_BACKGROUND_CONFIG: &'static str = include_str!("../../resources/template-react-normal/vite.background.config.ts");
    const VITE_CONTENT_CONFIG: &'static str = include_str!("../../resources/template-react-normal/vite.content.config.ts");


    // src
    const BACKGROUND_TS: &'static str = include_str!("../../resources/template-react-normal/src/background/index.ts");
    const CONTENT_TS: &'static str = include_str!("../../resources/template-react-normal/src/content/index.ts");
    const CONSTANTS_TS: &'static str = include_str!("../../resources/template-react-normal/src/global/constants.ts");
    const INDEX_STYL: &'static str = include_str!("../../resources/template-react-normal/src/index.styl");
    const VITE_ENV_D_TS: &'static str = include_str!("../../resources/template-react-normal/src/vite-env.d.ts");

    // src/hooks
    const USE_BADGE: &'static str = include_str!("../../resources/template-react-normal/src/hooks/useBadge.ts");
    const USE_CHROME: &'static str = include_str!("../../resources/template-react-normal/src/hooks/useChrome.ts");
    const USE_CONTEXT_MENU: &'static str = include_str!("../../resources/template-react-normal/src/hooks/useContextMenu.ts");
    const USE_NOTIFICATION: &'static str = include_str!("../../resources/template-react-normal/src/hooks/useNotification.ts");

    // src/options
    const OPTIONS_INDEX: &'static str = include_str!("../../resources/template-react-normal/src/options/index.tsx");
    const OPTIONS_MAIN: &'static str = include_str!("../../resources/template-react-normal/src/options/main.tsx");
    const OPTIONS_STYL: &'static str = include_str!("../../resources/template-react-normal/src/options/options.styl");

    // src/popup
    const COUNTER_INDEX: &'static str = include_str!("../../resources/template-react-normal/src/popup/counter/index.tsx");
    const COUNTER_STYL: &'static str = include_str!("../../resources/template-react-normal/src/popup/counter/counter.styl");
    const POPUP_INDEX: &'static str = include_str!("../../resources/template-react-normal/src/popup/index.tsx");
    const POPUP_MAIN: &'static str = include_str!("../../resources/template-react-normal/src/popup/main.tsx");
    const POPUP_STYL: &'static str = include_str!("../../resources/template-react-normal/src/popup/popup.styl");

    println!("Creating Vite + React project...");

    // create folders
    let folder_sequence: Vec<String> = vec![
        project_name.to_string(),
        format!("{}/.github", project_name),
        format!("{}/.github/imgs", project_name),
        format!("{}/public", project_name),
        format!("{}/src", project_name),
        format!("{}/public/images", project_name),
        format!("{}/src/assets", project_name),
        format!("{}/src/assets/images", project_name),
        format!("{}/src/background", project_name),
        format!("{}/src/content", project_name),
        format!("{}/src/global", project_name),
        format!("{}/src/hooks", project_name),
        format!("{}/src/options", project_name),
        format!("{}/src/popup", project_name),
        format!("{}/src/popup/counter", project_name)
    ];

    for path in folder_sequence {
        create_folder(&path);
    }
    

    // Dynamically generate several properties of manifest.json
    let mut manifest_json = MANIFEST.to_string();
    manifest_json = manifest_json.replace("${app_name}", project_name);

    match description {
        Some(content) => {
            manifest_json = manifest_json.replace("${app_description}", content);
        },
        None => {
            manifest_json = manifest_json.replace("${app_description}", "");
        }
    }

    // Dynamically generate several properties of package.json
    let mut package_json: String = PACKAGE_JSON.to_string();
    package_json = package_json.replace("app_name", project_name);

    
    // write texts
    let textfile_write_map: HashMap<String, String> = hashmap![
        manifest_json => format!("{}/public/manifest.json", project_name),
        VITE_CONFIG.to_string() => format!("{}/vite.config.ts", project_name),
        VITE_BACKGROUND_CONFIG.to_string() => format!("{}/vite.background.config.ts", project_name),
        VITE_CONTENT_CONFIG.to_string() => format!("{}/vite.content.config.ts", project_name),

        package_json => format!("{}/package.json", project_name),
        GITIGNORE.to_string() => format!("{}/.gitignore", project_name),
        TSCONFIG.to_string() => format!("{}/tsconfig.json", project_name),
        TSCONFIG_NODE.to_string() => format!("{}/tsconfig.node.json", project_name),
        README.to_string() => format!("{}/README.md", project_name),
        COPY_JS.to_string() => format!("{}/copy.js", project_name),
        OPTIONS_HTML.to_string() => format!("{}/options.html", project_name),
        POPUP_HTML.to_string() => format!("{}/popup.html", project_name),

        INDEX_STYL.to_string() => format!("{}/src/index.styl", project_name),
        VITE_ENV_D_TS.to_string() => format!("{}/src/vite-env.d.ts", project_name),
        BACKGROUND_TS.to_string() => format!("{}/src/background/index.ts", project_name),
        CONTENT_TS.to_string() => format!("{}/src/content/index.ts", project_name),
        CONSTANTS_TS.to_string() => format!("{}/src/global/constants.ts", project_name),
        USE_BADGE.to_string() => format!("{}/src/hooks/useBadge.ts", project_name),
        USE_CHROME.to_string() => format!("{}/src/hooks/useChrome.ts", project_name),
        USE_CONTEXT_MENU.to_string() => format!("{}/src/hooks/useContextMenu.ts", project_name),
        USE_NOTIFICATION.to_string() => format!("{}/src/hooks/useNotification.ts", project_name),

        OPTIONS_INDEX.to_string() => format!("{}/src/options/index.tsx", project_name),
        OPTIONS_MAIN.to_string() => format!("{}/src/options/main.tsx", project_name),
        OPTIONS_STYL.to_string() => format!("{}/src/options/options.styl", project_name),

        COUNTER_INDEX.to_string() => format!("{}/src/popup/counter/index.tsx", project_name),
        COUNTER_STYL.to_string() => format!("{}/src/popup/counter/counter.styl", project_name),
        POPUP_INDEX.to_string() => format!("{}/src/popup/index.tsx", project_name),
        POPUP_MAIN.to_string() => format!("{}/src/popup/main.tsx", project_name),
        POPUP_STYL.to_string() => format!("{}/src/popup/popup.styl", project_name)
    ];

    for (content, output_path) in &textfile_write_map {
        write_file(&output_path, &content);
    };


    // draw images
    let image_write_map: HashMap<&[u8], String> = hashmap![
        PACK_IMAGE => format!("{}/.github/imgs/pack.png", project_name),
        PUBLIC_LOGO => format!("{}/public/images/ashley.png", project_name),
        FAVICON_ICO => format!("{}/public/favicon.ico", project_name),
        MANIFEST_LOGO => format!("{}/src/assets/images/manifest.png", project_name),
        REACT_LOGO => format!("{}/src/assets/images/react.png", project_name),
        VITE_LOGO => format!("{}/src/assets/images/vite.png", project_name)
    ];

    for (bytes_ref, output_path) in &image_write_map {
        save_bytes_as_image(*bytes_ref, output_path);
    }
}


/**
 * Create React Chrome Extension(Lite)
 */
pub fn create_react_lite_extension(project_name: &str, description: Option<&str>) {    
    // manifest.json
    const MANIFEST: &'static str = include_str!("../../resources/template-react-lite/public/manifest.json");

    // package.json
    const PACKAGE_JSON: &'static str = include_str!("../../resources/template-react-lite/src/package.json");

    // vite.config.ts
    const VITE_CONFIG: &'static str = include_str!("../../resources/template-react-lite/vite.config.ts");

    // src
    const CONSTANTS_TS: &'static str = include_str!("../../resources/template-react-lite/src/global/constants.ts");
    const INDEX_STYL: &'static str = include_str!("../../resources/template-react-lite/src/index.styl");
    const VITE_ENV_D_TS: &'static str = include_str!("../../resources/template-react-lite/src/vite-env.d.ts");

    // src/hooks
    const USE_BADGE: &'static str = include_str!("../../resources/template-react-lite/src/hooks/useBadge.ts");
    const USE_CHROME: &'static str = include_str!("../../resources/template-react-lite/src/hooks/useChrome.ts");
    const USE_CONTEXT_MENU: &'static str = include_str!("../../resources/template-react-lite/src/hooks/useContextMenu.ts");
    const USE_NOTIFICATION: &'static str = include_str!("../../resources/template-react-lite/src/hooks/useNotification.ts");

    // src/options
    const OPTIONS_INDEX: &'static str = include_str!("../../resources/template-react-lite/src/options/index.tsx");
    const OPTIONS_MAIN: &'static str = include_str!("../../resources/template-react-lite/src/options/main.tsx");
    const OPTIONS_STYL: &'static str = include_str!("../../resources/template-react-lite/src/options/options.styl");

    // src/popup
    const COUNTER_INDEX: &'static str = include_str!("../../resources/template-react-lite/src/popup/counter/index.tsx");
    const COUNTER_STYL: &'static str = include_str!("../../resources/template-react-lite/src/popup/counter/counter.styl");
    const POPUP_INDEX: &'static str = include_str!("../../resources/template-react-lite/src/popup/index.tsx");
    const POPUP_MAIN: &'static str = include_str!("../../resources/template-react-lite/src/popup/main.tsx");
    const POPUP_STYL: &'static str = include_str!("../../resources/template-react-lite/src/popup/popup.styl");

    println!("Creating Vite + React project...");

    // create folders
    let folder_sequence: Vec<String> = vec![
        project_name.to_string(),
        format!("{}/.github", project_name),
        format!("{}/.github/imgs", project_name),
        format!("{}/public", project_name),
        format!("{}/src", project_name),
        format!("{}/public/images", project_name),
        format!("{}/src/assets", project_name),
        format!("{}/src/assets/images", project_name),
        format!("{}/src/global", project_name),
        format!("{}/src/hooks", project_name),
        format!("{}/src/options", project_name),
        format!("{}/src/popup", project_name),
        format!("{}/src/popup/counter", project_name)
    ];

    for path in folder_sequence {
        create_folder(&path);
    }
    

    // Dynamically generate several properties of manifest.json
    let mut manifest_json = MANIFEST.to_string();
    manifest_json = manifest_json.replace("${app_name}", project_name);

    match description {
        Some(content) => {
            manifest_json = manifest_json.replace("${app_description}", content);
        },
        None => {
            manifest_json = manifest_json.replace("${app_description}", "");
        }
    }

    // Dynamically generate several properties of package.json
    let mut package_json: String = PACKAGE_JSON.to_string();
    package_json = package_json.replace("app_name", project_name);

    
    // write texts
    let textfile_write_map: HashMap<String, String> = hashmap![
        manifest_json => format!("{}/public/manifest.json", project_name),
        VITE_CONFIG.to_string() => format!("{}/vite.config.ts", project_name),

        package_json => format!("{}/package.json", project_name),
        GITIGNORE.to_string() => format!("{}/.gitignore", project_name),
        TSCONFIG.to_string() => format!("{}/tsconfig.json", project_name),
        TSCONFIG_NODE.to_string() => format!("{}/tsconfig.node.json", project_name),
        README.to_string() => format!("{}/README.md", project_name),
        OPTIONS_HTML.to_string() => format!("{}/options.html", project_name),
        POPUP_HTML.to_string() => format!("{}/popup.html", project_name),

        INDEX_STYL.to_string() => format!("{}/src/index.styl", project_name),
        VITE_ENV_D_TS.to_string() => format!("{}/src/vite-env.d.ts", project_name),
        CONSTANTS_TS.to_string() => format!("{}/src/global/constants.ts", project_name),
        USE_BADGE.to_string() => format!("{}/src/hooks/useBadge.ts", project_name),
        USE_CHROME.to_string() => format!("{}/src/hooks/useChrome.ts", project_name),
        USE_CONTEXT_MENU.to_string() => format!("{}/src/hooks/useContextMenu.ts", project_name),
        USE_NOTIFICATION.to_string() => format!("{}/src/hooks/useNotification.ts", project_name),

        OPTIONS_INDEX.to_string() => format!("{}/src/options/index.tsx", project_name),
        OPTIONS_MAIN.to_string() => format!("{}/src/options/main.tsx", project_name),
        OPTIONS_STYL.to_string() => format!("{}/src/options/options.styl", project_name),

        COUNTER_INDEX.to_string() => format!("{}/src/popup/counter/index.tsx", project_name),
        COUNTER_STYL.to_string() => format!("{}/src/popup/counter/counter.styl", project_name),
        POPUP_INDEX.to_string() => format!("{}/src/popup/index.tsx", project_name),
        POPUP_MAIN.to_string() => format!("{}/src/popup/main.tsx", project_name),
        POPUP_STYL.to_string() => format!("{}/src/popup/popup.styl", project_name)
    ];

    for (content, output_path) in &textfile_write_map {
        write_file(&output_path, &content);
    };


    // draw images
    let image_write_map: HashMap<&[u8], String> = hashmap![
        PACK_IMAGE => format!("{}/.github/imgs/pack.png", project_name),
        PUBLIC_LOGO => format!("{}/public/images/ashley.png", project_name),
        FAVICON_ICO => format!("{}/public/favicon.ico", project_name),
        MANIFEST_LOGO => format!("{}/src/assets/images/manifest.png", project_name),
        REACT_LOGO => format!("{}/src/assets/images/react.png", project_name),
        VITE_LOGO => format!("{}/src/assets/images/vite.png", project_name)
    ];

    for (bytes_ref, output_path) in &image_write_map {
        save_bytes_as_image(*bytes_ref, output_path);
    }
}