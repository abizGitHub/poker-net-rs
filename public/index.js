

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

document.getElementById("btn").addEventListener("click", () => {
  let table_id = rustApp.set_a_table();

  //const num = Math.floor(Math.random() * 1_000_000_000);
  const link = `pocker_room.html?table_id=${table_id}`;
  const container = document.getElementById("linkContainer");
  container.innerHTML = `<br/> link to join : <a href="${link}">${location.origin}/${table_id}</a>`;
});
