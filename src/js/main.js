const { invoke } = window.__TAURI__.tauri

function champ(name) {
  invoke("write", { name: name }).then(result => {
      const champion = document.getElementById(name)
      if (result) champion.style.backgroundColor = "#0f0f0f98"
      else champion.style.backgroundColor = "green"
  }).catch(e => {console.warn(e)})
}

function modifySetting(key, value) {
  invoke("get_setting", { setting: key }).then(result => {
    const button = document.getElementById(result)
    button.style.backgroundColor = "#0f0f0f98"
    button.removeAttribute("disabled")
  }).catch(e => console.error(e))
  invoke("change_setting", { key: key , value: value})
  const button = document.getElementById(value)
  button.style.backgroundColor = "green"
  button.disabled = true
}

function generateChamp(champion, champions) {
  const element = document.createElement("button")
  element.addEventListener("click", () => champ(champion))
  element.id = champion
  invoke("read", { name: champion }).then(result => result ? element.style.backgroundColor = "green" : {})
    .catch(e => {console.warn(e)})
  const elementP = document.createElement("p")
  elementP.textContent = champion.charAt(0).toUpperCase() + champion.slice(1)
  const elementA = document.createElement("a")
  const elementImg = document.createElement("img")
  elementImg.src = `../assets/icons/icon${champions[champion]["key"]}.png`
  elementImg.alt = champion
  elementA.appendChild(elementImg)
  element.appendChild(elementA)
  element.appendChild(elementP)
  document.getElementById("champions").appendChild(element)
}

function changeSetting(setting) {
  switch (setting) {
    case "top": {
      modifySetting("position", "top")
      break
    }
    case "jungle": {
      modifySetting("position", "jungle")
      break
    }
    case "middle": {
      modifySetting("position", "middle")
      break
    }
    case "bottom": {
      modifySetting("position", "bottom")
      break
    }
    case "utility": {
      modifySetting("position", "utility")
      break
    }
    case "pick": {
      modifySetting("type_", "pick")
      break
    }
    case "ban": {
      modifySetting("type_", "ban")
      break
    }
    case "drafts": {
      modifySetting("mode", "drafts")
      const button2 = document.getElementById("ban")
      button2.style.backgroundColor = "#0f0f0f98"
      button2.disabled = false
      const buttons = document.getElementById("positions").children
      for (let i = 0; i < buttons.length; i++) {
        buttons[i].style.backgroundColor = "#0f0f0f98"
        buttons[i].disabled = false
      }
      invoke("get_setting", { setting: "position" }).then(result => {
        document.getElementById(result).style.backgroundColor = "green"
      }).catch(e => console.error(e))
      invoke("get_setting", { setting: "type_" }).then(result => {
        document.getElementById(result).style.backgroundColor = "green"
      }).catch(e => console.error(e))
      break;
    }
    case "blind": {
      modifySetting("mode", "blind")
      invoke("change_setting", { key: "position", value: "middle"})
      const button2 = document.getElementById("ban")
      button2.style.backgroundColor = "red"
      button2.disabled = true
      const buttons = document.getElementById("positions").children
      for (let i = 0; i < buttons.length; i++) {
        buttons[i].style.backgroundColor = "red"
        buttons[i].disabled = true
      }
      invoke("change_setting", { key: "type_", value: "pick" })
      invoke("get_setting", { setting: "type_" }).then(result => {
        document.getElementById(result).style.backgroundColor = "green"
      }).catch(e => console.error(e))
      break;
    }
  }

  const buttons = document.getElementById("champions").children
  for (let i = 0; i < buttons.length; i++) {
    invoke("read", { name: buttons[i].id }).then(result => {
      if (result) {
        buttons[i].style.backgroundColor = "green";
      } else {
        buttons[i].style.backgroundColor = "#0f0f0f98";
      }
    }).catch(e => {console.warn(e)})
  }
}

document.addEventListener("DOMContentLoaded", () => {
  const buttons = document.getElementById("champions").children
  const _setting = ["position", "type_", "mode"]

  for (let i = 0; i < _setting.length; i++) {
    invoke("get_setting", { setting: _setting[i] }).then(result => {
      const button = document.getElementById(result)
      button.style.backgroundColor = "green"
      button.disabled = true
    })
  }

  invoke("get_setting", { setting: "mode" }).then(result => {
    if (result === "blind") {
      const positions = document.getElementById("positions").children
      for (let i = 0; i < positions.length; i++) {
        positions[i].style.backgroundColor = "red"
        positions[i].disabled = true
      }
      const ban = document.getElementById("ban")
      ban.style.backgroundColor = "red"
      ban.disabled = true
    }
  })

  for (let i = 0; i < buttons.length; i++) {
    invoke("read", { name: buttons[i].id }).then(result => {
      if (result) {
        buttons[i].style.backgroundColor = "green";
      }
    }).catch(e => {console.warn(e)})
  }

  invoke("get_champions").then(_champions => {
    const champions = JSON.parse(_champions).data;
    const input = document.getElementById("input");

    input.oninput = () => {
      const championsContainer = document.getElementById("champions");
      championsContainer.innerHTML = "";
    
      if (input.value !== "") {
        for (let champion in champions) {
          if (champion.includes(input.value.toLowerCase())) {
            generateChamp(champion, champions);
          }
        }
      } else {
        for (let champion in champions) {
          generateChamp(champion, champions);
        }
      }
    };
  })

  const connect = document.getElementById("connect")
  connect.onclick = () => {
    conn = invoke("connect")
    connect.style.backgroundColor = "green"
    connect.disabled = true
    conn.catch((error) => {
      connect.style.backgroundColor = "#0f0f0f98"
      connect.disabled = false
      console.error(error)
    })
  }
})