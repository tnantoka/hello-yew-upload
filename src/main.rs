use yew::prelude::*;

#[derive(Clone, PartialEq)]
struct UploadedFile {
    name: String,
    content: String,
}

impl UploadedFile {
    fn new(name: String, content: String) -> Self {
        Self { name, content }
    }
}

#[function_component(App)]
fn app() -> Html {
    let uploaded_file = use_state(|| UploadedFile::new("none".to_string(), "".to_string()));
    let file_reader = use_state(|| None);
    let can_drop = use_state(|| false);

    let on_drop = {
        let uploaded_file = uploaded_file.clone();

        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            match e.data_transfer() {
                Some(data_transfer) => {
                    let file = data_transfer.files().unwrap().item(0).unwrap();
                    let file_name = file.name();

                    let file = gloo_file::File::from(file);
                    let uploaded_file = uploaded_file.clone();
                    let reader = gloo_file::callbacks::read_as_data_url(&file, move |res| {
                        web_sys::console::log_1(&"".into());
                        uploaded_file.set(UploadedFile::new(file_name, res.unwrap()));
                    });
                    file_reader.set(Some(reader));
                }
                None => {
                    uploaded_file.set(UploadedFile::new("none".to_string(), "".to_string()));
                }
            }
        })
    };

    html! {
        <div>
            <div
                style="height: 5rem; padding: 1rem; background: lightgray; border: 1px solid gray;"
                ondragenter={
                   let can_drop = can_drop.clone();

                   Callback::from(move |e: DragEvent| {
                      e.prevent_default();
                      can_drop.set(true);
                   })
                }
                ondragleave={
                   let can_drop = can_drop.clone();

                    Callback::from(move |_| {
                    can_drop.set(false);
                })}
                ondragover={|e: DragEvent| e.prevent_default()}
                ondrop={on_drop}
            >
                {format!("{} here", if *can_drop { "Drop" } else { "Drag" })}
            </div>
            <h4>{(*uploaded_file).clone().name}</h4>
            <img src={(*uploaded_file).clone().content} />
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
