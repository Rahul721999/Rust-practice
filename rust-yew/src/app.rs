use crate::{Video, VideoDetails, VideosList};
use yew::prelude::*;
use gloo_net::http::Request;

#[function_component(App)]
pub fn app() -> Html {

    let videos = use_state(|| vec![]);
    {
        let videos = videos.clone();
        use_effect_with((), move |_| {
            let videos = videos.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_videos: Vec<Video> = Request::get("/tutorial/data.json")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                videos.set(fetched_videos);
            });
            || ()
        });
    }
    // set selected video
    let selected_video = use_state(|| None);

    // set onclick event for selected video
    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |v: Video|{
            selected_video.set(Some(v))
        })
    };

    // details of the selected vidoe
    let details: Option<yew::virtual_dom::VNode> = selected_video.as_ref().map(|v| 
        html! {
            <VideoDetails video={v.clone()}/> 
    });

    // return the html
    html! {
        <>
             <h1>{"RustCont testing"}</h1>
             <div>
             <h3>{"Videos to watch"}</h3>
                  <VideosList videos = {(*videos).clone()} on_click={on_video_select.clone()}/>
             </div>

             {for details} // Special syntax for
        </>
    }
}
