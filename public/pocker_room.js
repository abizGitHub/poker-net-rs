const table_id = new URLSearchParams(window.location.search).get("table_id");

const table_section = document.getElementById("table-sec");
const hand_section = document.getElementById("hand-sec");
const user_section = document.getElementById("user-sec");

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
  console.log(table_id)
  socket.addEventListener('message', (event) => {
    console.log(event.data);
  });
  sendMessage("add_player_to_table::" + table_id);
});


document.getElementById("fold").addEventListener("click", () => {
  console.log(table_id)
  socket.addEventListener('message', (event) => {
    user_section.innerHTML = event.data.split(',').map((a) => a + '<br/>').join("")
  });
  sendMessage("get_table_players::" + table_id);
});
