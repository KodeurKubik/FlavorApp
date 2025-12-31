(function () {
  if (window.__FLAVORAPP__) return;
  window.__FLAVORAPP__ = {};

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", init);
  } else {
    init();
  }
})();

function get(name) {
  return window.__FLAVORAPP__[name];
}
function set(name, value) {
  window.__FLAVORAPP__[name] = value;
}
function interval(name, func, every, run_now = false) {
  let int = get("INTERVAL_" + name);
  if (int) clearInterval(int);

  let newInt = setInterval(func, every);
  set("INTERVAL_" + name, newInt);

  if (run_now) func();
}

const BACKEND = "https://flavortown.hackclub.com";
async function api(endpoint, method = "GET") {
  return await (
    await fetch(BACKEND + endpoint, {
      method,
      headers: {
        Authorization: "Bearer " + get("API_KEY"),
        "X-Flavortown-Ext-2793": true,
      },
    })
  ).json();
}

async function notify(title, description) {
  await window.__TAURI__.core.invoke("notify", { title, description });
}

function makeSettings() {
  const settings = document.getElementById("settings-modal");
  const firstItem = settings.firstChild;

  // title
  const title2 = document.createElement("h3");
  title2.innerHTML =
    '<h3 class="modal__title" style="margin-bottom: var(--space-l); border-radius: calc(var(--border-radius)*.6);">FlavorApp</h3>';
  settings.insertBefore(title2, firstItem);

  // API key text
  document.querySelector(
    "#settings-modal > div.api-key-section > small"
  ).innerHTML += `<br />This key is used by FlavorApp if the settings are turned on.`;

  function getFormHTML(id, title, description, checked) {
    return `<div class="settings-form__field">
        <label class="settings-form__checkbox">
            <input type="checkbox" name="${id}" id="${id}" value=${
      checked ? "1" : "0"
    } ${
      checked ? 'checked="checked"' : ""
    } onchange="localStorage.setItem('${id}', this.checked)">
            <span>${title}</span>
        </label>
        <small class="settings-form__hint">${description}</small>
      </div>`;
  }

  // notification when can buy
  {
    let flavorapp_settings_canbuy = localStorage.getItem(
      "flavorapp_settings_canbuy"
    );
    if (!flavorapp_settings_canbuy) {
      localStorage.setItem("flavorapp_settings_canbuy", true);
      flavorapp_settings_canbuy = true;
    }

    const canBuy = document.createElement("div");
    canBuy.innerHTML = getFormHTML(
      "flavorapp_settings_canbuy",
      "Notify when ready to order",
      "Receive a notification when you reach one of your cookies goal from the shop",
      flavorapp_settings_canbuy
    );
    settings.insertBefore(canBuy, firstItem);

    interval(
      "canbuy",
      () => {
        if (localStorage.getItem("flavorapp_settings_canbuy") != "true") return;
        const wishlist = JSON.parse(
          localStorage.getItem("shop_wishlist") || "{}"
        );
        if (!wishlist) return;

        const COOKIE_COUNT = get("USER").cookies;
        let NOTIFIED = JSON.parse(
          localStorage.getItem("shop_wishlist_notified") || "[]"
        );

        // clean up NOTIFIED list
        NOTIFIED = NOTIFIED.filter((n) => wishlist[n]);

        let ready = Object.values(wishlist).filter(
          (el) => el.price <= COOKIE_COUNT && !NOTIFIED.includes(el.id)
        );

        ready.forEach((yay) => {
          notify(
            `${yay.name} available!`,
            `You can now purchase ${yay.name} for 🍪 ${yay.price} in the shop!`
          );
          NOTIFIED.push(yay.id);
        });

        localStorage.setItem(
          "shop_wishlist_notified",
          JSON.stringify(NOTIFIED)
        );
      },
      3 * 60 * 1000
    );
  }
}

async function init() {
  console.log("FlavorApp script injected!");
  window.__TAURI__.log.attachConsole();

  if (!document.body.classList.contains("signed-in"))
    return console.log("Not running FlavorApp because not logged in");

  // CONSTANTS
  const API_KEY = document
    .querySelector("#settings-modal > div > div > div")
    .innerHTML.trim();
  set("API_KEY", API_KEY);

  const USERID = document
    .querySelector(
      "body > aside > div.sidebar__user > div > div.sidebar__user-details > a"
    )
    .href.split("users/")[1];
  set("USERID", USERID);

  if (!document.getElementById("flavorapp")) {
    // inject element
    const id = document.createElement("div");
    id.id = "flavorapp";
    document.body.appendChild(id);

    makeSettings();

    // Update user info once in 3 minutes
    interval(
      "userinfo",
      async () => {
        set("USER", await api(`/api/v1/users/${get("USERID")}`));
      },
      3 * 60 * 1000,
      true
    );
  } else {
    console.error(
      "Element #flavorapp present, please reload the page if FlavorApp isn't working as expected."
    );
  }
}
