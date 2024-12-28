use leptos::prelude::*;








pub fn video_player  (video_url: String) -> impl IntoView {
    view! {
                            <div>
                                <p>{format!("{:?}", video_url)}</p>
                                <video controls width="600"
                                src={video_url}
                            >
                                    "Your browser does not support the video tag."
                                </video>
                            </div>
                        }

}
