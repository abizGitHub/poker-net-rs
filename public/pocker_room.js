
let rustApp = null;

async function init() {
  try {
    rustApp = await import('../pkg');
  } catch (e) {
    console.log(e)
    return;
  }
  alert(rustApp);
}

init()

const params = new URLSearchParams(window.location.search);
const table_id = params.get("table_id");
const table_section = document.getElementById("table-sec");

table_section.innerHTML = [1, 2, 3, 4, 5].map((a) => prepare_a_card("", "", "white")).join("");

alert(rustApp);

document.getElementById("check").addEventListener("click", () => {
    
});




function prepare_a_card(rank, suit, color) {
    let card_class = "card"
    if (color == "white") {
        card_class = "mock"
    }

    let dev =
        `<div class="${card_class}" tabindex="0" role="group">
       <div class="corner top-left ${color}">${rank}${suit}</div>
       <div class="suit-big ${color}" aria-hidden="true">${suit}</div>
       <div class="corner bottom-right ${color}">${rank}${suit}</div>
    </div>`

    return dev
}