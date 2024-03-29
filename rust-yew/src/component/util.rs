use yew::prelude::*;
use serde::Deserialize;

///
///  Define Struct Video
#[derive(Clone, PartialEq, Deserialize)]
pub struct Video {
    pub id: usize,
    pub title: String,
    pub speaker: String,
    pub url: String,
}

///
/// Props of video List
#[derive(Properties, PartialEq)]
pub struct VideosListProps {
    pub videos: Vec<Video>,
    pub on_click: Callback<Video>,
}

///
/// VideoList fn component
#[function_component(VideosList)]
pub fn videos_list(VideosListProps { videos, on_click }: &VideosListProps) -> Html {
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
pub struct VideoDetailsProps {
    pub video: Video,
}

///
/// fn component of Video Details
#[function_component(VideoDetails)]
pub fn video_details(VideoDetailsProps { video }: &VideoDetailsProps) -> Html {
    html! {
        <div>
            <h3>{video.title.clone()}</h3>
            <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail"/>
        </div>
    }
}