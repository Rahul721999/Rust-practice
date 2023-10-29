mod app;
use app::App;
use yew::prelude::*;
use serde::Deserialize;

///
///  Define Struct Video
#[derive(Clone, PartialEq, Deserialize)]
struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}

///
/// Props of video List
#[derive(Properties, PartialEq)]
struct VideosListProps {
    videos: Vec<Video>,
    on_click: Callback<Video>,
}

///
/// VideoList fn component
#[function_component(VideosList)]
fn videos_list(VideosListProps { videos, on_click }: &VideosListProps) -> Html {
    // handle Onclick event
    let on_click = on_click.clone();
    videos.iter().map(|video|{
        let on_video_select = {
            let on_click = on_click.clone();
            let video = video.clone();
            Callback::from(move|_|{
                on_click.emit(video.clone())
            })
        };
        html!{
            <p key={video.id} onclick = {on_video_select}>{format!("{} : {}", video.speaker, video.title)}</p>
        }
    }).collect()
}

///
/// Props for video Details
#[derive(Properties, PartialEq)]
struct VideoDetailsProps {
    video: Video,
}

///
/// fn component of Video Details
#[function_component(VideoDetails)]
fn video_details(VideoDetailsProps { video }: &VideoDetailsProps) -> Html {
    html! {
        <div>
            <h3>{video.title.clone()}</h3>
            <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail"/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
