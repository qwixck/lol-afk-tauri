const { invoke } = window.__TAURI__.tauri

function champ(name) {
  invoke("write", { name: name }).then(result => {
      const champion = document.getElementById(name)
      if (result) champion.style.backgroundColor = "#0f0f0f98"
      else champion.style.backgroundColor = "green"
  }).catch(e => {console.warn(e)})
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