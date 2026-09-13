async function toggleButton(button) {
    button.enabled = !button.enabled;

    button.element.classList.toggle("on", button.enabled);

    button.activate();

    await chrome.storage.local.set({
        [button.text]: button.enabled
    });
}

(async () => {
    const wasm = await import(chrome.runtime.getURL("pkg/seterra_wasm.js"));

    await wasm.default();

    wasm.init_style_sheets();
    
    const response = await fetch(chrome.runtime.getURL("menu.html"));
    const html = await response.text();

    document.body.insertAdjacentHTML("beforeend", html);
    
    console.log("seterra-extension loading..");
    
    const is_old_ver = location.hostname === "web.archive.org";
    
    const buttons = [
		{
			text: "Dark Mode",
			category: "Website",
			enabled: true,
			element: null,
			keybind: null,
			activate() {
				wasm.set_darkmode(this.enabled);
			}
		},
		{
			text: "Center Map",
			category: "Website",
			enabled: true,
			element: null,
			keybind: null,
			activate() {
				wasm.set_center_game(this.enabled);
			}
		},
		{
			text: "Map Padding",
			category: "Website",
			enabled: false,
			element: null,
			keybind: null,
			activate() {
				wasm.set_map_padding(this.enabled);
			}
		},
		{
			text: "Dim Screen",
			category: "Website",
			enabled: false,
			element: null,
			keybind: null,
			activate() {
				wasm.set_dim_screen(this.enabled);
			}
		},
		{
			text: "Label",
			category: "Gameplay",
			enabled: false,
			element: null,
			keybind: null,
			activate() {
				wasm.set_label(this.enabled);
			}
		},
		{
			text: "Old Header",
			category: "Gameplay",
			enabled: false,
			element: null,
			keybind: null,
			activate() {
				wasm.set_old_header(this.enabled);
			}
		},
		{
			text: "Old Header Flag",
			category: "Gameplay",
			enabled: false,
			element: null,
			keybind: null,
			activate() {
				wasm.set_old_header_flag(this.enabled);
			}
		},
		{
			text: "Type Easy Text on Top",
			category: "Gameplay",
			enabled: false,
			element: null,
			keybind: null,
			activate() {
				wasm.set_type_easy_text_on_top(this.enabled);
			}
		},
		{
			text: "Water",
			category: "Gameplay",
			enabled: true,
			element: null,
			keybind: null,
			activate() {
				wasm.set_water(this.enabled);
			}
		},
		{
			text: "Fast Click",
			category: "Oldver",
			enabled: false,
			element: null,
			keybind: null,
			activate() {
				wasm.set_url_parameter("fastclick", this.enabled);
			}
		},
		{
			text: "No Flags",
			category: "Oldver",
			enabled: false,
			element: null,
			keybind: null,
			activate() {
				wasm.set_url_parameter("noflags", this.enabled);
			}
		},
		{
			text: "No Timer",
			category: "Oldver",
			enabled: false,
			element: null,
			keybind: null,
			activate() {
				wasm.set_url_parameter("notimer", this.enabled);
			}
		},
		{
			text: "No Cursor (label)",
			category: "Oldver",
			enabled: false,
			element: null,
			keybind: null,
			activate() {
				wasm.set_url_parameter("nocursor", this.enabled);
			}
		},
		{
			text: "Show Next",
			category: "Oldver",
			enabled: false,
			element: null,
			keybind: null,
			activate() {
				wasm.set_url_parameter("shownext", this.enabled);
			}
		},
		{
			text: "Hover Click",
			category: "Oldver",
			enabled: false,
			element: null,
			keybind: null,
			activate() {
				wasm.set_url_parameter("hoverclick", this.enabled);
			}
		}
	];
	
	const saved = await chrome.storage.local.get(
        buttons.map(button => button.text)
    );
    
	const menu = document.getElementById("seterra-menu");
	
	for (const button of buttons) {
		if (saved[button.text] !== undefined) {
            button.enabled = saved[button.text];
        }
        
		const element = document.createElement("button");
		button.element = element;
		
		element.textContent = button.text;
		element.className = "toggle-button";
		
		if (button.enabled) {
			element.classList.add("on");
		}
		
		let should_activate = true;
		
		if (is_old_ver) {
			should_activate = button.category === "Oldver";
		} else {
			should_activate = button.category !== "Oldver";
		}
		
		if (should_activate) {
			element.addEventListener("click", async () => {
				toggleButton(button);
			});
			
			button.activate();
		}
		
		const category = document.getElementById(
			"category-" + button.category
		);

		category.appendChild(element);
	}
	
	document.addEventListener("keydown", async (event) => {
		if (event.key === "Delete" || event.key === "Home") {
			document.getElementById("seterra-menu").classList.toggle('open');
		}
		if (event.key === "Escape") {
			document.querySelector(".game-header_quitGameButton__zTYUz").click();
		}
	});
	
	if (!is_old_ver) {
		const gg_header = document.querySelector("[data-qa='game-map-header']");
	
		const observer = new MutationObserver(wasm.update_old_header);

		observer.observe(gg_header, {
			childList: true,
			subtree: true,
			characterData: true,
			attributes: true
		});
	}
	console.log("seterra-extension loaded");
	
	//update checker
	document.getElementById("extension-version").textContent = chrome.runtime.getManifest().version;

    let button =  document.getElementById("check-updates-button");
	button.addEventListener("click", async () => {
		//Check for updates
		//const button = this;
		button.textContent = 'Checking...';
		
		const current_version = chrome.runtime.getManifest().version;
		
		try {
			const response = await fetch(
				'https://api.github.com/repos/Afdusrt/seterra-qol-extension/releases/latest'
			);
			if (!response.ok) {
				throw new Error('Fetching error');
			}
			const release = await response.json();
			const latest_version = release.tag_name;
			const download_url = release.assets[0].browser_download_url;
			
			if (current_version === latest_version) {
				button.textContent = 'No updates';
			} else {
				button.textContent = 'Update found! Click to download.';
				button.href = download_url;
				button.classList.add("clickable-url");
			}
			
		} catch {
			console.error(error);
			button.textContent = 'Update check failed';
		}
		
		//button.textContent = 'Check for updates';
	});
})();
