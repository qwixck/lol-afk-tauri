const { invoke } = window.__TAURI__.tauri

function champ(name) {
  invoke("write", { name: name }).then(result => {
      const champion = document.getElementById(name)
      if (result) champion.style.backgroundColor = "#0f0f0f98"
      else champion.style.backgroundColor = "green"
  }).catch(e => {console.warn(e)})
}

function changeSetting(setting) {
  switch (setting) {
    case "top": {
      invoke("get_setting", { setting: "position" }).then(result => {
        const button = document.getElementById(result)
        button.style.backgroundColor = "#2f2f2f"
        button.removeAttribute("disabled")
      }).catch(e => console.error(e))
      invoke("change_setting", { key: "position", value: "top"})
      const button = document.getElementById("top")
      button.style.backgroundColor = "green"
      button.disabled = true
      break;
    }
    case "jungle": {
      invoke("get_setting", { setting: "position" }).then(result => {
        const button = document.getElementById(result)
        button.style.backgroundColor = "#2f2f2f"
        button.removeAttribute("disabled")
      }).catch(e => console.error(e))
      invoke("change_setting", { key: "position", value: "jungle"})
      const button = document.getElementById("jungle")
      button.style.backgroundColor = "green"
      button.disabled = true
      break;
    }
    case "middle": {
      invoke("get_setting", { setting: "position" }).then(result => {
        const button = document.getElementById(result)
        button.style.backgroundColor = "#2f2f2f"
        button.removeAttribute("disabled")
      }).catch(e => console.error(e))
      invoke("change_setting", { key: "position", value: "middle"})
      const button = document.getElementById("middle")
      button.style.backgroundColor = "green"
      button.disabled = true
      break;
    }
    case "bottom": {
      invoke("get_setting", { setting: "position" }).then(result => {
        const button = document.getElementById(result)
        button.style.backgroundColor = "#2f2f2f"
        button.removeAttribute("disabled")
      }).catch(e => console.error(e))
      invoke("change_setting", { key: "position", value: "bottom"})
      const button = document.getElementById("bottom")
      button.style.backgroundColor = "green"
      button.disabled = true
      break;
    }
    case "utility": {
      invoke("get_setting", { setting: "position" }).then(result => {
        const button = document.getElementById(result)
        button.style.backgroundColor = "#2f2f2f"
        button.removeAttribute("disabled")
      }).catch(e => console.error(e))
      invoke("change_setting", { key: "position", value: "utility"})
      const button = document.getElementById("utility")
      button.style.backgroundColor = "green"
      button.disabled = true
      break;
    }
    case "pick": {
      invoke("get_setting", { setting: "type" }).then(result => {
        const button = document.getElementById(result)
        button.style.backgroundColor = "#2f2f2f"
        button.removeAttribute("disabled")
      }).catch(e => console.error(e))
      invoke("change_setting", { key: "type", value: "pick"})
      const button = document.getElementById("pick")
      button.style.backgroundColor = "green"
      button.disabled = true
      break;
    }
    case "ban": {
      invoke("get_setting", { setting: "type" }).then(result => {
        const button = document.getElementById(result)
        button.style.backgroundColor = "#2f2f2f"
        button.removeAttribute("disabled")
      }).catch(e => console.error(e))
      invoke("change_setting", { key: "type", value: "ban"})
      const button = document.getElementById("ban")
      button.style.backgroundColor = "green"
      button.disabled = true
      break;
    }
    case "drafts": {
      invoke("get_setting", { setting: "mode" }).then(result => {
        const button = document.getElementById(result)
        button.style.backgroundColor = "#2f2f2f"
        button.removeAttribute("disabled")
      }).catch(e => console.error(e))
      invoke("change_setting", { key: "mode", value: "drafts"})
      const button = document.getElementById("drafts")
      button.style.backgroundColor = "green"
      button.disabled = true
      break;
    }
    case "blind": {
      invoke("get_setting", { setting: "mode" }).then(result => {
        const button = document.getElementById(result)
        button.style.backgroundColor = "#2f2f2f"
        button.removeAttribute("disabled")
      }).catch(e => console.error(e))
      invoke("change_setting", { key: "mode", value: "blind"})
      const button = document.getElementById("blind")
      button.style.backgroundColor = "green"
      button.disabled = true
      break;
    }
  }
}

function generateChamps(champion, champions) {
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

document.addEventListener("DOMContentLoaded", () => {
  const buttons = document.getElementById("champions").children

  for (let i = 0; i < buttons.length; i++) {
    invoke("read", { name: buttons[i].id }).then(result => {
      if (result) {
        buttons[i].style.backgroundColor = "green";
      }
    }).catch(e => {console.warn(e)})
  }

  const _setting = ["position", "type", "mode"]
  for (let i = 0; i < _setting.length; i++) {
    invoke("get_setting", { setting: _setting[i] }).then(result => {
      const button = document.getElementById(result)
      button.style.backgroundColor = "green"
      button.disabled = true
    })
  }

  invoke("get_champions").then(_champions => {
    const champions = JSON.parse(_champions).data;
    const input = document.getElementById("input");

    input.oninput = () => {
      if (input.value !== "") {
        for (let champion in champions) {
          try {
            document.getElementById(champion).remove()
          } catch (e) {}
        }

        for (let champion in champions) {
          if (champion.includes(input.value.toLowerCase())) {
            generateChamps(champion, champions)
          }
        }
      } else {
        for (let champion in champions) {
          try {
            document.getElementById(champion).remove()
          } catch (e) {}
        }
        
        for (let champion in champions) {
          generateChamps(champion, champions)
        }
      }
    }
  })
})