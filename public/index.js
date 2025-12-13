let socket = null;
const WS_URL = 'ws://127.0.0.1:9001'; // Example URL

function produce_join_link(table_id) {
  const link = `pocker_room.html?table_id=${table_id}`;
  const container = document.getElementById("linkContainer");
  container.innerHTML = `<br/> link to join : <a href="${link}">${link}</a>`;
}

function init() {

  connect();
  
  document.getElementById("btn").addEventListener("click", () => {
    socket.addEventListener('message', (event) => {
      produce_join_link(event.data.split("::")[1]);
    });
    sendMessage("set_a_table");
  });

}

function logMessage(msg, isOwn = false) {
  console.log(msg + ' << ' + isOwn)
}

// Handle sending messages
function sendMessage(msg) {
  if (msg && socket && socket.readyState === WebSocket.OPEN) {
    socket.send(msg);
    logMessage(msg, true);
  } else {
    console.log("cant send!")
  }
}

// Handle connection
function connect() {
  if (socket && socket.readyState === WebSocket.OPEN) {
    return;
  }

  try {
    socket = new WebSocket(WS_URL);

    // Connection opened
    socket.addEventListener('open', (event) => {
      logMessage('Connected to server');
    });

    // Message received
    socket.addEventListener('message', (event) => {
      logMessage(event.data);
    });

    // Connection closed
    socket.addEventListener('close', (event) => {
      logMessage('Disconnected from server');
    });

    // Connection error
    socket.addEventListener('error', (event) => {
      logMessage('Connection error!');
    });

  } catch (error) {
    logMessage('Failed to create WebSocket: ' + error.message);
  }
}

function disconnect() {
  if (socket) {
    socket.close();
  }
}

init();
