
async function init() {

    let rustApp = null
    try {
        rustApp = await import('../pkg')
    } catch (e) {
        console.log(e)
        return;
    }
    console.log(rustApp)

    const logBox = document.getElementById("log");
    const canvas = document.getElementById("canvas");
    rustApp.set_canvas(canvas);
    
    
    // List of events you want to capture
    const eventsToLog = [
        "click", //"mousemove",
        "keydown", "keyup", "scroll", "input",
        "mousedown", "mouseup", "mouseenter", "mouseleave", "touchstart",
        "touchend"
    ];

    function log(event) {
        let resp = rustApp.on_call(event.type);        
        if (resp != undefined) {
            const msg = `${new Date().toLocaleTimeString()} → ${resp.map(a => a + "<br>")}`;
            logBox.innerHTML += msg + "<br>";
            logBox.scrollTop = logBox.scrollHeight;
        }
        console.log(event.type)
    }

function prepare_a_card(rank, suit, color) {
    let dev =
        `<div class="card" tabindex="0" role="group" aria-label="Ten of hearts">
       <div class="corner top-left red">10♥</div>
       <div class="suit-big red" aria-hidden="true">♥</div>
       <div class="corner bottom-right red">10♥</div>
    </div>`

    return dev
}

init()