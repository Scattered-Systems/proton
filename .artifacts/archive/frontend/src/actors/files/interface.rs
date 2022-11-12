/*
    Appellation: interface <files>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::Msg;

use gloo::file::{callbacks::FileReader, File};
use std::collections::HashMap;
use web_sys::{Event, HtmlInputElement};
use yew::{html, html::TargetCast, Component, Context, Html};

pub struct Model {
    readers: HashMap<String, FileReader>,
    files: Vec<String>,
    read_bytes: bool,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            readers: HashMap::default(),
            files: vec![],
            read_bytes: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Loaded(file_name, data) => {
                let info = format!("file_name: {}, data: {:?}", file_name, data);
                self.files.push(info);
                self.readers.remove(&file_name);
                true
            }
            Msg::LoadedBytes(file_name, data) => {
                let info = format!("file_name: {}, data: {:?}", file_name, data);
                self.files.push(info);
                self.readers.remove(&file_name);
                true
            }
            Msg::Files(files, bytes) => {
                for file in files.into_iter() {
                    let file_name = file.name();
                    let task = {
                        let file_name = file_name.clone();
                        let link = ctx.link().clone();

                        if bytes {
                            gloo::file::callbacks::read_as_bytes(&file, move |res| {
                                link.send_message(Msg::LoadedBytes(
                                    file_name,
                                    res.expect("failed to read file"),
                                ))
                            })
                        } else {
                            gloo::file::callbacks::read_as_text(&file, move |res| {
                                link.send_message(Msg::Loaded(
                                    file_name,
                                    res.unwrap_or_else(|e| e.to_string()),
                                ))
                            })
                        }
                    };
                    self.readers.insert(file_name, task);
                }
                true
            }
            Msg::ToggleReadBytes => {
                self.read_bytes = !self.read_bytes;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let flag = self.read_bytes;
        let bg = "bg-transparent";
        html! {
            <div class="bg-transparent flex flex-auto min-h-full items-center justify-center">
                <div class="flex flex-initial">
                    <form class="">
                        <div class="form-control">
                            <label class="ml-3 text-base" for="fileInputBtn">{ "Choose a file to upload to see the uploaded bytes" }</label>
                            <input class="rounded-full" id="fileInputBtn" type="file" multiple=true onchange={ctx.link().callback(move |e: Event| {
                                    let mut result = Vec::new();
                                    let input: HtmlInputElement = e.target_unchecked_into();

                                    if let Some(files) = input.files() {
                                        let files = js_sys::try_iter(&files)
                                            .unwrap()
                                            .unwrap()
                                            .map(|v| web_sys::File::from(v.unwrap()))
                                            .map(File::from);
                                        result.extend(files);
                                    }
                                    Msg::Files(result, flag)
                                })}
                            />
                        </div>
                        <div class="form-check">
                            <input
                                class="rounded"
                                id="flexCheckChecked"
                                type="checkbox"
                                checked={flag}
                                onclick={ctx.link().callback(|_| Msg::ToggleReadBytes)}
                                value=""
                            />
                            <label class="form-check-label" for="flexCheckChecked">
                                {"Read Bytes"}
                            </label>
                        </div>
                    </form>
                </div>
                // Display the uploaded files
                <div class="card my-3 p-3">
                    <div class="card-body">
                        <ul class="list-none">
                            { for self.files.iter().map(|f| Self::view_file(f)) }
                        </ul>
                    </div>
                </div>
            </div>
        }
    }
}

impl Model {
    fn view_file(data: &str) -> Html {
        html! {
            <li>{ data }</li>
        }
    }
}
