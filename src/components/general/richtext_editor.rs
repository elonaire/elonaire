use icondata as IconId;
use leptos::task::spawn_local;
use leptos::wasm_bindgen::JsCast;
use leptos::{ev, prelude::*};
use markdown;
use reactive_stores::Store;
use web_sys::{Element, FormData, HtmlDivElement, HtmlInputElement, Node, window};

use crate::components::forms::input::{InputField, InputFieldType};
use crate::components::forms::select::{SelectInput, SelectOption};
use crate::components::general::button::BasicButton;
use crate::data::context::store::{AppStateContext, AppStateContextStoreFields};
use crate::data::models::general::{
    acl::{AuthInfoStoreFields, UserInfoStoreFields},
    files::UploadedFileResponse,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExtraFormatingOption {
    MarkdownUpload,
    ImageUpload,
    Heading,
    InlineCode,
    CodeBlock,
    Lists,
}

#[component]
pub fn RichTextEditor(
    #[prop(optional, default = Callback::new(|_| {}))] on_input: Callback<String>,
    #[prop(into, optional, default = RwSignal::new("<p><br></p>".into()))]
    initial_content: RwSignal<String>,
    #[prop(into, optional)] id_attr: String,
    #[prop(into, optional)] name: String,
    #[prop(into, optional)] placeholder: String,
    #[prop(optional, default = Vec::new())] extra_formating_options: Vec<ExtraFormatingOption>,
) -> impl IntoView {
    let editor_ref = NodeRef::new();
    let file_input_ref = NodeRef::new();
    let current_state = expect_context::<Store<AppStateContext>>();
    let font_options = RwSignal::new(vec![
        SelectOption::new("p", "Normal"),
        SelectOption::new("h1", "H1"),
        SelectOption::new("h2", "H2"),
        SelectOption::new("h3", "H3"),
        SelectOption::new("h4", "H4"),
        SelectOption::new("h5", "H5"),
        SelectOption::new("h6", "H6"),
    ]);
    let language_options = RwSignal::new(vec![
        SelectOption::new("plaintext", "Plain Text"),
        SelectOption::new("rust", "Rust"),
        SelectOption::new("surql", "SurrealQL"),
        SelectOption::new("javascript", "JavaScript"),
        SelectOption::new("typescript", "TypeScript"),
        SelectOption::new("python", "Python"),
        SelectOption::new("java", "Java"),
        SelectOption::new("cpp", "C++"),
        SelectOption::new("c", "C"),
        SelectOption::new("csharp", "C#"),
        SelectOption::new("go", "Go"),
        SelectOption::new("ruby", "Ruby"),
        SelectOption::new("php", "PHP"),
        SelectOption::new("html", "HTML"),
        SelectOption::new("css", "CSS"),
        SelectOption::new("json", "JSON"),
        SelectOption::new("sql", "SQL"),
        SelectOption::new("bash", "Bash"),
        SelectOption::new("yaml", "YAML"),
        SelectOption::new("markdown", "Markdown"),
    ]);
    let last_enter_empty = RwSignal::new(false);
    let show_language_picker = RwSignal::new(false);
    let (tracked_content, set_tracked_content) = signal(String::new());
    let md_file_input_ref = NodeRef::new();

    // Track active formatting states
    let is_bold = RwSignal::new(false);
    let is_italic = RwSignal::new(false);
    let is_underline = RwSignal::new(false);
    let is_strikethrough = RwSignal::new(false);
    let is_inline_code = RwSignal::new(false);
    let is_code_block = RwSignal::new(false);
    let is_ordered_list = RwSignal::new(false);
    let is_unordered_list = RwSignal::new(false);

    // Create style functions for buttons
    let bold_style = Memo::new(move |_| {
        if is_bold.get() {
            "bg-primary text-contrast-white".into()
        } else {
            "hover:bg-light-gray".into()
        }
    });
    let italic_style = Memo::new(move |_| {
        if is_italic.get() {
            "bg-primary text-contrast-white".into()
        } else {
            "hover:bg-light-gray".into()
        }
    });
    let underline_style = Memo::new(move |_| {
        if is_underline.get() {
            "bg-primary text-contrast-white".into()
        } else {
            "hover:bg-light-gray".into()
        }
    });
    let strikethrough_style = Memo::new(move |_| {
        if is_strikethrough.get() {
            "bg-primary text-contrast-white".into()
        } else {
            "hover:bg-light-gray".into()
        }
    });
    let inline_code_style = Memo::new(move |_| {
        if is_inline_code.get() {
            "bg-primary text-contrast-white".into()
        } else {
            "hover:bg-light-gray".into()
        }
    });
    let code_block_style = Memo::new(move |_| {
        if is_code_block.get() {
            "bg-primary text-contrast-white".into()
        } else {
            "hover:bg-light-gray".into()
        }
    });
    let ordered_list_style = Memo::new(move |_| {
        if is_ordered_list.get() {
            "bg-primary text-contrast-white".into()
        } else {
            "hover:bg-light-gray".into()
        }
    });
    let unordered_list_style = Memo::new(move |_| {
        if is_unordered_list.get() {
            "bg-primary text-contrast-white".into()
        } else {
            "hover:bg-light-gray".into()
        }
    });

    let update_button_states = move || {
        is_bold.set(cursor_inside("b").is_some());
        is_italic.set(cursor_inside("i").is_some());
        is_underline.set(cursor_inside("u").is_some());
        is_strikethrough.set(cursor_inside("s").is_some());
        is_inline_code.set(cursor_inside("code").is_some() && current_code_block().is_none());
        is_code_block.set(current_code_block().is_some());

        if let Some((list, _)) = current_list_item() {
            let tag = list.tag_name().to_lowercase();
            is_ordered_list.set(tag == "ol");
            is_unordered_list.set(tag == "ul");
        } else {
            is_ordered_list.set(false);
            is_unordered_list.set(false);
        }
    };

    let toggle_style = move |tag_name: &'static str| {
        if let Some(el) = cursor_inside(tag_name) {
            // Toggle off: insert a zero-width space after the element and move cursor there
            if let Some(doc) = window().and_then(|w| w.document()) {
                if let Ok(Some(selection)) = doc.get_selection() {
                    // Create a zero-width space to break out of the formatting
                    let space = doc.create_text_node("\u{200B}");

                    // Insert it after the styled element
                    if let Some(parent) = el.parent_node() {
                        if let Some(next_sibling) = el.next_sibling() {
                            parent.insert_before(&space, Some(&next_sibling)).ok();
                        } else {
                            parent.append_child(&space).ok();
                        }

                        // Move cursor after the zero-width space
                        if let Ok(new_range) = doc.create_range() {
                            new_range.set_start(&space, 1).ok();
                            new_range.set_end(&space, 1).ok();
                            selection.remove_all_ranges().ok();
                            selection.add_range(&new_range).ok();
                        }
                    }
                }
            }
        } else {
            // Toggle on: Wrap selection or insert at cursor
            if let Some(doc) = window().and_then(|w| w.document()) {
                if let Ok(Some(selection)) = doc.get_selection() {
                    if let Ok(range) = selection.get_range_at(0) {
                        if let Ok(element) = doc.create_element(tag_name) {
                            if range.collapsed() {
                                // Insert zero-width space so caret can live inside the tag
                                let text = doc.create_text_node("\u{200B}");
                                let _ = element.append_child(&text);
                                let _ = range.insert_node(&element);
                                if let Ok(new_range) = doc.create_range() {
                                    // Place cursor AFTER the zero-width space (inside tag)
                                    let _ = new_range.set_start(&text, 1);
                                    let _ = new_range.set_end(&text, 1);
                                    let _ = selection.remove_all_ranges();
                                    let _ = selection.add_range(&new_range);
                                }
                            } else {
                                // Wrap selected text
                                let contents = range.clone_contents().ok();
                                range.delete_contents().ok();
                                if let Some(contents) = contents {
                                    element.append_child(&contents).ok();
                                }
                                range.insert_node(&element).ok();

                                // Move cursor to the end of the new element
                                if let Ok(new_range) = doc.create_range() {
                                    new_range.select_node_contents(&element).ok();
                                    new_range.collapse_with_to_start(false);
                                    let _ = selection.remove_all_ranges();
                                    let _ = selection.add_range(&new_range);
                                }
                            }
                        }
                    }
                }
            }
        }
        update_button_states();
    };

    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        if ev.key() != "Enter" {
            return;
        }

        if let Some((pre, code)) = current_code_block() {
            ev.prevent_default();
            handle_code_enter(&pre, &code, &last_enter_empty);
        } else if let Some((list, li)) = current_list_item() {
            ev.prevent_default();
            handle_list_enter(&list, &li);
        } else {
            ev.prevent_default();
            if let Some(editor) = editor_ref.get() {
                insert_clean_line(&editor);
            }
        }

        update_button_states();
    };

    // Update button states on selection change
    let on_selection_change = move |_: ev::MouseEvent| {
        update_button_states();
    };

    let on_keyup = move |_: ev::KeyboardEvent| {
        update_button_states();
    };

    let bold = Callback::new(move |_| toggle_style("b"));
    let italic = Callback::new(move |_| toggle_style("i"));
    let underline = Callback::new(move |_| toggle_style("u"));
    let strikethrough = Callback::new(move |_| toggle_style("s"));

    // Fixed inline code handler - check if we're in a code block first
    let inline_code = Callback::new(move |_| {
        // Don't toggle inline code if we're in a code block
        if current_code_block().is_none() {
            toggle_style("code");
        }
    });

    let code_block = Callback::new(move |_| {
        // If we're already in a code block, exit it
        if let Some((pre, code)) = current_code_block() {
            exit_code_block(&pre);
            show_language_picker.set(false);
        } else {
            insert_code_block();
            show_language_picker.set(true);
        }
        update_button_states();
    });

    // Insert image callback
    let insert_image = Callback::new(move |_| {
        if let Some(input) = file_input_ref.get() as Option<HtmlInputElement> {
            let _ = input.click();
        }
    });

    // Handle file selection
    let on_file_change = Callback::new(move |_ev: ev::Event| {
        if let Some(file_input) = file_input_ref.to_owned().get() as Option<HtmlInputElement> {
            if let Ok(files_form_data) = FormData::new() {
                if let Some(filelist) = file_input.files() {
                    for i in 0..filelist.length() {
                        if let Some(file) = filelist.item(i) {
                            if let Err(e) = files_form_data.append_with_blob("file", &file) {
                                leptos::logging::error!("Failed to append Blob: {:?}", e);
                            };
                        }
                    }
                }

                spawn_local(async move {
                    match gloo_net::http::Request::post("http://localhost:8080/api/files/upload")
                        .header(
                            "Authorization",
                            format!(
                                "Bearer {}",
                                current_state.user().auth_info().token().get_untracked()
                            )
                            .as_str(),
                        )
                        .body(files_form_data)
                        .unwrap()
                        .send()
                        .await
                    {
                        Ok(response) => {
                            match response.json::<Vec<UploadedFileResponse>>().await {
                                Ok(uploaded_files) => {
                                    if let Some(doc) = window().and_then(|w| w.document()) {
                                        if let Ok(Some(selection)) = doc.get_selection() {
                                            if let Ok(range) = selection.get_range_at(0) {
                                                let img = doc.create_element("img").unwrap();
                                                img.set_attribute(
                                                    "src",
                                                    &format!(
                                                        "http://localhost:3001/view/{}",
                                                        uploaded_files[0].file_name
                                                    ),
                                                )
                                                .unwrap();
                                                img.set_attribute(
                                                    "style",
                                                    "max-width: 100%; height: auto;",
                                                )
                                                .unwrap();

                                                range.delete_contents().ok();
                                                range.insert_node(&img).ok();

                                                // Move cursor after the image
                                                if let Ok(new_range) = doc.create_range() {
                                                    new_range.set_start_after(&img).ok();
                                                    new_range.set_end_after(&img).ok();
                                                    selection.remove_all_ranges().ok();
                                                    selection.add_range(&new_range).ok();
                                                }
                                            }
                                        }
                                    }
                                }
                                Err(err) => {
                                    leptos::logging::error!(
                                        "Failed to parse uploaded file response: {:?}",
                                        err
                                    );
                                }
                            };
                        }
                        Err(err) => {
                            leptos::logging::error!("Failed to upload files: {:?}", err);
                        }
                    };
                });
            };
        };
    });

    // Add this effect to position the cursor
    Effect::new(move |_| {
        if let Some(editor) = editor_ref.get() {
            if let Some(doc) = window().and_then(|w| w.document()) {
                if let Ok(Some(selection)) = doc.get_selection() {
                    if let Some(p) = editor.first_element_child() {
                        let range = doc.create_range().unwrap();
                        range.set_start(&p, 0).ok();
                        range.set_end(&p, 0).ok();
                        selection.remove_all_ranges().ok();
                        selection.add_range(&range).ok();
                    }
                }
            }
        }
    });

    let apply_heading = Callback::new(move |ev: ev::Event| {
        let tag = event_target_value(&ev);
        if let Some(editor) = editor_ref.get() {
            if let Some(doc) = window().and_then(|w| w.document()) {
                if let Ok(Some(selection)) = doc.get_selection() {
                    if let Ok(range) = selection.get_range_at(0) {
                        let mut node = range.start_container().unwrap();
                        let editor_node: &Node = editor.as_ref();
                        loop {
                            if let Some(el) = node.dyn_ref::<Element>() {
                                if let Some(parent) = el.parent_element() {
                                    if parent.is_same_node(Some(editor_node)) {
                                        if ["p", "h1", "h2", "h3", "h4", "h5", "h6"]
                                            .contains(&el.tag_name().to_lowercase().as_str())
                                        {
                                            let new_el = doc.create_element(&tag).unwrap();
                                            while let Some(child) = el.first_child() {
                                                new_el.append_child(&child).ok();
                                            }
                                            parent.replace_child(&new_el, el).ok();
                                            let new_range = doc.create_range().unwrap();
                                            new_range.select_node_contents(&new_el).ok();
                                            new_range.collapse();
                                            selection.remove_all_ranges().ok();
                                            selection.add_range(&new_range).ok();
                                            break;
                                        }
                                    }
                                }
                            }
                            match node.parent_node() {
                                Some(p) => node = p,
                                None => break,
                            }
                        }
                    }
                }
            }
        }
    });

    let apply_language = Callback::new(move |ev: ev::Event| {
        let lang = event_target_value(&ev);
        if let Some((pre, code)) = current_code_block() {
            code.set_attribute("class", &format!("language-{}", lang))
                .ok();
        }
        show_language_picker.set(false);
    });

    let handle_on_input = move |_: ev::Event| {
        if let Some(editor) = editor_ref.get() {
            leptos::logging::log!("This is changing");
            set_tracked_content.set(editor.inner_html());
        }
    };

    Effect::new(move |_| {
        let changed_value = tracked_content.get();
        on_input.run(changed_value);
    });

    let upload_md = Callback::new(move |_| {
        if let Some(input) = md_file_input_ref.get() as Option<HtmlInputElement> {
            let _ = input.click();
        }
    });

    let on_md_file_change = Callback::new(move |_ev: ev::Event| {
        if let Some(file_input) = md_file_input_ref.get() as Option<HtmlInputElement> {
            if let Some(files) = file_input.files() {
                if let Some(file) = files.item(0) {
                    spawn_local(async move {
                        match gloo_file::futures::read_as_text(&file.into()).await {
                            Ok(markdown_content) => {
                                // Parse markdown to HTML using markdown crate
                                let html_output = markdown::to_html(&markdown_content);

                                // Update the editor content
                                initial_content.set(html_output.clone());

                                // Also update the tracked content
                                set_tracked_content.set(html_output);

                                leptos::logging::log!("Markdown file loaded successfully");
                            }
                            Err(err) => {
                                leptos::logging::error!("Failed to read markdown file: {:?}", err);
                            }
                        }
                    });
                }
            }
        }
    });

    // Ordered list
    let ordered_list = Callback::new(move |_| {
        if let Some(editor) = editor_ref.get() {
            if let Some((list, _)) = current_list_item() {
                // If we're in a list, exit it
                exit_list(&editor, &list);
            } else {
                // Otherwise, insert a new ordered list
                insert_list(&editor, "ol");
            }
        }
        update_button_states();
    });

    // Unordered list
    let unordered_list = Callback::new(move |_| {
        if let Some(editor) = editor_ref.get() {
            if let Some((list, _)) = current_list_item() {
                // If we're in a list, exit it
                exit_list(&editor, &list);
            } else {
                // Otherwise, insert a new unordered list
                insert_list(&editor, "ul");
            }
        }
        update_button_states();
    });

    view! {
        <div class="border-[1px] border-light-gray rounded-[5px]">
            // Toolbar
            <div class="flex gap-2 items-center flex-wrap border-b-[1px] border-light-gray p-[10px]">
                {
                    // if extra_formating_options.contains(&ExtraFormatingOption::Heading) {
                    //     Some(
                    //         view!{
                    //             <SelectInput id_attr="font-sizes" options=font_options onchange=apply_heading />
                    //         }
                    //     )
                    // } else {
                    //     None
                    // }
                    extra_formating_options.contains(&ExtraFormatingOption::Heading).then(|| view!{
                        <SelectInput id_attr="font-sizes" options=font_options onchange=apply_heading />
                    })
                }
                <BasicButton
                    icon=Some(IconId::FiBold)
                    icon_before=true
                    onclick=bold
                    style_ext_reactive=bold_style
                />
                <BasicButton
                    icon=Some(IconId::BsTypeItalic)
                    icon_before=true
                    onclick=italic
                    style_ext_reactive=italic_style
                />
                <BasicButton
                    icon=Some(IconId::BsTypeUnderline)
                    icon_before=true
                    onclick=underline
                    style_ext_reactive=underline_style
                />
                <BasicButton
                    icon=Some(IconId::BiStrikethroughRegular)
                    icon_before=true
                    onclick=strikethrough
                    style_ext_reactive=strikethrough_style
                />
                {
                    extra_formating_options.contains(&ExtraFormatingOption::ImageUpload).then(|| view!{
                        <BasicButton icon=Some(IconId::BsImage) icon_before=true onclick=insert_image style_ext="hover:bg-light-gray" />
                    })
                }
                {
                    extra_formating_options.contains(&ExtraFormatingOption::InlineCode).then(|| view!{
                        <BasicButton
                            icon=Some(IconId::BsCode)
                            onclick=inline_code
                            style_ext_reactive=inline_code_style
                        />
                    })
                }
                {
                    extra_formating_options.contains(&ExtraFormatingOption::CodeBlock).then(|| view!{
                        <BasicButton
                            icon=Some(IconId::BsBraces)
                            onclick=code_block
                            style_ext_reactive=code_block_style
                        />

                        <Show when=move || show_language_picker.get()>
                            <div class="ml-2">
                                <SelectInput id_attr="code-language" options=language_options onchange=apply_language />
                            </div>
                        </Show>
                    })
                }
                {
                    extra_formating_options.contains(&ExtraFormatingOption::MarkdownUpload).then(|| view!{
                        <BasicButton
                            icon=Some(IconId::VsMarkdown)
                            icon_before=true
                            onclick=upload_md
                            style_ext="hover:bg-light-gray"
                        />
                    })
                }


                {
                    extra_formating_options.contains(&ExtraFormatingOption::Lists).then(|| view!{
                        <BasicButton
                            icon=Some(IconId::BsListOl)
                            icon_before=true
                            onclick=ordered_list
                            style_ext_reactive=ordered_list_style
                        />
                        <BasicButton
                            icon=Some(IconId::BsListUl)
                            icon_before=true
                            onclick=unordered_list
                            style_ext_reactive=unordered_list_style
                        />
                    })
                }
            </div>

            // Editor
            <div
                node_ref=editor_ref
                contenteditable="true"
                on:keydown=on_keydown
                on:click=on_selection_change
                on:keyup=on_keyup
                class="min-h-[200px] max-h-[45svh] overflow-y-auto p-3 prose focus:rounded-b-none outline-secondary"
                inner_html=move || initial_content.get()
                on:input=handle_on_input
            />
            <InputField field_type=InputFieldType::File input_node_ref=file_input_ref accept="image/*" onchange=on_file_change ext_input_styles="hidden" id_attr=format!("{}-file-input", id_attr) />


            <InputField field_type=InputFieldType::Text id_attr=format!("{}-text-input", id_attr) ext_input_styles="hidden" initial_value=tracked_content name=name />

            <InputField
                field_type=InputFieldType::File
                input_node_ref=md_file_input_ref
                accept=".md,.markdown"
                onchange=on_md_file_change
                ext_input_styles="hidden"
                id_attr=format!("{}-md-file-input", id_attr)
            />
        </div>
    }
}

fn cursor_inside(tag: &str) -> Option<Element> {
    let doc = window()?.document()?;
    let selection = doc.get_selection().ok()??;
    let range = selection.get_range_at(0).ok()?;

    let container = range.start_container().ok()?;

    let mut node = if let Some(text) = container.dyn_ref::<web_sys::Text>() {
        text.parent_element()?
    } else if let Some(el) = container.dyn_ref::<Element>() {
        el.clone()
    } else {
        return None;
    };

    loop {
        if node.tag_name().eq_ignore_ascii_case(tag) {
            return Some(node);
        }
        match node.parent_element() {
            Some(p) => node = p,
            None => break,
        }
    }

    None
}

fn exit_code_block(pre: &Element) {
    let doc = window().unwrap().document().unwrap();
    let sel = doc.get_selection().unwrap().unwrap();

    let p = doc.create_element("p").unwrap();
    let br = doc.create_element("br").unwrap();
    p.append_child(&br).ok();

    pre.after_with_node_1(&p).ok();

    let new_range = doc.create_range().unwrap();
    new_range.set_start(&p, 0).ok();
    new_range.set_end(&p, 0).ok();

    sel.remove_all_ranges().ok();
    sel.add_range(&new_range).ok();
}

fn insert_clean_line(editor: &HtmlDivElement) {
    let doc = window().and_then(|w| w.document()).unwrap();
    let selection = doc.get_selection().unwrap().unwrap();
    let range = selection.get_range_at(0).unwrap();

    // Create a clean block
    let block = doc.create_element("p").unwrap();
    let br = doc.create_element("br").unwrap();
    block.append_child(&br).ok();

    // Find the current block: the closest p ancestor that is a direct child of editor
    let mut node = range.start_container().unwrap();
    let editor_as_node: &web_sys::Node = editor.as_ref();
    loop {
        if let Some(el) = node.dyn_ref::<web_sys::Element>() {
            if el.tag_name().eq_ignore_ascii_case("p") {
                if let Some(parent) = el.parent_element() {
                    if parent.is_same_node(Some(editor_as_node)) {
                        // Found current block: el
                        // Insert new block after el
                        let next_sibling = el.next_sibling();
                        match next_sibling {
                            Some(sib) => {
                                editor.insert_before(&block, Some(&sib)).ok();
                            }
                            None => {
                                editor.append_child(&block).ok();
                            }
                        }
                        break;
                    }
                }
            }
        }
        match node.parent_node() {
            Some(parent) => node = parent,
            None => {
                // Not found, append to editor
                editor.append_child(&block).ok();
                break;
            }
        }
    }

    // Move cursor inside the new block
    let new_range = doc.create_range().unwrap();
    new_range.set_start(&block, 0).ok();
    new_range.set_end(&block, 0).ok();

    selection.remove_all_ranges().ok();
    selection.add_range(&new_range).ok();
}

fn insert_code_block() {
    if let Some(doc) = window().and_then(|w| w.document()) {
        if let Ok(Some(selection)) = doc.get_selection() {
            if let Ok(range) = selection.get_range_at(0) {
                let pre = doc.create_element("pre").unwrap();
                pre.set_attribute("data-block", "code").ok();

                let code = doc.create_element("code").unwrap();
                code.set_attribute("class", "language-plaintext").ok();

                let text = doc.create_text_node("\n");
                code.append_child(&text).ok();
                pre.append_child(&code).ok();

                range.delete_contents().ok();
                range.insert_node(&pre).ok();

                // Move cursor inside code
                let new_range = doc.create_range().unwrap();
                new_range.set_start(&text, 1).ok();
                new_range.set_end(&text, 1).ok();

                selection.remove_all_ranges().ok();
                selection.add_range(&new_range).ok();
            }
        }
    }
}

fn current_code_block() -> Option<(web_sys::Element, web_sys::Element)> {
    let doc = window()?.document()?;
    let sel = doc.get_selection().ok()??;
    let range = sel.get_range_at(0).ok()?;

    let mut node = range.start_container().ok()?;

    loop {
        if let Some(code) = node.dyn_ref::<Element>() {
            if code.tag_name().eq_ignore_ascii_case("code") {
                if let Some(pre) = code.parent_element() {
                    if pre.tag_name().eq_ignore_ascii_case("pre") {
                        return Some((pre, code.clone()));
                    }
                }
            }
        }
        node = node.parent_node()?;
    }
}

fn is_current_line_empty(code: &Element) -> bool {
    let doc = window().unwrap().document().unwrap();
    let sel = doc.get_selection().unwrap().unwrap();
    let range = match sel.get_range_at(0) {
        Ok(r) => r,
        Err(_) => return false,
    };

    let text = code.text_content().unwrap_or_default();

    let offset_u32 = match range.start_offset() {
        Ok(o) => o,
        Err(_) => return false,
    };

    let offset = offset_u32 as usize;
    let offset = offset.min(text.len());

    let before = &text[..offset];
    let line_start = before.rfind('\n').map(|i| i + 1).unwrap_or(0);
    let line = &before[line_start..];

    line.trim().is_empty()
}

fn handle_code_enter(pre: &Element, code: &Element, last_enter_empty: &RwSignal<bool>) {
    let doc = window().unwrap().document().unwrap();
    let sel = doc.get_selection().unwrap().unwrap();
    let range = sel.get_range_at(0).unwrap();

    let empty = is_current_line_empty(code);

    if empty && last_enter_empty.get_untracked() {
        // EXIT CODE BLOCK
        last_enter_empty.set(false);

        let p = doc.create_element("p").unwrap();
        let br = doc.create_element("br").unwrap();
        p.append_child(&br).ok();

        pre.after_with_node_1(&p).ok();
        pre.remove();

        let new_range = doc.create_range().unwrap();
        new_range.set_start(&p, 0).ok();
        new_range.set_end(&p, 0).ok();

        sel.remove_all_ranges().ok();
        sel.add_range(&new_range).ok();
        return;
    }

    // NORMAL ENTER INSIDE CODE
    last_enter_empty.set(empty);

    let text = doc.create_text_node("\n");
    range.insert_node(&text).ok();

    let new_range = doc.create_range().unwrap();
    new_range.set_start_after(&text).ok();
    new_range.set_end_after(&text).ok();

    sel.remove_all_ranges().ok();
    sel.add_range(&new_range).ok();
}

fn insert_list(editor: &HtmlDivElement, list_type: &str) {
    let doc = window().and_then(|w| w.document()).unwrap();
    let selection = doc.get_selection().unwrap().unwrap();
    let range = selection.get_range_at(0).unwrap();

    // Create the list structure
    let list = doc.create_element(list_type).unwrap();

    // Check if there's selected content
    let has_selection = !range.collapsed();

    if has_selection {
        // Get the selected content
        if let Ok(contents) = range.clone_contents() {
            // Use first_child() and next_sibling() to iterate through children
            let mut has_blocks = false;
            let mut current_child = contents.first_child();

            // First pass: check if we have block elements
            while let Some(node) = current_child {
                if let Some(el) = node.dyn_ref::<Element>() {
                    let tag = el.tag_name().to_lowercase();
                    if ["p", "h1", "h2", "h3", "h4", "h5", "h6", "div"].contains(&tag.as_str()) {
                        has_blocks = true;
                        break;
                    }
                }
                current_child = node.next_sibling();
            }

            if has_blocks {
                // Convert each block element to a list item
                current_child = contents.first_child();

                while let Some(node) = current_child {
                    let next = node.next_sibling();

                    if let Some(el) = node.dyn_ref::<Element>() {
                        let tag = el.tag_name().to_lowercase();
                        if ["p", "h1", "h2", "h3", "h4", "h5", "h6", "div"].contains(&tag.as_str())
                        {
                            let li = doc.create_element("li").unwrap();

                            // Move all children of the block into the li
                            while let Some(child) = el.first_child() {
                                li.append_child(&child).ok();
                            }

                            // If li is empty, add a br
                            if li.first_child().is_none() {
                                let br = doc.create_element("br").unwrap();
                                li.append_child(&br).ok();
                            }

                            list.append_child(&li).ok();
                        }
                    } else if node.node_type() == 3 {
                        // Text node - only add if it's not just whitespace
                        if let Some(text) = node.text_content() {
                            if !text.trim().is_empty() {
                                let li = doc.create_element("li").unwrap();
                                li.append_child(&node.clone_node().unwrap()).ok();
                                list.append_child(&li).ok();
                            }
                        }
                    }

                    current_child = next;
                }
            } else {
                // No blocks, just wrap everything in a single li
                let li = doc.create_element("li").unwrap();
                li.append_child(&contents).ok();
                list.append_child(&li).ok();
            }
        }
    } else {
        // No selection, just add an empty li
        let li = doc.create_element("li").unwrap();
        let br = doc.create_element("br").unwrap();
        li.append_child(&br).ok();
        list.append_child(&li).ok();
    }

    // Find the current block element
    let mut node = range.start_container().unwrap();
    let editor_as_node: &web_sys::Node = editor.as_ref();

    loop {
        if let Some(el) = node.dyn_ref::<web_sys::Element>() {
            if ["p", "h1", "h2", "h3", "h4", "h5", "h6"]
                .contains(&el.tag_name().to_lowercase().as_str())
            {
                if let Some(parent) = el.parent_element() {
                    if parent.is_same_node(Some(editor_as_node)) {
                        // Replace the current block with the list
                        parent.replace_child(&list, el).ok();

                        // Move cursor to the first li
                        if let Some(first_li) = list.first_element_child() {
                            let new_range = doc.create_range().unwrap();
                            new_range.select_node_contents(&first_li).ok();
                            new_range.collapse_with_to_start(false);
                            selection.remove_all_ranges().ok();
                            selection.add_range(&new_range).ok();
                        }
                        return;
                    }
                }
            }
        }
        match node.parent_node() {
            Some(parent) => node = parent,
            None => break,
        }
    }

    // Fallback: append to editor if no block found
    editor.append_child(&list).ok();
    if let Some(first_li) = list.first_element_child() {
        let new_range = doc.create_range().unwrap();
        new_range.select_node_contents(&first_li).ok();
        new_range.collapse_with_to_start(false);
        selection.remove_all_ranges().ok();
        selection.add_range(&new_range).ok();
    }
}

fn current_list_item() -> Option<(web_sys::Element, web_sys::Element)> {
    let doc = window()?.document()?;
    let sel = doc.get_selection().ok()??;
    let range = sel.get_range_at(0).ok()?;

    let mut node = range.start_container().ok()?;

    loop {
        if let Some(li) = node.dyn_ref::<Element>() {
            if li.tag_name().eq_ignore_ascii_case("li") {
                if let Some(list) = li.parent_element() {
                    let tag = list.tag_name().to_lowercase();
                    if tag == "ol" || tag == "ul" {
                        return Some((list, li.clone()));
                    }
                }
            }
        }
        node = node.parent_node()?;
    }
}

fn handle_list_enter(list: &Element, li: &Element) {
    let doc = window().unwrap().document().unwrap();
    let sel = doc.get_selection().unwrap().unwrap();

    // Check if current li is empty
    let is_empty = li.text_content().unwrap_or_default().trim().is_empty();

    if is_empty {
        // Exit the list - create a new paragraph after the list
        let p = doc.create_element("p").unwrap();
        let br = doc.create_element("br").unwrap();
        p.append_child(&br).ok();

        list.after_with_node_1(&p).ok();

        // Remove the empty li
        li.remove();

        // If the list is now empty, remove it too
        if list.children().length() == 0 {
            list.remove();
        }

        // Move cursor to the new paragraph
        let new_range = doc.create_range().unwrap();
        new_range.set_start(&p, 0).ok();
        new_range.set_end(&p, 0).ok();
        sel.remove_all_ranges().ok();
        sel.add_range(&new_range).ok();
    } else {
        // Create a new list item
        let new_li = doc.create_element("li").unwrap();
        let br = doc.create_element("br").unwrap();
        new_li.append_child(&br).ok();

        // Insert after current li
        if let Some(next_sibling) = li.next_sibling() {
            list.insert_before(&new_li, Some(&next_sibling)).ok();
        } else {
            list.append_child(&new_li).ok();
        }

        // Move cursor to the new li
        let new_range = doc.create_range().unwrap();
        new_range.set_start(&new_li, 0).ok();
        new_range.set_end(&new_li, 0).ok();
        sel.remove_all_ranges().ok();
        sel.add_range(&new_range).ok();
    }
}

fn exit_list(editor: &HtmlDivElement, list: &Element) {
    let doc = window().unwrap().document().unwrap();
    let sel = doc.get_selection().unwrap().unwrap();

    // Create a new paragraph after the list
    let p = doc.create_element("p").unwrap();
    let br = doc.create_element("br").unwrap();
    p.append_child(&br).ok();

    // Insert the paragraph after the list (don't replace it)
    list.after_with_node_1(&p).ok();

    // Move cursor to the new paragraph
    let new_range = doc.create_range().unwrap();
    new_range.set_start(&p, 0).ok();
    new_range.set_end(&p, 0).ok();
    sel.remove_all_ranges().ok();
    sel.add_range(&new_range).ok();
}
