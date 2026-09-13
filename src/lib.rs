use wasm_bindgen::prelude::*;

fn create_style(document: &web_sys::Document, id: &str) {
    let style = document.create_element("style").unwrap();

    style.set_id(id);

    document.head().unwrap().append_child(&style).unwrap();
}

fn set_style(document: &web_sys::Document, id: &str, css: &str) {
    document
        .get_element_by_id(id)
        .unwrap()
        .set_text_content(Some(css));
}

#[wasm_bindgen]
pub fn init_style_sheets() {
    //console::log_1("setting style sheet...");
    let document = web_sys::window().unwrap().document().unwrap();

    for id in [
        "seterra-no-label-style",
        "seterra-dark-mode-style",
        "seterra-sidebar-style",
        "seterra-mappadding-style",
        "seterra-water-style",
        "seterra-header-style",
    ] {
        create_style(&document, id);
    }
}

#[wasm_bindgen]
pub fn set_label(enabled: bool) {
    let document = web_sys::window().unwrap().document().unwrap();

    set_style(
        &document,
        "seterra-no-label-style",
        if enabled {
            ".game-area_tooltip__Ns9Yi { display: flex !important; }"
        } else {
            ".game-area_tooltip__Ns9Yi { display: none !important; }"
        },
    )
}

#[wasm_bindgen]
pub fn set_darkmode(enabled: bool) {
    let document = web_sys::window().unwrap().document().unwrap();

    set_style(
        &document,
        "seterra-dark-mode-style",
        if enabled {
            ".seterra_content__nGh5_ { background: #181A1B !important; color: white !important; }
			.seterra_main__mwfLw { background: #181A1B !important; color: white !important; }
			.button_button__aR6_e { color: white !important; }
			.highscore_table__oKrYg { color: white !important; background-color: #000000; }
			.game-container_backgroundCard__k_11p { background: none !important; }
			.score-modal_scoreModal__j5FYk { background: none !important; color: white !important; }
			.button_button__HOmVR { color: white !important; }"
        } else {
            ".seterra_content__nGh5_ { background: #ffffff !important; color: black !important; }
            .seterra_main__mwfLw { background: #ffffff !important; color: black !important; }
			.button_button__aR6_e { color: black !important; }
			.highscore_table__oKrYg { color: black !important; background-color: #FFFFFF; }
			.game-container_backgroundCard__k_11p { background: #FFFFFF; }
			.score-modal_scoreModal__j5FYk { background: white !important; color: black !important; }
			.button_button__HOmVR { color: black !important; }"
        },
    )
}

#[wasm_bindgen]
pub fn set_center_game(enabled: bool) {
    let document = web_sys::window().unwrap().document().unwrap();

    set_style(
        &document,
        "seterra-sidebar-style",
        if enabled {
            ".seterra_sidebar__p6xf1 { display: none !important; }"
        } else {
            ".seterra_sidebar__p6xf1 { display: flex !important; }"
        },
    )
}

#[wasm_bindgen]
pub fn set_dim_screen(enabled: bool) {
    let document = web_sys::window().unwrap().document().unwrap();

    let element_domtokenlist = document
        .get_element_by_id("dim-overlay")
        .unwrap()
        .class_list();

    if enabled {
        let _ = element_domtokenlist.toggle_with_force("visible", true);
    } else {
        let _ = element_domtokenlist.toggle_with_force("visible", false);
    }
}

#[wasm_bindgen]
pub fn set_type_easy_text_on_top(enabled: bool) {
    let document = web_sys::window().unwrap().document().unwrap();

    let typeeasy_text = document
        .query_selector("[data-qa='area-list']")
        .unwrap()
        .unwrap();

    let parent = typeeasy_text.parent_element().unwrap();

    if enabled {
        parent.prepend_with_node_1(&typeeasy_text).unwrap(); //prepend 1 node at the start.
    } else {
        let second = parent.children().item(2).unwrap();

        let _ = parent.insert_before(&typeeasy_text, Some(&second));
    }
}

#[wasm_bindgen]
pub fn set_map_padding(enabled: bool) {
    let document = web_sys::window().unwrap().document().unwrap();

    set_style(
        &document,
        "seterra-mappadding-style",
        if enabled {
            ".game-area_gameArea__G2ABs { margin-bottom: 500px !important; }"
        } else {
            ".game-area_gameArea__G2ABs { margin-bottom: 0px !important; }"
        },
    )
}

#[wasm_bindgen]
pub fn set_water(enabled: bool) {
    let document = web_sys::window().unwrap().document().unwrap();

    set_style(
        &document,
        "seterra-water-style",
        if enabled {
            "#WATER { display: block !important; }
			#BACKGROUND { display: block !important; }
			#WATER_1_ { display: block !important; }
			#WATER_2_ { display: block !important; }
			#WATER_3_ { display: block !important; }"
        } else {
            "#WATER { display: none !important; }
			#BACKGROUND { display: none !important; }
			#WATER_1_ { display: none !important; }
			#WATER_2_ { display: none !important; }
			#WATER_3_ { display: none !important; }"
        },
    )
}

fn create_span(document: &web_sys::Document, parent: &web_sys::Element, id: &str) {
    let span = document.create_element("span").unwrap();
    span.set_id(id);

    let _ = parent.append_child(&span);
}

/*
#[wasm_bindgen]
pub fn create_old_header() {
    let document = web_sys::window().unwrap().document().unwrap();

    let gg_header = document.query_selector("[data-qa='game-map-header']").unwrap().unwrap();

        let old_header = document.create_element("dir").unwrap();
        old_header.set_id("old-header");
        /*
        let score_span = document.create_element("span").unwrap();
        score_span.set_id("old-header-score");
        score_span.set_text_content(Some("score-span"));

        old_header.append_child(&score_span);

        let score_span = document.create_element("span").unwrap();
        score_span.set_id("old-header-score2");
        score_span.set_text_content(Some("|score-span2"));

        old_header.append_child(&score_span);
        */
        for spanid in [
            "old-count",
            "old-score",
            "old-timer",
            "old-current-question",
        ] {
            create_span(&document, &old_header, spanid);
        }

        let flag_img = document.create_element("img").unwrap();
        flag_img.set_id("old-flag");
        old_header.append_child(&flag_img);

        let header_location = document.query_selector(".game-area_gameArea__G2ABs").unwrap().unwrap();
        let second = header_location.children().item(0).unwrap();
        let _ = header_location.insert_before(&old_header, Some(&second) );
}
*/

#[wasm_bindgen]
pub fn set_old_header(enabled: bool) {
    let document = web_sys::window().unwrap().document().unwrap();

    if !document.get_element_by_id("old-header").is_some() {
        //let gg_header = document.query_selector("[data-qa='game-map-header']").unwrap().unwrap();

        let old_header = document.create_element("dir").unwrap();
        old_header.set_id("old-header");
        /*
        let score_span = document.create_element("span").unwrap();
        score_span.set_id("old-header-score");
        score_span.set_text_content(Some("score-span"));

        old_header.append_child(&score_span);

        let score_span = document.create_element("span").unwrap();
        score_span.set_id("old-header-score2");
        score_span.set_text_content(Some("|score-span2"));

        old_header.append_child(&score_span);
        */
        for spanid in [
            "old-count",
            "old-score",
            "old-timer",
            "old-current-question",
        ] {
            create_span(&document, &old_header, spanid);
        }

        let flag_img = document.create_element("img").unwrap();
        flag_img.set_id("old-flag");
        let _ = old_header.append_child(&flag_img);

        let header_location = document
            .query_selector(".game-area_gameArea__G2ABs")
            .unwrap()
            .unwrap();
        let second = header_location.children().item(0).unwrap();
        let _ = header_location.insert_before(&old_header, Some(&second));
    }

    /*
    if !document.get_element_by_id("old-header").is_some() {
        create_old_header()
    }*/
    set_style(
        &document,
        "seterra-header-style",
        if enabled {
            ".game-header_root__EyjJc { display: none !important; }
			#old-header { display: block !important }"
        } else {
            ".game-header_root__EyjJc { display: grid !important; }
			#old-header { display: none !important }"
        },
    )
}
/*
        const score = newHeader.querySelector(".game-header_left__Psq9Q label:nth-of-type(2)");
        const timer = newHeader.querySelector(".game-header_stopwatchTime__jJKdx h4");
        const question = newHeader.getAttribute("data-current-question-text");
        const flag = newHeader.querySelector(".game-header_topRightImage__N1wKi");

        if (score)
            document.getElementById("old-score").textContent = score.textContent;

        if (timer)
            document.getElementById("old-timer").textContent = " | " + timer.textContent;

        if (question)
            document.getElementById("old-current-question").textContent = " | Click on " + question;
*/
#[wasm_bindgen]
pub fn update_old_header() {
    let document = web_sys::window().unwrap().document().unwrap();

    let new_header = document
        .query_selector("[data-qa='game-map-header']")
        .unwrap()
        .unwrap();

    let new_count = new_header
        .query_selector(".game-header_left__Psq9Q label:nth-of-type(1)")
        .unwrap()
        .unwrap();
    let new_score = new_header
        .query_selector(".game-header_left__Psq9Q label:nth-of-type(2)")
        .unwrap()
        .unwrap();
    let new_timer = new_header
        .query_selector(".game-header_stopwatchTime__jJKdx h4")
        .unwrap()
        .unwrap();
    let new_question = new_header
        .get_attribute("data-current-question-text")
        .unwrap();
    let new_flag = new_header
        .query_selector(".game-header_topRightImage__N1wKi")
        .unwrap();

    let old_score = document.get_element_by_id("old-score").unwrap();
    let old_count = document.get_element_by_id("old-count").unwrap();
    let old_timer = document.get_element_by_id("old-timer").unwrap();
    let old_question = document.get_element_by_id("old-current-question").unwrap();
    let old_flag = document.get_element_by_id("old-flag").unwrap();

    old_score.set_text_content(Some(&new_score.text_content().unwrap()));
    old_count.set_text_content(Some(&format!("{} | ", &new_count.text_content().unwrap())));
    old_timer.set_text_content(Some(&format!(" | {}", new_timer.text_content().unwrap())));
    old_question.set_text_content(Some(&format!(" | Click on {}", new_question)));

    if let Some(new_flag) = new_flag {
        if let Some(src) = new_flag.get_attribute("src") {
            old_flag.set_attribute("src", &src).unwrap();
        }
    } else {
        old_flag.remove_attribute("src").unwrap();
    }
}

#[wasm_bindgen]
pub fn set_old_header_flag(enabled: bool) {
    let document = web_sys::window().unwrap().document().unwrap();
    let old_flag = document.get_element_by_id("old-flag").unwrap();

    if enabled {
        let _ = old_flag.class_list().toggle_with_force("disabled", false);
    } else {
        let _ = old_flag.class_list().toggle_with_force("disabled", true);
    }
}

#[wasm_bindgen]
pub fn set_url_parameter(parameter: &str, to_enable: bool) {
    let window = web_sys::window().unwrap();
    let url = window.location();

    //web_sys::console::log_1(&url.into());

    let og_query_string = url.search().unwrap();
    //example: vgp/3737?fastclick=1&nocursor=1, returns ?fastclick=1&nocursor=1 as a String of application/x-www-form-urlencoded format

    let parameters = web_sys::UrlSearchParams::new_with_str(&og_query_string).unwrap();

    if to_enable {
        parameters.set(parameter, "1")
    } else {
        parameters.delete(parameter)
    }

    let new_query_string = parameters.to_string();

    let path = url.pathname().unwrap();

    let new_url = if new_query_string.length() < 1 {
        path
    } else {
        format!("{}?{}", path, new_query_string)
    };

    window
        .history()
        .unwrap()
        .replace_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&new_url))
        .unwrap();
}
