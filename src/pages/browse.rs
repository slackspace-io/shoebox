use leptos::logging::log;
use leptos::prelude::*;

#[component]
pub fn BrowsePage() -> impl IntoView {
    //get files
    let files = Resource::new_blocking(
        || (),
        |_| async move {get_files().await.unwrap() },
    );
    let fallback_message = &String::from("No files found");
//hello world
    view! {
        <h1>"Hello, world!"</h1>
    <Suspense
    fallback= move || {
        view! {
            <p>"Loading..."</p>
        }
    }>
    //list files
    {move || Suspend::new(async move {
    let async_files= files.get();
        async_files.iter().next().map(|file| {
            for f in file.iter() {
//                log!("File: {:?}", f);
            view! {
                <p>"File: "</p>
            };
            }
        })

    }
    )}
    </Suspense>
    }
}




#[server]
//show directories and files of a given path
pub async fn get_files() -> Result<Vec<String>, ServerFnError> {
    let path = "/home/dopey/videos";
    let mut files = Vec::new();
    // Iterate over entries in the specified directory
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if let Some(extension) = path.extension() {
                let ext = extension.to_string_lossy().to_lowercase();
                if matches!(ext.as_str(), "jpg" | "jpeg" | "png" | "gif") {
                    files.push(path.display().to_string());
                } else if matches!(ext.as_str(), "mp4" | "mkv" | "avi" | "mov") {
                    files.push(path.display().to_string());
                } else {
                    files.push(path.display().to_string());
                }
            }
        }
    }
    println!("{:?}", files);
    Ok(files)
}
