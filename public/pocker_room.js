const table_id = new URLSearchParams(window.location.search).get("table_id");

const table_section = document.getElementById("table-sec");
const hand_section = document.getElementById("hand-sec");
const user_section = document.getElementById("user-sec");
const players_section = document.getElementById("players-sec");
var user_id = ""

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

function prepare_card(rank, suit) {
  var map = {};
  map["Two"] = "2";
  map["Three"] = "3";
  map["Four"] = "4";
  map["Five"] = "5";
  map["Six"] = "6";
  map["Seven"] = "7";
  map["Eight"] = "8";
  map["Nine"] = "9";
  map["Ten"] = "10";
  map["Jack"] = "J";
  map["Queen"] = "Q";
  map["King"] = "K";
  map["Ace"] = "A";

  map["Clubs"] = "♣";
  map["Spades"] = "♠";
  map["Diamonds"] = "♦";
  map["Hearts"] = "♥";

  if (suit == "Diamonds" | "Hearts")
    return prepare_a_card(map[rank], map[suit], "red")
  else
    return prepare_a_card(map[rank], map[suit], "black")
}

document.getElementById("check").addEventListener("click", () => {
  sendMessage("ready")
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
  console.log("pps:" + splt[1]);

  if (splt[0] == "players") {
    let players = JSON.parse(splt[1]);
    let aaa = players.filter((p) => p.id == user_id)[0].hand.map((p) => prepare_card(p.rank, p.suit)).join("");
    console.log("xxx:  " + players[0].hand.join(","));
    console.log("aaa:" + aaa);
    players_section.innerHTML = players.map((a) => a.id + '<br/>------------------<br/>').join("");
    hand_section.innerHTML = aaa;
  } else if (splt[0] == "user_id") {
    user_section.innerHTML = splt[1]
    user_id = splt[1]
  } else if (splt[0] == "game") {
    user_section.innerHTML = splt[1]
  } else {
    console.log("unknown command!" + cmd)
  }
}

