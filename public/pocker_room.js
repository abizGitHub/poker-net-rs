const table_id = new URLSearchParams(window.location.search).get("table_id");

const table_section = document.getElementById("table-sec");
const hand_section = document.getElementById("hand-sec");
const user_section = document.getElementById("user-sec");
const players_section = document.getElementById("players-sec");

table_section.innerHTML = [1, 2, 3, 4, 5].map((a) => prepare_a_card("", "", "white")).join("");
hand_section.innerHTML = [1, 2].map((a) => prepare_a_card("", "", "white")).join("");


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

document.getElementById("check").addEventListener("click", () => {
  sendMessage(89)
});


document.getElementById("fold").addEventListener("click", () => {


});

function init_table() {

  socket.addEventListener('message', (event) => {
    dispatch_event(event.data);
  });

  sendMessage("add_player_to_table::" + table_id);
}

init_table()

function dispatch_event(cmd) {
  let splt = cmd.split('::')
  if (splt[0] == "players") {
    players_section.innerHTML = splt[1].split(',').map((a) => a + '<br/>------------------<br/>').join("")
  } else if (splt[0] == "user_id") {
    user_section.innerHTML = splt[1]
  } else {
    console.log("unknown command!" + cmd)
  }
}

