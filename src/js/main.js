const { invoke } = window.__TAURI__.tauri

function champ(name) {
  const newName = name.charAt(0).toUpperCase() + name.slice(1)
  invoke("write", { name: newName }).then(result => {
      const champion = document.getElementById(name.toLowerCase())
      if (result) champion.style.backgroundColor = "#0f0f0f98"
      else champion.style.backgroundColor = "green"
  }).catch(e => {console.warn(e)})
}

document.addEventListener("DOMContentLoaded", () => {
  const buttons = document.getElementById("champions").children

  for (let i = 0; i < buttons.length; i++) {
    const newName = buttons[i].id.charAt(0).toUpperCase() + buttons[i].id.slice(1)
    invoke("read", { name: newName }).then(result => {
      if (result) {
        buttons[i].style.backgroundColor = "green";
      }
    }).catch(e => {console.warn(e)})
  }

  invoke("get_champions").then(_champions => {
    const champions = JSON.parse(_champions).data;
    const input = document.getElementById("input");

    input.oninput = () => {
      if (input.value !== "") {
        for (let champion in champions) {
          try {
            document.getElementById(champion.toLowerCase()).remove()
          } catch (e) {}
        }

        for (let champion in champions) {
          if (champion.toLowerCase().includes(input.value.toLowerCase())) {
            const element = document.createElement("button")
            element.addEventListener("click", () => champ(champions[champion]["id"].toLowerCase()))
            element.id = champion.toLowerCase()
            invoke("read", { name: champions[champion]["id"] }).then(result => {
              if (result) {
                element.style.backgroundColor = "green"
              }
            }).catch(e => {console.warn(e)})
            const elementP = document.createElement("p")
            elementP.textContent = champion
            const elementA = document.createElement("a")
            const elementImg = document.createElement("img")
            elementImg.src = `../assets/icons/icon${champions[champion]["key"]}.png`
            elementImg.alt = champions[champion]["key"]
            elementA.appendChild(elementImg)
            element.appendChild(elementA)
            element.appendChild(elementP)
            document.getElementById("champions").appendChild(element)
          }
        }
      } else {
        for (let champion in champions) {
          try {
            document.getElementById(champion.toLowerCase()).remove()
          } catch (e) {}
        }
        
        for (let champion in champions) {
          if (champion.toLowerCase().includes(input.value.toLowerCase())) {
            const element = document.createElement("button")
            element.addEventListener("click", () => champ(champions[champion]["id"].toLowerCase()))
            element.id = champion.toLowerCase()
            invoke("read", { name: champions[champion]["id"] }).then(result => {
              if (result) {
                element.style.backgroundColor = "green"
              }
            }).catch(e => {console.warn(e)})
            const elementP = document.createElement("p")
            elementP.textContent = champion
            const elementA = document.createElement("a")
            const elementImg = document.createElement("img")
            elementImg.src = `../assets/icons/icon${champions[champion]["key"]}.png`
            elementImg.alt = champions[champion]["key"]
            elementA.appendChild(elementImg)
            element.appendChild(elementA)
            element.appendChild(elementP)
            document.getElementById("champions").appendChild(element)
          }
        }
      }
    }
  })
})